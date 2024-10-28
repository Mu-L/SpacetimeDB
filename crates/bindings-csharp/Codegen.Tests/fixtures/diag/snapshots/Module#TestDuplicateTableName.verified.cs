﻿//HintName: TestDuplicateTableName.cs
// <auto-generated />
#nullable enable

partial struct TestDuplicateTableName : SpacetimeDB.Internal.ITable<TestDuplicateTableName>
{
    public void ReadFields(System.IO.BinaryReader reader) { }

    public void WriteFields(System.IO.BinaryWriter writer) { }

    public readonly partial struct BSATN : SpacetimeDB.BSATN.IReadWrite<TestDuplicateTableName>
    {
        public TestDuplicateTableName Read(System.IO.BinaryReader reader) =>
            SpacetimeDB.BSATN.IStructuralReadWrite.Read<TestDuplicateTableName>(reader);

        public void Write(System.IO.BinaryWriter writer, TestDuplicateTableName value)
        {
            value.WriteFields(writer);
        }

        public SpacetimeDB.BSATN.AlgebraicType GetAlgebraicType(
            SpacetimeDB.BSATN.ITypeRegistrar registrar
        ) =>
            registrar.RegisterType<TestDuplicateTableName>(
                _ => new SpacetimeDB.BSATN.AlgebraicType.Product(
                    new SpacetimeDB.BSATN.AggregateElement[] { }
                )
            );
    }

    static IEnumerable<SpacetimeDB.Internal.TableDesc> SpacetimeDB.Internal.ITable<TestDuplicateTableName>.MakeTableDesc(
        SpacetimeDB.BSATN.ITypeRegistrar registrar
    ) =>
        [
            new(
                new(
                    TableName: nameof(SpacetimeDB.Local.TestDuplicateTableName),
                    Columns: [],
                    Indexes: [],
                    Constraints: [],
                    Sequences: [],
                    // "system" | "user"
                    TableType: "user",
                    // "public" | "private"
                    TableAccess: "private",
                    Scheduled: null
                ),
                (uint)
                    (
                        (SpacetimeDB.BSATN.AlgebraicType.Ref)new BSATN().GetAlgebraicType(registrar)
                    ).Ref_
            ),
        ];
} // TestDuplicateTableName
