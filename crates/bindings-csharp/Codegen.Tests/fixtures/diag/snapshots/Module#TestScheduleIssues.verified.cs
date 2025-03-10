﻿//HintName: TestScheduleIssues.cs
// <auto-generated />
#nullable enable

partial struct TestScheduleIssues
    : System.IEquatable<TestScheduleIssues>,
        SpacetimeDB.BSATN.IStructuralReadWrite
{
    public void ReadFields(System.IO.BinaryReader reader)
    {
        IdWrongType = BSATN.IdWrongType.Read(reader);
        IdCorrectType = BSATN.IdCorrectType.Read(reader);
        ScheduleAtWrongType = BSATN.ScheduleAtWrongType.Read(reader);
        ScheduleAtCorrectType = BSATN.ScheduleAtCorrectType.Read(reader);
    }

    public void WriteFields(System.IO.BinaryWriter writer)
    {
        BSATN.IdWrongType.Write(writer, IdWrongType);
        BSATN.IdCorrectType.Write(writer, IdCorrectType);
        BSATN.ScheduleAtWrongType.Write(writer, ScheduleAtWrongType);
        BSATN.ScheduleAtCorrectType.Write(writer, ScheduleAtCorrectType);
    }

    public readonly partial struct BSATN : SpacetimeDB.BSATN.IReadWrite<TestScheduleIssues>
    {
        internal static readonly SpacetimeDB.BSATN.String IdWrongType = new();
        internal static readonly SpacetimeDB.BSATN.I32 IdCorrectType = new();
        internal static readonly SpacetimeDB.BSATN.I32 ScheduleAtWrongType = new();
        internal static readonly SpacetimeDB.ScheduleAt.BSATN ScheduleAtCorrectType = new();

        public TestScheduleIssues Read(System.IO.BinaryReader reader) =>
            SpacetimeDB.BSATN.IStructuralReadWrite.Read<TestScheduleIssues>(reader);

        public void Write(System.IO.BinaryWriter writer, TestScheduleIssues value)
        {
            value.WriteFields(writer);
        }

        public SpacetimeDB.BSATN.AlgebraicType.Ref GetAlgebraicType(
            SpacetimeDB.BSATN.ITypeRegistrar registrar
        ) =>
            registrar.RegisterType<TestScheduleIssues>(
                _ => new SpacetimeDB.BSATN.AlgebraicType.Product(
                    new SpacetimeDB.BSATN.AggregateElement[]
                    {
                        new(nameof(IdWrongType), IdWrongType.GetAlgebraicType(registrar)),
                        new(nameof(IdCorrectType), IdCorrectType.GetAlgebraicType(registrar)),
                        new(
                            nameof(ScheduleAtWrongType),
                            ScheduleAtWrongType.GetAlgebraicType(registrar)
                        ),
                        new(
                            nameof(ScheduleAtCorrectType),
                            ScheduleAtCorrectType.GetAlgebraicType(registrar)
                        )
                    }
                )
            );

        SpacetimeDB.BSATN.AlgebraicType SpacetimeDB.BSATN.IReadWrite<TestScheduleIssues>.GetAlgebraicType(
            SpacetimeDB.BSATN.ITypeRegistrar registrar
        ) => GetAlgebraicType(registrar);
    }

    public override int GetHashCode()
    {
        return IdWrongType.GetHashCode()
            ^ IdCorrectType.GetHashCode()
            ^ ScheduleAtWrongType.GetHashCode()
            ^ ScheduleAtCorrectType.GetHashCode();
    }

    public override string ToString()
    {
        return $"TestScheduleIssues(IdWrongType = {IdWrongType}, IdCorrectType = {IdCorrectType}, ScheduleAtWrongType = {ScheduleAtWrongType}, ScheduleAtCorrectType = {ScheduleAtCorrectType})";
    }

    public bool Equals(TestScheduleIssues that)
    {
        return IdWrongType.Equals(that.IdWrongType)
            && IdCorrectType.Equals(that.IdCorrectType)
            && ScheduleAtWrongType.Equals(that.ScheduleAtWrongType)
            && ScheduleAtCorrectType.Equals(that.ScheduleAtCorrectType);
    }

    public override bool Equals(object? that)
    {
        if (that == null)
        {
            return false;
        }
        var that_ = that as TestScheduleIssues?;
        if (((object?)that_) == null)
        {
            return false;
        }
        return Equals(that_);
    }

    public static bool operator ==(TestScheduleIssues this_, TestScheduleIssues that)
    {
        if (((object)this_) == null || ((object)that) == null)
        {
            return object.Equals(this_, that);
        }
        return this_.Equals(that);
    }

    public static bool operator !=(TestScheduleIssues this_, TestScheduleIssues that)
    {
        if (((object)this_) == null || ((object)that) == null)
        {
            return !object.Equals(this_, that);
        }
        return !this_.Equals(that);
    }
} // TestScheduleIssues
