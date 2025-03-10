﻿//HintName: TestIndexIssues.cs
// <auto-generated />
#nullable enable

partial struct TestIndexIssues
    : System.IEquatable<TestIndexIssues>,
        SpacetimeDB.BSATN.IStructuralReadWrite
{
    public void ReadFields(System.IO.BinaryReader reader)
    {
        SelfIndexingColumn = BSATN.SelfIndexingColumn.Read(reader);
    }

    public void WriteFields(System.IO.BinaryWriter writer)
    {
        BSATN.SelfIndexingColumn.Write(writer, SelfIndexingColumn);
    }

    public readonly partial struct BSATN : SpacetimeDB.BSATN.IReadWrite<TestIndexIssues>
    {
        internal static readonly SpacetimeDB.BSATN.I32 SelfIndexingColumn = new();

        public TestIndexIssues Read(System.IO.BinaryReader reader) =>
            SpacetimeDB.BSATN.IStructuralReadWrite.Read<TestIndexIssues>(reader);

        public void Write(System.IO.BinaryWriter writer, TestIndexIssues value)
        {
            value.WriteFields(writer);
        }

        public SpacetimeDB.BSATN.AlgebraicType.Ref GetAlgebraicType(
            SpacetimeDB.BSATN.ITypeRegistrar registrar
        ) =>
            registrar.RegisterType<TestIndexIssues>(
                _ => new SpacetimeDB.BSATN.AlgebraicType.Product(
                    new SpacetimeDB.BSATN.AggregateElement[]
                    {
                        new(
                            nameof(SelfIndexingColumn),
                            SelfIndexingColumn.GetAlgebraicType(registrar)
                        )
                    }
                )
            );

        SpacetimeDB.BSATN.AlgebraicType SpacetimeDB.BSATN.IReadWrite<TestIndexIssues>.GetAlgebraicType(
            SpacetimeDB.BSATN.ITypeRegistrar registrar
        ) => GetAlgebraicType(registrar);
    }

    public override int GetHashCode()
    {
        return SelfIndexingColumn.GetHashCode();
    }

    public override string ToString()
    {
        return $"TestIndexIssues(SelfIndexingColumn = {SelfIndexingColumn}";
    }

    public bool Equals(TestIndexIssues that)
    {
        return SelfIndexingColumn.Equals(that.SelfIndexingColumn);
    }

    public override bool Equals(object? that)
    {
        if (that == null)
        {
            return false;
        }
        var that_ = that as TestIndexIssues?;
        if (that_ == null)
        {
            return false;
        }
        return Equals(that);
    }

    public static bool operator ==(TestIndexIssues this_, TestIndexIssues that)
    {
        if (((object)this_) == null || ((object)that) == null)
        {
            return Object.Equals(this_, that);
        }
        return this_.Equals(that);
    }

    public static bool operator !=(TestIndexIssues this_, TestIndexIssues that)
    {
        return !(this_ == that);
    }
} // TestIndexIssues
