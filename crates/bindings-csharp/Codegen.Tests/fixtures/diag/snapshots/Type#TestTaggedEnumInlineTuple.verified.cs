﻿//HintName: TestTaggedEnumInlineTuple.cs
// <auto-generated />
#nullable enable

partial record TestTaggedEnumInlineTuple : System.IEquatable<TestTaggedEnumInlineTuple>
{
    private TestTaggedEnumInlineTuple() { }

    internal enum @enum : byte
    {
        Item1
    }

    public sealed record Item1(int Item1_) : TestTaggedEnumInlineTuple;

    public readonly partial struct BSATN : SpacetimeDB.BSATN.IReadWrite<TestTaggedEnumInlineTuple>
    {
        internal static readonly SpacetimeDB.BSATN.Enum<@enum> __enumTag = new();
        internal static readonly SpacetimeDB.BSATN.I32 Item1 = new();

        public TestTaggedEnumInlineTuple Read(System.IO.BinaryReader reader) =>
            __enumTag.Read(reader) switch
            {
                @enum.Item1 => new Item1(Item1.Read(reader)),
                _
                    => throw new System.InvalidOperationException(
                        "Invalid tag value, this state should be unreachable."
                    )
            };

        public void Write(System.IO.BinaryWriter writer, TestTaggedEnumInlineTuple value)
        {
            switch (value)
            {
                case Item1(var inner):
                    __enumTag.Write(writer, @enum.Item1);
                    Item1.Write(writer, inner);
                    break;
            }
        }

        public SpacetimeDB.BSATN.AlgebraicType.Ref GetAlgebraicType(
            SpacetimeDB.BSATN.ITypeRegistrar registrar
        ) =>
            registrar.RegisterType<TestTaggedEnumInlineTuple>(
                _ => new SpacetimeDB.BSATN.AlgebraicType.Sum(
                    new SpacetimeDB.BSATN.AggregateElement[]
                    {
                        new(nameof(Item1), Item1.GetAlgebraicType(registrar))
                    }
                )
            );

        SpacetimeDB.BSATN.AlgebraicType SpacetimeDB.BSATN.IReadWrite<TestTaggedEnumInlineTuple>.GetAlgebraicType(
            SpacetimeDB.BSATN.ITypeRegistrar registrar
        ) => GetAlgebraicType(registrar);
    }

    public override int GetHashCode()
    {
        switch (this)
        {
            case Item1(var inner):
                return inner.GetHashCode();
            default:
                return 0;
        }
    }

    public override string ToString()
    {
        switch (this)
        {
            case Item1(var inner):
                return $"Item1({inner})";
            default:
                return "UNKNOWN";
        }
    }
} // TestTaggedEnumInlineTuple
