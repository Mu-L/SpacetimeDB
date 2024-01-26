use super::{
    btree_index::BTreeIndex,
    committed_state::{CommittedIndexIter, CommittedState},
    datastore::Result,
    indexes::{RowPointer, SquashedOffset},
    sequence::{Sequence, SequenceError, SequencesState},
    state_view::{Iter, IterByColRange, ScanIterByColRange, StateView},
    table::{IndexScanIter, InsertError, RowRef, Table},
    tx_state::TxState,
    SharedMutexGuard, SharedWriteGuard,
};
use crate::{
    address::Address,
    db::datastore::{
        system_tables::{
            table_name_is_system, StColumnRow, StConstraintFields, StConstraintRow, StIndexFields, StIndexRow,
            StSequenceFields, StSequenceRow, StTableFields, StTableRow, SystemTable, ST_COLUMNS_ID, ST_CONSTRAINTS_ID,
            ST_INDEXES_ID, ST_SEQUENCES_ID, ST_TABLES_ID,
        },
        traits::TxData,
    },
    error::{DBError, TableError},
    execution_context::ExecutionContext,
};
use anyhow::anyhow;
use spacetimedb_primitives::{ColList, ConstraintId, IndexId, SequenceId, TableId};
use spacetimedb_sats::{
    db::{
        def::{
            ConstraintDef, ConstraintSchema, IndexDef, IndexSchema, SequenceDef, SequenceSchema, TableDef, TableSchema,
            SEQUENCE_PREALLOCATION_AMOUNT,
        },
        error::SchemaErrors,
    },
    AlgebraicType, AlgebraicValue, ProductType, ProductValue,
};
use std::{
    borrow::Cow,
    ops::RangeBounds,
    time::{Duration, Instant},
};

/// Represents a Mutable transaction. Holds locks for its duration
///
/// The initialization of this struct is sensitive because improper
/// handling can lead to deadlocks. Therefore, it is strongly recommended to use
/// `Locking::begin_mut_tx()` for instantiation to ensure safe acquisition of locks.
#[allow(unused)]
pub struct MutTxId {
    pub(crate) tx_state: TxState,
    pub(crate) committed_state_write_lock: SharedWriteGuard<CommittedState>,
    pub(crate) sequence_state_lock: SharedMutexGuard<SequencesState>,
    pub(crate) lock_wait_time: Duration,
    pub(crate) timer: Instant,
}

impl MutTxId {
    fn validate_table(table_schema: &TableDef) -> Result<()> {
        if table_name_is_system(&table_schema.table_name) {
            return Err(TableError::System(table_schema.table_name.clone()).into());
        }

        table_schema
            .clone()
            .into_schema(0.into())
            .validated()
            .map_err(|err| DBError::Schema(SchemaErrors(err)))?;

        Ok(())
    }

    pub fn create_table(&mut self, table_schema: TableDef, database_address: Address) -> Result<TableId> {
        log::trace!("TABLE CREATING: {}", table_schema.table_name);

        Self::validate_table(&table_schema)?;

        // Insert the table row into `st_tables`
        // NOTE: Because `st_tables` has a unique index on `table_name`, this will
        // fail if the table already exists.
        let row = StTableRow {
            table_id: ST_TABLES_ID,
            table_name: table_schema.table_name.clone(),
            table_type: table_schema.table_type,
            table_access: table_schema.table_access,
        };
        let mut row = row.into();
        self.insert(ST_TABLES_ID, &mut row, database_address)?;
        let table_id = StTableRow::try_from(&row)?.table_id;

        // Generate the full definition of the table, with the generated indexes, constraints, sequences...
        let table_schema = table_schema.into_schema(table_id);

        // Insert the columns into `st_columns`
        for col in table_schema.columns() {
            let row = StColumnRow {
                table_id,
                col_pos: col.col_pos,
                col_name: col.col_name.clone(),
                col_type: col.col_type.clone(),
            };
            self.insert(ST_COLUMNS_ID, &mut row.into(), database_address)?;
        }

        // Create the in memory representation of the table
        // NOTE: This should be done before creating the indexes
        let mut schema_internal = table_schema.clone();
        // Remove the adjacent object that has an unset `id = 0`, they will be created below with the correct `id`
        schema_internal.clear_adjacent_schemas();

        self.create_table_internal(table_id, schema_internal);

        // Insert constraints into `st_constraints`
        for constraint in table_schema.constraints {
            self.create_constraint(constraint.table_id, constraint.into(), database_address)?;
        }

        // Insert sequences into `st_sequences`
        for seq in table_schema.sequences {
            self.create_sequence(seq.table_id, seq.into(), database_address)?;
        }

        // Create the indexes for the table
        for index in table_schema.indexes {
            self.create_index(table_id, index.into(), database_address)?;
        }

        log::trace!("TABLE CREATED: {}, table_id:{table_id}", table_schema.table_name);

        Ok(table_id)
    }

