﻿//HintName: MultiTableRow.cs
// <auto-generated />
#nullable enable

partial struct MultiTableRow
    : System.IEquatable<MultiTableRow>,
        SpacetimeDB.BSATN.IStructuralReadWrite
{
    public void ReadFields(System.IO.BinaryReader reader)
    {
        Name = BSATN.Name.Read(reader);
        Foo = BSATN.Foo.Read(reader);
        Bar = BSATN.Bar.Read(reader);
    }

    public void WriteFields(System.IO.BinaryWriter writer)
    {
        BSATN.Name.Write(writer, Name);
        BSATN.Foo.Write(writer, Foo);
        BSATN.Bar.Write(writer, Bar);
    }

    public readonly partial struct BSATN : SpacetimeDB.BSATN.IReadWrite<MultiTableRow>
    {
        internal static readonly SpacetimeDB.BSATN.String Name = new();
        internal static readonly SpacetimeDB.BSATN.U32 Foo = new();
        internal static readonly SpacetimeDB.BSATN.U32 Bar = new();

        public MultiTableRow Read(System.IO.BinaryReader reader) =>
            SpacetimeDB.BSATN.IStructuralReadWrite.Read<MultiTableRow>(reader);

        public void Write(System.IO.BinaryWriter writer, MultiTableRow value)
        {
            value.WriteFields(writer);
        }

        public SpacetimeDB.BSATN.AlgebraicType.Ref GetAlgebraicType(
            SpacetimeDB.BSATN.ITypeRegistrar registrar
        ) =>
            registrar.RegisterType<MultiTableRow>(_ => new SpacetimeDB.BSATN.AlgebraicType.Product(
                new SpacetimeDB.BSATN.AggregateElement[]
                {
                    new(nameof(Name), Name.GetAlgebraicType(registrar)),
                    new(nameof(Foo), Foo.GetAlgebraicType(registrar)),
                    new(nameof(Bar), Bar.GetAlgebraicType(registrar))
                }
            ));

        SpacetimeDB.BSATN.AlgebraicType SpacetimeDB.BSATN.IReadWrite<MultiTableRow>.GetAlgebraicType(
            SpacetimeDB.BSATN.ITypeRegistrar registrar
        ) => GetAlgebraicType(registrar);
    }

    public override int GetHashCode()
    {
        return Name.GetHashCode() ^ Foo.GetHashCode() ^ Bar.GetHashCode();
    }

    public override string ToString()
    {
        return $"MultiTableRow(Name = {SpacetimeDB.BSATN.StringUtil.GenericToString(Name)}, Foo = {SpacetimeDB.BSATN.StringUtil.GenericToString(Foo)}, Bar = {SpacetimeDB.BSATN.StringUtil.GenericToString(Bar)})";
    }

#nullable enable
    public bool Equals(MultiTableRow that)
    {
        return Name.Equals(that.Name) && Foo.Equals(that.Foo) && Bar.Equals(that.Bar);
    }

    public override bool Equals(object? that)
    {
        if (that == null)
        {
            return false;
        }
        var that_ = that as MultiTableRow?;
        if (((object?)that_) == null)
        {
            return false;
        }
        return Equals(that_);
    }

    public static bool operator ==(MultiTableRow this_, MultiTableRow that)
    {
        if (((object?)this_) == null || ((object?)that) == null)
        {
            return object.Equals(this_, that);
        }
        return this_.Equals(that);
    }

    public static bool operator !=(MultiTableRow this_, MultiTableRow that)
    {
        if (((object?)this_) == null || ((object?)that) == null)
        {
            return !object.Equals(this_, that);
        }
        return !this_.Equals(that);
    }
#nullable restore
} // MultiTableRow
