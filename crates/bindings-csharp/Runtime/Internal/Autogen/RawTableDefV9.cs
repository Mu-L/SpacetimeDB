// THIS FILE IS AUTOMATICALLY GENERATED BY SPACETIMEDB. EDITS TO THIS FILE
// WILL NOT BE SAVED. MODIFY TABLES IN RUST INSTEAD.
// <auto-generated />

#nullable enable

using System;
using SpacetimeDB;
using System.Collections.Generic;
using System.Runtime.Serialization;

namespace SpacetimeDB.Internal
{
	[SpacetimeDB.Type]
	[DataContract]
	public partial class RawTableDefV9
	{
		[DataMember(Name = "name")]
		public string Name;
		[DataMember(Name = "product_type_ref")]
		public uint ProductTypeRef;
		[DataMember(Name = "primary_key")]
		public ushort? PrimaryKey;
		[DataMember(Name = "indexes")]
		public System.Collections.Generic.List<SpacetimeDB.Internal.RawIndexDefV9> Indexes;
		[DataMember(Name = "constraints")]
		public System.Collections.Generic.List<SpacetimeDB.Internal.RawConstraintDefV9> Constraints;
		[DataMember(Name = "sequences")]
		public System.Collections.Generic.List<SpacetimeDB.Internal.RawSequenceDefV9> Sequences;
		[DataMember(Name = "schedule")]
		public SpacetimeDB.Internal.RawScheduleDefV9? Schedule;
		[DataMember(Name = "table_type")]
		public SpacetimeDB.Internal.TableType TableType;
		[DataMember(Name = "table_access")]
		public SpacetimeDB.Internal.TableAccess TableAccess;

		public RawTableDefV9(
			string Name,
			uint ProductTypeRef,
			ushort? PrimaryKey,
			System.Collections.Generic.List<SpacetimeDB.Internal.RawIndexDefV9> Indexes,
			System.Collections.Generic.List<SpacetimeDB.Internal.RawConstraintDefV9> Constraints,
			System.Collections.Generic.List<SpacetimeDB.Internal.RawSequenceDefV9> Sequences,
			SpacetimeDB.Internal.RawScheduleDefV9? Schedule,
			SpacetimeDB.Internal.TableType TableType,
			SpacetimeDB.Internal.TableAccess TableAccess
		)
		{
			this.Name = Name;
			this.ProductTypeRef = ProductTypeRef;
			this.PrimaryKey = PrimaryKey;
			this.Indexes = Indexes;
			this.Constraints = Constraints;
			this.Sequences = Sequences;
			this.Schedule = Schedule;
			this.TableType = TableType;
			this.TableAccess = TableAccess;
		}

		public RawTableDefV9()
		{
			this.Name = "";
			this.Indexes = new();
			this.Constraints = new();
			this.Sequences = new();
		}

	}
}