    fn create_table_internal(&mut self, table_id: TableId, schema: TableSchema) {
        self.tx_state
            .insert_tables
            .insert(table_id, Table::new(schema, SquashedOffset::TX_STATE));
    }

    fn get_row_type(&self, table_id: TableId) -> Option<&ProductType> {
        if let Some(row_type) = self
            .tx_state
            .insert_tables
            .get(&table_id)
            .map(|table| table.get_row_type())
        {
            return Some(row_type);
        }
        self.committed_state_write_lock
            .tables
            .get(&table_id)
            .map(|table| table.get_row_type())
    }

    pub fn row_type_for_table(&self, table_id: TableId, database_address: Address) -> Result<Cow<'_, ProductType>> {
        // Fetch the `ProductType` from the in memory table if it exists.
        // The `ProductType` is invalidated if the schema of the table changes.
        if let Some(row_type) = self.get_row_type(table_id) {
            return Ok(Cow::Borrowed(row_type));
        }

        let ctx = ExecutionContext::internal(database_address);

        // Look up the columns for the table in question.
        // NOTE: This is quite an expensive operation, although we only need
        // to do this in situations where there is not currently an in memory
        // representation of a table. This would happen in situations where
        // we have created the table in the database, but have not yet
        // represented in memory or inserted any rows into it.
        Ok(match self.schema_for_table(&ctx, table_id)? {
            Cow::Borrowed(x) => Cow::Borrowed(x.get_row_type()),
            Cow::Owned(x) => Cow::Owned(x.into_row_type()),
        })
    }

    pub fn drop_table(&mut self, _table_id: TableId, _database_address: Address) -> Result<()> {
        unimplemented!("Phoebe remains unconvinced that dynamic schema modifications are a good idea")
    }

    pub fn rename_table(&mut self, _table_id: TableId, _new_name: &str, _database_address: Address) -> Result<()> {
        unimplemented!("Phoebe remains unconvinced that dynamic schema modifications are a good idea")
    }

    pub fn table_id_from_name(&self, table_name: &str, database_address: Address) -> Result<Option<TableId>> {
        self.iter_by_col_eq(
            &ExecutionContext::internal(database_address),
            &ST_TABLES_ID,
            StTableFields::TableName.into(),
            AlgebraicValue::String(table_name.to_owned()),
        )
        .map(|mut iter| {
            iter.next().map(|row| {
                TableId(
                    *row.read_row().elements[StTableFields::TableId.col_idx()]
                        .as_u32()
                        .unwrap(),
                )
            })
        })
    }

    pub fn table_name_from_id<'a>(&'a self, ctx: &'a ExecutionContext, table_id: TableId) -> Result<Option<String>> {
        self.iter_by_col_eq(ctx, &ST_TABLES_ID, StTableFields::TableId.into(), table_id.into())
            .map(|mut iter| {
                iter.next().map(|row| {
                    let ProductValue { mut elements, .. } = row.read_row();
                    let elt = elements.swap_remove(StTableFields::TableName.col_idx());
                    elt.into_string().unwrap()
                })
            })
    }

    pub fn create_index(&mut self, table_id: TableId, index: IndexDef, database_address: Address) -> Result<IndexId> {
        log::trace!(
            "INDEX CREATING: {} for table: {} and col(s): {:?}",
            index.index_name,
            table_id,
            index.columns
        );
        if self.table_exists(&table_id).is_none() {
            return Err(TableError::IdNotFoundState(table_id).into());
        }

        // Insert the index row into st_indexes
        // NOTE: Because st_indexes has a unique index on index_name, this will
        // fail if the index already exists.
        let row = StIndexRow {
            index_id: 0.into(), // Autogen'd
            table_id,
            index_type: index.index_type,
            index_name: index.index_name.clone(),
            columns: index.columns.clone(),
            is_unique: index.is_unique,
        };
        let mut row = row.into();
        self.insert(ST_INDEXES_ID, &mut row, database_address)?;
        let index_id = StIndexRow::try_from(&row)?.index_id;

        let mut index = IndexSchema::from_def(table_id, index);
        index.index_id = index_id;
        let index_name = index.index_name.clone();
        let columns = index.columns.clone();
        self.create_index_internal(index, database_address)?;

        log::trace!(
            "INDEX CREATED: {} for table: {} and col(s): {:?}",
            index_name,
            table_id,
            columns
        );
        Ok(index_id)
    }

    fn create_index_internal(&mut self, index: IndexSchema, database_address: Address) -> Result<()> {
        let index_id = index.index_id;
        let table_id = index.table_id;

        let (table, blob_store) = if let Some(pair) = self.tx_state.get_table_and_blob_store_mut(table_id) {
            pair
        } else {
            let ctx = ExecutionContext::internal(database_address);
            let schema = self.schema_for_table(&ctx, table_id)?.into_owned();
            self.tx_state
                .insert_tables
                .insert(index.table_id, Table::new(schema, SquashedOffset::TX_STATE));
            self.tx_state.get_table_and_blob_store_mut(table_id).unwrap()
        };

        let mut insert_index = BTreeIndex::new(index.index_id, index.is_unique);
        insert_index.build_from_rows(&index.columns, table.scan_rows(blob_store))?;

        // NOTE: Also add all the rows in the already committed table to the index.
        // FIXME: Is this correct? Index scan iterators (incl. the existing `Locking` versions)
        // appear to assume that a table's index refers only to rows within that table,
        // and does not handle the case where a `TxState` index refers to `CommittedState` rows.
        if let Some(committed_table) = self.committed_state_write_lock.get_table(table_id) {
            insert_index.build_from_rows(
                &index.columns,
                committed_table.scan_rows(&self.committed_state_write_lock.blob_store),
            )?;
        }

        table.schema.indexes.push(IndexSchema {
            table_id: index.table_id,
            columns: index.columns.clone(),
            index_name: index.index_name.clone(),
            is_unique: index.is_unique,
            index_id,
            index_type: index.index_type,
        });

        table.indexes.insert(index.columns, insert_index);
        Ok(())
    }

    pub fn drop_index(&mut self, _index_id: IndexId, _database_address: Address) -> Result<()> {
        unimplemented!("Phoebe remains unconvinced that dynamic schema modifications are a good idea")
    }

    pub fn index_id_from_name(&self, index_name: &str, database_address: Address) -> Result<Option<IndexId>> {
        self.iter_by_col_eq(
            &ExecutionContext::internal(database_address),
            &ST_INDEXES_ID,
            StIndexFields::IndexName.into(),
            AlgebraicValue::String(index_name.to_owned()),
        )
        .map(|mut iter| {
            iter.next().map(|row| {
                IndexId(
                    *row.read_row().elements[StIndexFields::IndexId.col_idx()]
                        .as_u32()
                        .unwrap(),
                )
            })
        })
    }

    pub fn get_next_sequence_value(&mut self, seq_id: SequenceId, database_address: Address) -> Result<i128> {
        {
            let Some(sequence) = self.sequence_state_lock.get_sequence_mut(seq_id) else {
                return Err(SequenceError::NotFound(seq_id).into());
            };

            // If there are allocated sequence values, return the new value, if it is not bigger than
            // the upper range of `sequence.allocated`
            if let Some(value) = sequence.gen_next_value().filter(|v| v < &sequence.allocated()) {
                return Ok(value);
            }
        }
        // Allocate new sequence values
        // If we're out of allocations, then update the sequence row in st_sequences to allocate a fresh batch of sequences.
        let ctx = ExecutionContext::internal(database_address);
        let old_seq_row_ref = self
            .iter_by_col_eq(
                &ctx,
                &ST_SEQUENCES_ID,
                StSequenceFields::SequenceId.into(),
                seq_id.into(),
            )?
            .last()
            .unwrap();
        let old_seq_row = old_seq_row_ref.read_row();
        let old_seq_row_ptr = old_seq_row_ref.pointer();
        let seq_row = {
            let mut seq_row = StSequenceRow::try_from(&old_seq_row)?.to_owned();

            let Some(sequence) = self.sequence_state_lock.get_sequence_mut(seq_id) else {
                return Err(SequenceError::NotFound(seq_id).into());
            };
            seq_row.allocated = sequence.nth_value(SEQUENCE_PREALLOCATION_AMOUNT as usize);
            sequence.set_allocation(seq_row.allocated);
            seq_row
        };

        self.delete(ST_SEQUENCES_ID, old_seq_row_ptr)?;
        self.insert(ST_SEQUENCES_ID, &mut ProductValue::from(seq_row), database_address)?;

        let Some(sequence) = self.sequence_state_lock.get_sequence_mut(seq_id) else {
            return Err(SequenceError::NotFound(seq_id).into());
        };
        if let Some(value) = sequence.gen_next_value() {
            return Ok(value);
        }
        Err(SequenceError::UnableToAllocate(seq_id).into())
    }

    pub fn create_sequence(
        &mut self,
        table_id: TableId,
        seq: SequenceDef,
        database_address: Address,
    ) -> Result<SequenceId> {
        log::trace!(
            "SEQUENCE CREATING: {} for table: {} and col: {}",
            seq.sequence_name,
            table_id,
            seq.col_pos
        );

        // Insert the sequence row into st_sequences
        // NOTE: Because st_sequences has a unique index on sequence_name, this will
        // fail if the table already exists.
        let sequence_row = StSequenceRow {
            sequence_id: 0.into(), // autogen'd
            sequence_name: seq.sequence_name,
            table_id,
            col_pos: seq.col_pos,
            allocated: seq.allocated,
            increment: seq.increment,
            start: seq.start.unwrap_or(1),
            min_value: seq.min_value.unwrap_or(1),
            max_value: seq.max_value.unwrap_or(i128::MAX),
        };
        let mut row = sequence_row.into();
        self.insert(ST_SEQUENCES_ID, &mut row, database_address)?;
        // TODO(centril): `row` is already owned, so pass that in.
        let sequence_row = StSequenceRow::try_from(&row)?.to_owned();
        let sequence_id = sequence_row.sequence_id;

        let schema: SequenceSchema = sequence_row.into();
        let insert_table = self.get_insert_table_mut(schema.table_id)?;
        insert_table.schema.update_sequence(schema.clone());
        self.sequence_state_lock
            .sequences
            .insert(schema.sequence_id, Sequence::new(schema));

        log::trace!("SEQUENCE CREATED: id = {}", sequence_id);

        Ok(sequence_id)
    }

    pub fn drop_sequence(&mut self, _sequence_id: SequenceId, _database_address: Address) -> Result<()> {
        unimplemented!("Phoebe remains unconvinced that dynamic schema modifications are a good idea")
    }

    pub fn sequence_id_from_name(&self, seq_name: &str, database_address: Address) -> Result<Option<SequenceId>> {
        self.iter_by_col_eq(
            &ExecutionContext::internal(database_address),
            &ST_SEQUENCES_ID,
            StSequenceFields::SequenceName.into(),
            AlgebraicValue::String(seq_name.to_owned()),
        )
        .map(|mut iter| {
            iter.next().map(|row| {
                let row = row.read_row();
                let id = row.elements[StSequenceFields::SequenceId.col_idx()].as_u32().unwrap();
                (*id).into()
            })
        })
    }

    fn create_constraint(
        &mut self,
        table_id: TableId,
        constraint: ConstraintDef,
        database_address: Address,
    ) -> Result<ConstraintId> {
        log::trace!(
            "CONSTRAINT CREATING: {} for table: {} and cols: {:?}",
            constraint.constraint_name,
            table_id,
            constraint.columns
        );

        // Verify we have 1 column if need `auto_inc`
        if constraint.constraints.has_autoinc() && constraint.columns.len() != 1 {
            return Err(SequenceError::MultiColumnAutoInc(table_id, constraint.columns).into());
        };

        // Insert the constraint row into st_constraint
        // NOTE: Because st_constraint has a unique index on constraint_name, this will
        // fail if the table already exists.
        let constraint_row = StConstraintRow {
            constraint_id: 0.into(), // autogen'd
            columns: constraint.columns.clone(),
            constraint_name: constraint.constraint_name.clone(),
            constraints: constraint.constraints,
            table_id,
        };

        let mut row = ProductValue::from(constraint_row);
        self.insert(ST_CONSTRAINTS_ID, &mut row, database_address)?;
        let constraint_row = StConstraintRow::try_from(&row)?;
        let constraint_id = constraint_row.constraint_id;

        let mut constraint = ConstraintSchema::from_def(table_id, constraint);
        constraint.constraint_id = constraint_id;
        let insert_table = self.get_insert_table_mut(constraint.table_id)?;
        let constraint_name = constraint.constraint_name.clone();
        insert_table.schema.update_constraint(constraint);

        log::trace!("CONSTRAINT CREATED: {}", constraint_name);

        Ok(constraint_id)
    }

    fn get_insert_table_mut(&mut self, table_id: TableId) -> Result<&mut Table> {
        self.tx_state
            .get_table_and_blob_store_mut(table_id)
            .map(|(tbl, _)| tbl)
            .ok_or_else(|| TableError::IdNotFoundState(table_id).into())
    }

    pub fn drop_constraint(&mut self, _constraint_id: ConstraintId, _database_address: Address) -> Result<()> {
        unimplemented!("Phoebe remains unconvinced that dynamic schema modifications are a good idea")
    }

    pub fn constraint_id_from_name(
        &self,
        constraint_name: &str,
        database_address: Address,
    ) -> Result<Option<ConstraintId>> {
        self.iter_by_col_eq(
            &ExecutionContext::internal(database_address),
            &ST_CONSTRAINTS_ID,
            StConstraintFields::ConstraintName.into(),
            AlgebraicValue::String(constraint_name.to_owned()),
        )
        .map(|mut iter| {
            iter.next().map(|row| {
                let row = row.read_row();
                let id = row.elements[StConstraintFields::ConstraintId.col_idx()]
                    .as_u32()
                    .unwrap();
                (*id).into()
            })
        })
    }

    // TODO(perf, deep-integration):
    //   When all of [`Table::read_row`], [`RowRef::new`], [`CommittedState::get`]
    //   and [`TxState::get`] become unsafe,
    //   make this method `unsafe` as well.
    //   Add the following to the docs:
    //
    // # Safety
    //
    // `pointer` must refer to a row within the table at `table_id`
    // which was previously inserted and has not been deleted since.
    //
    // See [`RowRef::new`] for more detailed requirements.
    //
    // Showing that `pointer` was the result of a call to `self.insert`
    // with `table_id`
    // and has not been passed to `self.delete`
    // is sufficient to demonstrate that a call to `self.get` is safe.
    pub fn get(&self, table_id: TableId, row_ptr: RowPointer) -> Result<Option<RowRef<'_>>> {
        if self.table_exists(&table_id).is_none() {
            return Err(TableError::IdNotFound(SystemTable::st_table, table_id.0).into());
        }
        Ok(match row_ptr.squashed_offset() {
            SquashedOffset::TX_STATE => Some(
                // TODO(perf, deep-integration):
                // See above. Once `TxState::get` is unsafe, justify with:
                //
                // Our invariants satisfy `TxState::get`.
                self.tx_state.get(table_id, row_ptr),
            ),
            SquashedOffset::COMMITTED_STATE => {
                if self.tx_state.is_deleted(table_id, row_ptr) {
                    None
                } else {
                    Some(
                        // TODO(perf, deep-integration):
                        // See above. Once `CommittedState::get` is unsafe, justify with:
                        //
                        // Our invariants satisfy `CommittedState::get`.
                        self.committed_state_write_lock.get(table_id, row_ptr),
                    )
                }
            }
            _ => unreachable!("Invalid SquashedOffset for row pointer: {:?}", row_ptr),
        })
    }

    pub fn commit(self) -> TxData {
        let Self {
            mut committed_state_write_lock,
            tx_state,
            ..
        } = self;
        committed_state_write_lock.merge(tx_state)
    }

    pub fn rollback(self) {
        // TODO: Check that no sequences exceed their allocation after the rollback.
    }

    fn algebraic_type_is_numeric(ty: &AlgebraicType) -> bool {
        matches!(
            *ty,
            AlgebraicType::I8
                | AlgebraicType::U8
                | AlgebraicType::I16
                | AlgebraicType::U16
                | AlgebraicType::I32
                | AlgebraicType::U32
                | AlgebraicType::I64
                | AlgebraicType::U64
                | AlgebraicType::I128
                | AlgebraicType::U128
        )
    }

    fn sequence_value_to_algebraic_value(ty: &AlgebraicType, sequence_value: i128) -> AlgebraicValue {
        match *ty {
            AlgebraicType::I8 => (sequence_value as i8).into(),
            AlgebraicType::U8 => (sequence_value as u8).into(),
            AlgebraicType::I16 => (sequence_value as i16).into(),
            AlgebraicType::U16 => (sequence_value as u16).into(),
            AlgebraicType::I32 => (sequence_value as i32).into(),
            AlgebraicType::U32 => (sequence_value as u32).into(),
            AlgebraicType::I64 => (sequence_value as i64).into(),
            AlgebraicType::U64 => (sequence_value as u64).into(),
            AlgebraicType::I128 => sequence_value.into(),
            AlgebraicType::U128 => (sequence_value as u128).into(),
            _ => unreachable!("should have been prevented in `MutTxId::apply_sequences_to_autoinc_columns`"),
        }
    }

    pub fn insert(&mut self, table_id: TableId, row: &mut ProductValue, database_address: Address) -> Result<()> {
        let ctx = ExecutionContext::internal(database_address);

        // TODO: Executing schema_for_table for every row insert is expensive.
        // However we ask for the schema in the [Table] struct instead.
        let schema = self.schema_for_table(&ctx, table_id)?;

        let mut col_to_update = None;
        for seq in &schema.sequences {
            if !row.elements[usize::from(seq.col_pos)].is_numeric_zero() {
                continue;
            }
            for seq_row in self.iter_by_col_eq(
                &ctx,
                &ST_SEQUENCES_ID,
                StSequenceFields::TableId.into(),
                table_id.into(),
            )? {
                let seq_row = seq_row.read_row();
                let seq_row = StSequenceRow::try_from(&seq_row)?;
                if seq_row.col_pos != seq.col_pos {
                    continue;
                }

                col_to_update = Some((seq.col_pos, seq_row.sequence_id));
                break;
            }
        }

        if let Some((col_id, sequence_id)) = col_to_update {
            let col_idx = col_id.idx();
            let col = &schema.columns()[col_idx];
            if !Self::algebraic_type_is_numeric(&col.col_type) {
                return Err(SequenceError::NotInteger {
                    col: format!("{}.{}", &schema.table_name, &col.col_name),
                    found: col.col_type.clone(),
                }
                .into());
            }
            // At this point, we know this will be essentially a cheap copy.
            let col_ty = col.col_type.clone();
            let seq_val = self.get_next_sequence_value(sequence_id, database_address)?;
            row.elements[col_idx] = Self::sequence_value_to_algebraic_value(&col_ty, seq_val);
        }

        self.insert_row_internal(table_id, row)?;

        Ok(())
    }

    pub fn insert_row_internal(&mut self, table_id: TableId, row: &ProductValue) -> Result<()> {
        let commit_table = self.committed_state_write_lock.get_table(table_id);

        // Check for constraint violations as early as possible,
        // to ensure that `UniqueConstraintViolation` errors have precedence over other errors.
        // `tx_table.insert` will later perform the same check on the tx table,
        // so this method needs only to check the committed state.
        if let Some(commit_table) = commit_table {
            commit_table
                .check_unique_constraints(row, |maybe_conflict| self.tx_state.is_deleted(table_id, maybe_conflict))?;
        }

        // Get the insert table, so we can write the row into it.
        let (tx_table, tx_blob_store, delete_table) = self
            .tx_state
            .get_table_and_blob_store_or_maybe_create_from(table_id, commit_table)
            .ok_or(TableError::IdNotFoundState(table_id))?;

        match tx_table.insert(tx_blob_store, row) {
            Ok((hash, ptr)) => {
                // `row` not previously present in insert tables,
                // but may still be a set-semantic conflict with a row
                // in the committed state.

                if let Some(commit_table) = commit_table {
                    // Safety:
                    // - `commit_table` and `tx_table` use the same schema
                    //   because `tx_table` is derived from `commit_table`.
                    // - `ptr` and `hash` are correct because we just got them from `tx_table.insert`.
                    if let Some(committed_ptr) = unsafe { Table::find_same_row(commit_table, tx_table, ptr, hash) } {
                        // If `row` was already present in the committed state,
                        // either this is a set-semantic duplicate,
                        // or the row is marked as deleted, so we will undelete it
                        // and leave it in the committed state.
                        // Either way, it should not appear in the insert tables,
                        // so roll back the insertion.
                        tx_table
                            .delete(tx_blob_store, ptr)
                            .expect("Failed to delete a row we just inserted");

                        // It's possible that `row` appears in the committed state,
                        // but is marked as deleted.
                        // In this case, undelete it, so it remains in the committed state.
                        delete_table.remove(&committed_ptr);
                    }
                }
                Ok(())
            }
            // `row` previously present in insert tables; do nothing.
            Err(InsertError::Duplicate(_)) => Ok(()),

            // Index error: return more structured error type
            // rather than `anyhow::Error`.
            Err(InsertError::IndexError(e)) => Err(e.into()),

            // Misc. insertion error; fail.
            Err(e) => Err(anyhow!("Table::insert failed: {:?}", e).into()),
        }
    }

    pub fn delete(&mut self, table_id: TableId, row_pointer: RowPointer) -> Result<bool> {
        match row_pointer.squashed_offset() {
            // For newly-inserted rows,
            // just delete them from the insert tables
            // - there's no reason to have them in both the insert and delete tables.
            SquashedOffset::TX_STATE => self
                .tx_state
                .with_table_and_blob_store(table_id, |table, blob_store| {
                    table.delete(blob_store, row_pointer).is_some()
                })
                .ok_or_else(|| TableError::IdNotFoundState(table_id).into()),
            SquashedOffset::COMMITTED_STATE => {
                // NOTE: We trust the `row_pointer` refers to an extant row,
                // and check only that it hasn't yet been deleted.
                let delete_table = self.tx_state.get_delete_table_mut(table_id);

                Ok(delete_table.insert(row_pointer))
            }
            _ => unreachable!("Invalid SquashedOffset for RowPointer: {:?}", row_pointer),
        }
    }

    pub fn delete_by_rel(&mut self, table_id: TableId, rel: &ProductValue) -> Result<bool> {
        // Four cases here:
        // - Table exists in both tx_state and committed_state.
        //   - Temporary insert into tx_state.
        //   - If match exists in tx_state, delete it immediately.
        //   - Else if match exists in committed_state, add to delete tables.
        //   - Roll back temp insertion.
        // - Table exists only in tx_state.
        //   - As above, but without else branch.
        // - Table exists only in committed_state.
        //   - Create table in tx_state, then as above.
        // - Table does not exist.
        //   - No such row; return false.

        let commit_table = self.committed_state_write_lock.get_table_mut(table_id);

        // If the tx table exists, get it.
        // If it doesn't exist, but the commit table does,
        // create the tx table using the commit table as a template.
        let Some((tx_table, tx_blob_store, delete_table)) = self
            .tx_state
            .get_table_and_blob_store_or_maybe_create_from(table_id, commit_table.as_deref())
        else {
            // If neither the committed table nor the tx table exists,
            // the row can't exist, so delete nothing.
            return Ok(false);
        };

        match tx_table.insert(tx_blob_store, rel) {
            Ok((hash, ptr)) => {
                // Not present in insert tables; attempt to delete from commit tables.
                let to_delete = commit_table.and_then(|commit_table| {
                    // Safety:
                    // - `commit_table` and `tx_table` use the same schema
                    //   because `tx_table` is derived from `commit_table`.
                    // - `ptr` and `hash` are correct because we just got them from `tx_table.insert`.
                    unsafe { Table::find_same_row(commit_table, tx_table, ptr, hash) }
                });

                // Remove the temporary entry from the insert tables.
                // Do this before actually deleting to drop the borrows on the tables.
                tx_table.delete(tx_blob_store, ptr);

                // Mark the committed row to be deleted by adding it to the delete table.
                Ok(to_delete
                    .map(|to_delete| delete_table.insert(to_delete))
                    .unwrap_or(false))
            }

            Err(InsertError::Duplicate(existing)) => {
                // Present in insert tables; delete.
                // Set semantics mean no need to check commit tables.
                tx_table
                    .delete(tx_blob_store, existing)
                    .expect("Discovered pointer from Table::insert of duplicate, but failed to delete");
                Ok(true)
            }

            Err(e) => Err(anyhow!("Table::insert failed: {:?}", e).into()),
        }
    }
}

