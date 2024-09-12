﻿//HintName: MultiTableRow.cs
// <auto-generated />
#nullable enable

partial struct MultiTableRow : SpacetimeDB.Internal.ITable<MultiTableRow>
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

        public SpacetimeDB.BSATN.AlgebraicType GetAlgebraicType(
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
    }

    static IEnumerable<SpacetimeDB.Internal.TableDesc> SpacetimeDB.Internal.ITable<MultiTableRow>.MakeTableDesc(
        SpacetimeDB.BSATN.ITypeRegistrar registrar
    ) =>
        [
            new(
                new(
                    TableName: nameof(SpacetimeDB.Local.MultiTable1),
                    Columns:
                    [
                        new(nameof(Name), BSATN.Name.GetAlgebraicType(registrar)),
                        new(nameof(Foo), BSATN.Foo.GetAlgebraicType(registrar)),
                        new(nameof(Bar), BSATN.Bar.GetAlgebraicType(registrar))
                    ],
                    Indexes:
                    [
                        new("bt_MultiTable1_Name", false, SpacetimeDB.Internal.IndexType.BTree, [0])
                    ],
                    Constraints:
                    [
                        new(
                            "MultiTable1_Foo",
                            (byte)SpacetimeDB.Internal.ColumnAttrs.PrimaryKeyAuto,
                            [1]
                        )
                    ],
                    Sequences: [],
                    // "system" | "user"
                    TableType: "user",
                    // "public" | "private"
                    TableAccess: "public",
                    Scheduled: null
                ),
                (uint)
                    (
                        (SpacetimeDB.BSATN.AlgebraicType.Ref)new BSATN().GetAlgebraicType(registrar)
                    ).Ref_
            ),
            new(
                new(
                    TableName: nameof(SpacetimeDB.Local.MultiTable2),
                    Columns:
                    [
                        new(nameof(Name), BSATN.Name.GetAlgebraicType(registrar)),
                        new(nameof(Foo), BSATN.Foo.GetAlgebraicType(registrar)),
                        new(nameof(Bar), BSATN.Bar.GetAlgebraicType(registrar))
                    ],
                    Indexes: [],
                    Constraints:
                    [
                        new("MultiTable2_Foo", (byte)SpacetimeDB.Internal.ColumnAttrs.AutoInc, [1]),
                        new("MultiTable2_Bar", (byte)SpacetimeDB.Internal.ColumnAttrs.Unique, [2])
                    ],
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
} // MultiTableRow
