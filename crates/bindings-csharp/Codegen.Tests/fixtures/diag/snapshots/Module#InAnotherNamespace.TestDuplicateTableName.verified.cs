﻿//HintName: InAnotherNamespace.TestDuplicateTableName.cs
// <auto-generated />
#nullable enable

partial class InAnotherNamespace
{
    partial struct TestDuplicateTableName
        : System.IEquatable<TestDuplicateTableName>,
            SpacetimeDB.BSATN.IStructuralReadWrite
    {
        public void ReadFields(System.IO.BinaryReader reader) { }

        public void WriteFields(System.IO.BinaryWriter writer) { }

        public readonly partial struct BSATN
            : SpacetimeDB.BSATN.IReadWrite<InAnotherNamespace.TestDuplicateTableName>
        {
            public InAnotherNamespace.TestDuplicateTableName Read(System.IO.BinaryReader reader) =>
                SpacetimeDB.BSATN.IStructuralReadWrite.Read<InAnotherNamespace.TestDuplicateTableName>(
                    reader
                );

            public void Write(
                System.IO.BinaryWriter writer,
                InAnotherNamespace.TestDuplicateTableName value
            )
            {
                value.WriteFields(writer);
            }

            public SpacetimeDB.BSATN.AlgebraicType.Ref GetAlgebraicType(
                SpacetimeDB.BSATN.ITypeRegistrar registrar
            ) =>
                registrar.RegisterType<InAnotherNamespace.TestDuplicateTableName>(
                    _ => new SpacetimeDB.BSATN.AlgebraicType.Product(
                        new SpacetimeDB.BSATN.AggregateElement[] { }
                    )
                );

            SpacetimeDB.BSATN.AlgebraicType SpacetimeDB.BSATN.IReadWrite<InAnotherNamespace.TestDuplicateTableName>.GetAlgebraicType(
                SpacetimeDB.BSATN.ITypeRegistrar registrar
            ) => GetAlgebraicType(registrar);
        }

        public override int GetHashCode()
        {
            return;
        }

        public override string ToString()
        {
            return $"TestDuplicateTableName(";
        }

        public bool Equals(InAnotherNamespace.TestDuplicateTableName that)
        {
            return;
        }

        public override bool Equals(object? that)
        {
            if (that == null)
            {
                return false;
            }
            var that_ = that as InAnotherNamespace.TestDuplicateTableName?;
            if (that_ == null)
            {
                return false;
            }
            return Equals(that);
        }

        public static bool operator ==(
            InAnotherNamespace.TestDuplicateTableName this_,
            InAnotherNamespace.TestDuplicateTableName that
        )
        {
            if (((object)this_) == null || ((object)that) == null)
            {
                return Object.Equals(this_, that);
            }
            return this_.Equals(that);
        }

        public static bool operator !=(
            InAnotherNamespace.TestDuplicateTableName this_,
            InAnotherNamespace.TestDuplicateTableName that
        )
        {
            return !(this_ == that);
        }
    } // TestDuplicateTableName
} // InAnotherNamespace
