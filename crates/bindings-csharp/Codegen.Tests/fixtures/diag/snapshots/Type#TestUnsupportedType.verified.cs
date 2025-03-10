﻿//HintName: TestUnsupportedType.cs
// <auto-generated />
#nullable enable

partial struct TestUnsupportedType
    : System.IEquatable<TestUnsupportedType>,
        SpacetimeDB.BSATN.IStructuralReadWrite
{
    public void ReadFields(System.IO.BinaryReader reader)
    {
        UnsupportedSpecialType = BSATN.UnsupportedSpecialType.Read(reader);
        UnsupportedSystemType = BSATN.UnsupportedSystemType.Read(reader);
        UnresolvedType = BSATN.UnresolvedType.Read(reader);
        UnsupportedEnum = BSATN.UnsupportedEnum.Read(reader);
    }

    public void WriteFields(System.IO.BinaryWriter writer)
    {
        BSATN.UnsupportedSpecialType.Write(writer, UnsupportedSpecialType);
        BSATN.UnsupportedSystemType.Write(writer, UnsupportedSystemType);
        BSATN.UnresolvedType.Write(writer, UnresolvedType);
        BSATN.UnsupportedEnum.Write(writer, UnsupportedEnum);
    }

    public readonly partial struct BSATN : SpacetimeDB.BSATN.IReadWrite<TestUnsupportedType>
    {
        internal static readonly SpacetimeDB.BSATN.Unsupported<System.DateTime> UnsupportedSpecialType =
            new();
        internal static readonly SpacetimeDB.BSATN.Unsupported<System.Exception> UnsupportedSystemType =
            new();
        internal static readonly SpacetimeDB.BSATN.Unsupported<object> UnresolvedType = new();
        internal static readonly SpacetimeDB.BSATN.Unsupported<LocalEnum> UnsupportedEnum = new();

        public TestUnsupportedType Read(System.IO.BinaryReader reader) =>
            SpacetimeDB.BSATN.IStructuralReadWrite.Read<TestUnsupportedType>(reader);

        public void Write(System.IO.BinaryWriter writer, TestUnsupportedType value)
        {
            value.WriteFields(writer);
        }

        public SpacetimeDB.BSATN.AlgebraicType.Ref GetAlgebraicType(
            SpacetimeDB.BSATN.ITypeRegistrar registrar
        ) =>
            registrar.RegisterType<TestUnsupportedType>(
                _ => new SpacetimeDB.BSATN.AlgebraicType.Product(
                    new SpacetimeDB.BSATN.AggregateElement[]
                    {
                        new(
                            nameof(UnsupportedSpecialType),
                            UnsupportedSpecialType.GetAlgebraicType(registrar)
                        ),
                        new(
                            nameof(UnsupportedSystemType),
                            UnsupportedSystemType.GetAlgebraicType(registrar)
                        ),
                        new(nameof(UnresolvedType), UnresolvedType.GetAlgebraicType(registrar)),
                        new(nameof(UnsupportedEnum), UnsupportedEnum.GetAlgebraicType(registrar))
                    }
                )
            );

        SpacetimeDB.BSATN.AlgebraicType SpacetimeDB.BSATN.IReadWrite<TestUnsupportedType>.GetAlgebraicType(
            SpacetimeDB.BSATN.ITypeRegistrar registrar
        ) => GetAlgebraicType(registrar);
    }

    public override int GetHashCode()
    {
        return UnsupportedSpecialType.GetHashCode()
            ^ UnsupportedSystemType.GetHashCode()
            ^ UnresolvedType.GetHashCode()
            ^ UnsupportedEnum.GetHashCode();
    }

    public override string ToString()
    {
        return $"TestUnsupportedType(UnsupportedSpecialType = {UnsupportedSpecialType}, UnsupportedSystemType = {UnsupportedSystemType}, UnresolvedType = {UnresolvedType}, UnsupportedEnum = {UnsupportedEnum})";
    }

#nullable enable
    public bool Equals(TestUnsupportedType that)
    {
        return UnsupportedSpecialType.Equals(that.UnsupportedSpecialType)
            && UnsupportedSystemType.Equals(that.UnsupportedSystemType)
            && UnresolvedType.Equals(that.UnresolvedType)
            && UnsupportedEnum.Equals(that.UnsupportedEnum);
    }

    public override bool Equals(object? that)
    {
        if (that == null)
        {
            return false;
        }
        var that_ = that as TestUnsupportedType?;
        if (((object?)that_) == null)
        {
            return false;
        }
        return Equals(that_);
    }

    public static bool operator ==(TestUnsupportedType this_, TestUnsupportedType that)
    {
        if (((object)this_) == null || ((object)that) == null)
        {
            return object.Equals(this_, that);
        }
        return this_.Equals(that);
    }

    public static bool operator !=(TestUnsupportedType this_, TestUnsupportedType that)
    {
        if (((object)this_) == null || ((object)that) == null)
        {
            return !object.Equals(this_, that);
        }
        return !this_.Equals(that);
    }
#nullable restore
} // TestUnsupportedType