impl StateView for MutTxId {
    fn get_schema(&self, table_id: &TableId) -> Option<&TableSchema> {
        if let Some(row_type) = self
            .tx_state
            .insert_tables
            .get(table_id)
            .map(|table| table.get_schema())
        {
            return Some(row_type);
        }
        self.committed_state_write_lock
            .tables
            .get(table_id)
            .map(|table| table.get_schema())
    }

    fn iter<'a>(&'a self, ctx: &'a ExecutionContext, table_id: &TableId) -> Result<Iter<'a>> {
        if let Some(table_name) = self.table_exists(table_id) {
            return Ok(Iter::new(
                ctx,
                *table_id,
                table_name,
                Some(&self.tx_state),
                &self.committed_state_write_lock,
            ));
        }
        Err(TableError::IdNotFound(SystemTable::st_table, table_id.0).into())
    }

    fn table_exists(&self, table_id: &TableId) -> Option<&str> {
        // TODO(bikeshedding, docs): should this also check if the schema is in the system tables,
        // but the table hasn't been constructed yet?
        // If not, document why.
        if let Some(table) = self.tx_state.insert_tables.get(table_id) {
            Some(&table.schema.table_name)
        } else if let Some(table) = self.committed_state_write_lock.tables.get(table_id) {
            Some(&table.schema.table_name)
        } else {
            None
        }
    }

