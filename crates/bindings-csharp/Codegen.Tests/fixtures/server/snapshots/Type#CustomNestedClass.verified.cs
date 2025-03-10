﻿//HintName: CustomNestedClass.cs
// <auto-generated />
#nullable enable

partial class CustomNestedClass
    : System.IEquatable<CustomNestedClass>,
        SpacetimeDB.BSATN.IStructuralReadWrite
{
    public void ReadFields(System.IO.BinaryReader reader)
    {
        NestedClass = BSATN.NestedClass.Read(reader);
        NestedNullableClass = BSATN.NestedNullableClass.Read(reader);
        NestedEnum = BSATN.NestedEnum.Read(reader);
        NestedNullableEnum = BSATN.NestedNullableEnum.Read(reader);
        NestedTaggedEnum = BSATN.NestedTaggedEnum.Read(reader);
        NestedNullableTaggedEnum = BSATN.NestedNullableTaggedEnum.Read(reader);
        NestedCustomRecord = BSATN.NestedCustomRecord.Read(reader);
        NestedNullableCustomRecord = BSATN.NestedNullableCustomRecord.Read(reader);
    }

    public void WriteFields(System.IO.BinaryWriter writer)
    {
        BSATN.NestedClass.Write(writer, NestedClass);
        BSATN.NestedNullableClass.Write(writer, NestedNullableClass);
        BSATN.NestedEnum.Write(writer, NestedEnum);
        BSATN.NestedNullableEnum.Write(writer, NestedNullableEnum);
        BSATN.NestedTaggedEnum.Write(writer, NestedTaggedEnum);
        BSATN.NestedNullableTaggedEnum.Write(writer, NestedNullableTaggedEnum);
        BSATN.NestedCustomRecord.Write(writer, NestedCustomRecord);
        BSATN.NestedNullableCustomRecord.Write(writer, NestedNullableCustomRecord);
    }

    public readonly partial struct BSATN : SpacetimeDB.BSATN.IReadWrite<CustomNestedClass>
    {
        internal static readonly CustomClass.BSATN NestedClass = new();
        internal static readonly SpacetimeDB.BSATN.RefOption<
            CustomClass,
            CustomClass.BSATN
        > NestedNullableClass = new();
        internal static readonly SpacetimeDB.BSATN.Enum<CustomEnum> NestedEnum = new();
        internal static readonly SpacetimeDB.BSATN.ValueOption<
            CustomEnum,
            SpacetimeDB.BSATN.Enum<CustomEnum>
        > NestedNullableEnum = new();
        internal static readonly CustomTaggedEnum.BSATN NestedTaggedEnum = new();
        internal static readonly SpacetimeDB.BSATN.RefOption<
            CustomTaggedEnum,
            CustomTaggedEnum.BSATN
        > NestedNullableTaggedEnum = new();
        internal static readonly CustomRecord.BSATN NestedCustomRecord = new();
        internal static readonly SpacetimeDB.BSATN.RefOption<
            CustomRecord,
            CustomRecord.BSATN
        > NestedNullableCustomRecord = new();

        public CustomNestedClass Read(System.IO.BinaryReader reader) =>
            SpacetimeDB.BSATN.IStructuralReadWrite.Read<CustomNestedClass>(reader);

        public void Write(System.IO.BinaryWriter writer, CustomNestedClass value)
        {
            value.WriteFields(writer);
        }

        public SpacetimeDB.BSATN.AlgebraicType.Ref GetAlgebraicType(
            SpacetimeDB.BSATN.ITypeRegistrar registrar
        ) =>
            registrar.RegisterType<CustomNestedClass>(
                _ => new SpacetimeDB.BSATN.AlgebraicType.Product(
                    new SpacetimeDB.BSATN.AggregateElement[]
                    {
                        new(nameof(NestedClass), NestedClass.GetAlgebraicType(registrar)),
                        new(
                            nameof(NestedNullableClass),
                            NestedNullableClass.GetAlgebraicType(registrar)
                        ),
                        new(nameof(NestedEnum), NestedEnum.GetAlgebraicType(registrar)),
                        new(
                            nameof(NestedNullableEnum),
                            NestedNullableEnum.GetAlgebraicType(registrar)
                        ),
                        new(nameof(NestedTaggedEnum), NestedTaggedEnum.GetAlgebraicType(registrar)),
                        new(
                            nameof(NestedNullableTaggedEnum),
                            NestedNullableTaggedEnum.GetAlgebraicType(registrar)
                        ),
                        new(
                            nameof(NestedCustomRecord),
                            NestedCustomRecord.GetAlgebraicType(registrar)
                        ),
                        new(
                            nameof(NestedNullableCustomRecord),
                            NestedNullableCustomRecord.GetAlgebraicType(registrar)
                        )
                    }
                )
            );

        SpacetimeDB.BSATN.AlgebraicType SpacetimeDB.BSATN.IReadWrite<CustomNestedClass>.GetAlgebraicType(
            SpacetimeDB.BSATN.ITypeRegistrar registrar
        ) => GetAlgebraicType(registrar);
    }

    public override int GetHashCode()
    {
        return NestedClass.GetHashCode()
            ^ (NestedNullableClass == null ? 0 : NestedNullableClass.GetHashCode())
            ^ NestedEnum.GetHashCode()
            ^ NestedNullableEnum.GetHashCode()
            ^ NestedTaggedEnum.GetHashCode()
            ^ (NestedNullableTaggedEnum == null ? 0 : NestedNullableTaggedEnum.GetHashCode())
            ^ NestedCustomRecord.GetHashCode()
            ^ (NestedNullableCustomRecord == null ? 0 : NestedNullableCustomRecord.GetHashCode());
    }

    public override string ToString()
    {
        return $"CustomNestedClass(NestedClass = {SpacetimeDB.BSATN.StringUtil.GenericToString(NestedClass)}, NestedNullableClass = {SpacetimeDB.BSATN.StringUtil.GenericToString(NestedNullableClass)}, NestedEnum = {SpacetimeDB.BSATN.StringUtil.GenericToString(NestedEnum)}, NestedNullableEnum = {SpacetimeDB.BSATN.StringUtil.GenericToString(NestedNullableEnum)}, NestedTaggedEnum = {SpacetimeDB.BSATN.StringUtil.GenericToString(NestedTaggedEnum)}, NestedNullableTaggedEnum = {SpacetimeDB.BSATN.StringUtil.GenericToString(NestedNullableTaggedEnum)}, NestedCustomRecord = {SpacetimeDB.BSATN.StringUtil.GenericToString(NestedCustomRecord)}, NestedNullableCustomRecord = {SpacetimeDB.BSATN.StringUtil.GenericToString(NestedNullableCustomRecord)})";
    }

#nullable enable
    public bool Equals(CustomNestedClass? that)
    {
        if (((object?)that) == null)
        {
            return false;
        }
        return NestedClass.Equals(that.NestedClass)
            && (
                NestedNullableClass == null
                    ? that.NestedNullableClass == null
                    : NestedNullableClass.Equals(that.NestedNullableClass)
            )
            && NestedEnum.Equals(that.NestedEnum)
            && NestedNullableEnum.Equals(that.NestedNullableEnum)
            && NestedTaggedEnum.Equals(that.NestedTaggedEnum)
            && (
                NestedNullableTaggedEnum == null
                    ? that.NestedNullableTaggedEnum == null
                    : NestedNullableTaggedEnum.Equals(that.NestedNullableTaggedEnum)
            )
            && NestedCustomRecord.Equals(that.NestedCustomRecord)
            && (
                NestedNullableCustomRecord == null
                    ? that.NestedNullableCustomRecord == null
                    : NestedNullableCustomRecord.Equals(that.NestedNullableCustomRecord)
            );
    }

    public override bool Equals(object? that)
    {
        if (that == null)
        {
            return false;
        }
        var that_ = that as CustomNestedClass;
        if (((object?)that_) == null)
        {
            return false;
        }
        return Equals(that_);
    }

    public static bool operator ==(CustomNestedClass? this_, CustomNestedClass? that)
    {
        if (((object?)this_) == null || ((object?)that) == null)
        {
            return object.Equals(this_, that);
        }
        return this_.Equals(that);
    }

    public static bool operator !=(CustomNestedClass? this_, CustomNestedClass? that)
    {
        if (((object?)this_) == null || ((object?)that) == null)
        {
            return !object.Equals(this_, that);
        }
        return !this_.Equals(that);
    }
#nullable restore
} // CustomNestedClass
