﻿//HintName: EmptyRecord.cs
// <auto-generated />
#nullable enable

partial record EmptyRecord : System.IEquatable<EmptyRecord>, SpacetimeDB.BSATN.IStructuralReadWrite
{
    public void ReadFields(System.IO.BinaryReader reader) { }

    public void WriteFields(System.IO.BinaryWriter writer) { }

    public readonly partial struct BSATN : SpacetimeDB.BSATN.IReadWrite<EmptyRecord>
    {
        public EmptyRecord Read(System.IO.BinaryReader reader) =>
            SpacetimeDB.BSATN.IStructuralReadWrite.Read<EmptyRecord>(reader);

        public void Write(System.IO.BinaryWriter writer, EmptyRecord value)
        {
            value.WriteFields(writer);
        }

        public SpacetimeDB.BSATN.AlgebraicType.Ref GetAlgebraicType(
            SpacetimeDB.BSATN.ITypeRegistrar registrar
        ) =>
            registrar.RegisterType<EmptyRecord>(_ => new SpacetimeDB.BSATN.AlgebraicType.Product(
                new SpacetimeDB.BSATN.AggregateElement[] { }
            ));

        SpacetimeDB.BSATN.AlgebraicType SpacetimeDB.BSATN.IReadWrite<EmptyRecord>.GetAlgebraicType(
            SpacetimeDB.BSATN.ITypeRegistrar registrar
        ) => GetAlgebraicType(registrar);
    }

    public override int GetHashCode()
    {
        return 0;
    }

    public override string ToString()
    {
        return $"EmptyRecord()";
    }
} // EmptyRecord