    fn iter_by_col_range<'a, R: RangeBounds<AlgebraicValue>>(
        &'a self,
        ctx: &'a ExecutionContext,
        table_id: &TableId,
        cols: ColList,
        range: R,
    ) -> Result<IterByColRange<'a, R>> {
        // We have to index_seek in both the committed state and the current tx state.
        // First, we will check modifications in the current tx. It may be that the table
        // has not been modified yet in the current tx, in which case we will only search
        // the committed state. Finally, the table may not be indexed at all, in which case
        // we fall back to iterating the entire table.
        //
        // We need to check the tx_state first. In particular, it may be that the index
        // was only added in the current transaction.
        // TODO(george): It's unclear that we truly support dynamically creating an index
        // yet. In particular, I don't know if creating an index in a transaction and
        // rolling it back will leave the index in place.
        if let Some(inserted_rows) = self.tx_state.index_seek(*table_id, &cols, &range) {
            // The current transaction has modified this table, and the table is indexed.
            Ok(IterByColRange::Index(IndexSeekIterMutTxId {
                ctx,
                table_id: *table_id,
                tx_state: &self.tx_state,
                inserted_rows,
                committed_rows: self.committed_state_write_lock.index_seek(*table_id, &cols, &range),
                committed_state: &self.committed_state_write_lock,
                num_committed_rows_fetched: 0,
            }))
        } else {
            // Either the current transaction has not modified this table, or the table is not
            // indexed.
            match self.committed_state_write_lock.index_seek(*table_id, &cols, &range) {
                Some(committed_rows) => Ok(IterByColRange::CommittedIndex(CommittedIndexIter::new(
                    ctx,
                    *table_id,
                    Some(&self.tx_state),
                    &self.committed_state_write_lock,
                    committed_rows,
                ))),
                None => Ok(IterByColRange::Scan(ScanIterByColRange::new(
                    self.iter(ctx, table_id)?,
                    cols,
                    range,
                ))),
            }
        }
    }
}

