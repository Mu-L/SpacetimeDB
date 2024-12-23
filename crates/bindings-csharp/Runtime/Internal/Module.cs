namespace SpacetimeDB.Internal;

using System;
using System.Runtime.InteropServices;
using SpacetimeDB;
using SpacetimeDB.BSATN;

partial class RawModuleDefV9
{
    // Note: this intends to generate a valid identifier, but it's not guaranteed to be unique as it's not proper mangling.
    // Fix it up to a different mangling scheme if it causes problems.
    private static string GetFriendlyName(Type type) =>
        type.IsGenericType
            ? $"{type.Name.Remove(type.Name.IndexOf('`'))}_{string.Join("_", type.GetGenericArguments().Select(GetFriendlyName))}"
            : type.Name;

    private void RegisterTypeName<T>(AlgebraicType.Ref typeRef)
    {
        var scopedName = new RawScopedTypeNameV9([], GetFriendlyName(typeof(T)));
        Types.Add(new(scopedName, (uint)typeRef.Ref_, CustomOrdering: true));
    }

    internal AlgebraicType.Ref RegisterType<T>(Func<AlgebraicType.Ref, AlgebraicType> makeType)
    {
        var types = Typespace.Types;
        var typeRef = new AlgebraicType.Ref(types.Count);
        // Put a dummy self-reference just so that we get stable index even if `makeType` recursively adds more types.
        types.Add(typeRef);
        // Now we can safely call `makeType` and assign the result to the reserved slot.
        types[typeRef.Ref_] = makeType(typeRef);
        RegisterTypeName<T>(typeRef);
        return typeRef;
    }

    internal void RegisterReducer(RawReducerDefV9 reducer) => Reducers.Add(reducer);

    internal void RegisterTable(RawTableDefV9 table) => Tables.Add(table);
}

public static class Module
{
    private static readonly RawModuleDefV9 moduleDef = new();
    private static readonly List<IReducer> reducers = [];

    private static Func<Identity, Address?, Random, DateTimeOffset, IReducerContext>? newContext =
        null;

    public static void SetReducerContextConstructor(
        Func<Identity, Address?, Random, DateTimeOffset, IReducerContext> ctor
    ) => newContext = ctor;

    readonly struct TypeRegistrar() : ITypeRegistrar
    {
        private readonly Dictionary<Type, AlgebraicType.Ref> types = [];

        // Registers type in the module definition.
        //
        // To avoid issues with self-recursion during registration as well as unnecessary construction
        // of algebraic types for types that have already been registered, we accept a factory
        // returning an AlgebraicType instead of the AlgebraicType itself.
        //
        // The factory callback will be called with the allocated type reference that can be used for
        // e.g. self-recursion even before the algebraic type itself is constructed.
        public AlgebraicType.Ref RegisterType<T>(Func<AlgebraicType.Ref, AlgebraicType> makeType)
        {
            // Store for the closure access.
            var types = this.types;
            if (types.TryGetValue(typeof(T), out var existingTypeRef))
            {
                return existingTypeRef;
            }
            return moduleDef.RegisterType<T>(typeRef =>
            {
                // Store the type reference in the dictionary so that we can resolve it later and to avoid infinite recursion inside `makeType`.
                types.Add(typeof(T), typeRef);
                return makeType(typeRef);
            });
        }
    }

    static readonly TypeRegistrar typeRegistrar = new();

    public static void RegisterReducer<R>()
        where R : IReducer, new()
    {
        var reducer = new R();
        reducers.Add(reducer);
        moduleDef.RegisterReducer(reducer.MakeReducerDef(typeRegistrar));
    }

    public static void RegisterTable<T, View>()
        where T : IStructuralReadWrite, new()
        where View : ITableView<View, T>, new()
    {
        moduleDef.RegisterTable(View.MakeTableDesc(typeRegistrar));
    }

    private static MemoryStream Consume(this BytesSource source, ref byte[] buffer)
    {
        if (source == BytesSource.INVALID)
        {
            return new();
        }
        var written = 0U;
        while (true)
        {
            // Write into the spare capacity of the buffer.
            var spare = buffer.AsSpan((int)written);
            var buf_len = (uint)spare.Length;
            var ret = FFI.bytes_source_read(source, spare, ref buf_len);
            written += buf_len;
            switch (ret)
            {
                // Host side source exhausted, we're done.
                case Errno.EXHAUSTED:
                    return new(buffer, 0, (int)written);
                // Wrote the entire spare capacity.
                // Need to reserve more space in the buffer.
                case Errno.OK when written == buffer.Length:
                    Array.Resize(ref buffer, buffer.Length + 1024);
                    break;
                // Host didn't write as much as possible.
                // Try to read some more.
                // The host will likely not trigger this branch (current host doesn't),
                // but a module should be prepared for it.
                case Errno.OK:
                    ret.Check();
                    break;
            }
        }
    }

    private static void Write(this BytesSink sink, byte[] bytes)
    {
        var start = 0U;
        while (start != bytes.Length)
        {
            var written = (uint)bytes.Length;
            var buffer = bytes.AsSpan((int)start);
            FFI.bytes_sink_write(sink, buffer, ref written);
            start += written;
        }
    }

#pragma warning disable IDE1006 // Naming Styles - methods below are meant for FFI.

    public static void __describe_module__(BytesSink description)
    {
        try
        {
            // We need this explicit cast here to make `ToBytes` understand the types correctly.
            RawModuleDef versioned = new RawModuleDef.V9(moduleDef);
            var moduleBytes = IStructuralReadWrite.ToBytes(new RawModuleDef.BSATN(), versioned);
            description.Write(moduleBytes);
        }
        catch (Exception e)
        {
            Log.Error($"Error while describing the module: {e}");
        }
    }

    // Note: `__call_reducer__` can't be invoked in parallel because we don't support multithreading in Wasm,
    // nor is it supposed to be invoked recursively.
    //
    // This means we can reuse the same argument buffer for all `__call_reducer__` invocations -
    // unlike in e.g. iterators, where multiple iterators can easily exist at the same time.
    private static byte[] reducerArgsBuffer = new byte[0x10_000];

    public static Errno __call_reducer__(
        uint id,
        ulong sender_0,
        ulong sender_1,
        ulong sender_2,
        ulong sender_3,
        ulong address_0,
        ulong address_1,
        DateTimeOffsetRepr timestamp,
        BytesSource args,
        BytesSink error
    )
    {
        try
        {
            var senderIdentity = Identity.From(
                MemoryMarshal.AsBytes([sender_0, sender_1, sender_2, sender_3]).ToArray()
            );
            var senderAddress = Address.From(
                MemoryMarshal.AsBytes([address_0, address_1]).ToArray()
            );
            var random = new Random((int)timestamp.MicrosecondsSinceEpoch);
            var time = timestamp.ToStd();

            var ctx = newContext!(senderIdentity, senderAddress, random, time);

            using var stream = args.Consume(ref reducerArgsBuffer);
            using var reader = new BinaryReader(stream);
            reducers[(int)id].Invoke(reader, ctx);
            if (stream.Position != stream.Length)
            {
                throw new Exception("Unrecognised extra bytes in the reducer arguments");
            }
            return Errno.OK; /* no exception */
        }
        catch (Exception e)
        {
            var error_str = e.ToString();
            var error_bytes = System.Text.Encoding.UTF8.GetBytes(error_str);
            error.Write(error_bytes);
            return Errno.HOST_CALL_FAILURE;
        }
    }
}