// ### iterators

#[allow(unused)]
pub struct IndexSeekIterMutTxId<'a> {
    ctx: &'a ExecutionContext<'a>,
    table_id: TableId,
    tx_state: &'a TxState,
    committed_state: &'a CommittedState,
    inserted_rows: IndexScanIter<'a>,
    committed_rows: Option<IndexScanIter<'a>>,
    num_committed_rows_fetched: u64,
}

impl Drop for IndexSeekIterMutTxId<'_> {
    fn drop(&mut self) {
        // // Increment number of index seeks
        // DB_METRICS
        //     .rdb_num_index_seeks
        //     .with_label_values(
        //         &self.ctx.workload(),
        //         &self.ctx.database(),
        //         self.ctx.reducer_or_query(),
        //         &self.table_id.0,
        //     )
        //     .inc();

        // // Increment number of index keys scanned
        // DB_METRICS
        //     .rdb_num_keys_scanned
        //     .with_label_values(
        //         &self.ctx.workload(),
        //         &self.ctx.database(),
        //         self.ctx.reducer_or_query(),
        //         &self.table_id.0,
        //     )
        //     .inc_by(self.committed_rows.as_ref().map_or(0, |iter| iter.keys_scanned()));

        // // Increment number of rows fetched
        // DB_METRICS
        //     .rdb_num_rows_fetched
        //     .with_label_values(
        //         &self.ctx.workload(),
        //         &self.ctx.database(),
        //         self.ctx.reducer_or_query(),
        //         &self.table_id.0,
        //     )
        //     .inc_by(self.num_committed_rows_fetched);
    }
}

impl<'a> Iterator for IndexSeekIterMutTxId<'a> {
    type Item = RowRef<'a>;

    // #[tracing::instrument(skip_all)]
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(row_ref) = self.inserted_rows.next() {
            return Some(row_ref);
        }

        if let Some(row_ref) = self
            .committed_rows
            .as_mut()
            .and_then(|i| i.find(|row_ref| !self.tx_state.is_deleted(self.table_id, row_ref.pointer())))
        {
            // TODO(metrics): This doesn't actually fetch a row.
            // Move this counter to `RowRef::read_row`.
            self.num_committed_rows_fetched += 1;
            return Some(row_ref);
        }

        None
    }
}
