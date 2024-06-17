// TODO(jgilles): migrate this out of sats?

use crate::def::*;
use spacetimedb_primitives::*;
use spacetimedb_sats::db::auth::{StAccess, StTableType};
use spacetimedb_sats::db::raw_def::IndexType;
use spacetimedb_sats::product_value::InvalidFieldError;
use spacetimedb_sats::relation::{Column, DbTable, FieldName, Header};
use spacetimedb_sats::{AlgebraicType, ProductType, ProductTypeElement};
use std::sync::Arc;

/// Represents a schema definition for a database sequence.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SequenceSchema {
    pub sequence_id: SequenceId,
    pub table_id: TableId,
    /// The position of the column associated with this sequence.
    pub col_pos: ColId,
    pub increment: i128,
    pub start: i128,
    pub min_value: i128,
    pub max_value: i128,
    pub allocated: i128,
}

impl SequenceSchema {
    /*
    /// Creates a new [SequenceSchema] instance from a [SequenceDef] and a `table_id`.
    ///
    /// # Arguments
    ///
    /// * `table_id` - The ID of the table associated with the sequence.
    /// * `sequence` - The [SequenceDef] to derive the schema from.
    ///
    /// # Example
    ///
    /// ```
    /// use spacetimedb_sats::db::def::*;
    ///
    /// let sequence_def = SequenceDef::for_column("my_table".into(), "my_sequence".into(), 1.into());
    /// let schema = SequenceSchema::from_def(42.into(), sequence_def);
    ///
    /// assert_eq!(schema.table_id, 42.into());
    /// ```
    pub fn from_def(table_id: TableId, sequence: SequenceDef) -> Self {
        Self {
            sequence_id: SequenceId(0), // Will be replaced later when created
            table_id,
            col_pos: sequence.col_pos,
            increment: 1,
            start: sequence.start.unwrap_or(1),
            min_value: sequence.min_value.unwrap_or(1),
            max_value: sequence.max_value.unwrap_or(i128::MAX),
            allocated: SEQUENCE_PREALLOCATION_AMOUNT,
        }
    }
    */
}

/// A struct representing the schema of a database index.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IndexSchema {
    pub index_id: IndexId,
    pub table_id: TableId,
    pub index_type: IndexType,
    pub columns: ColList,
}

impl IndexSchema {
    /*
    /// Constructs an [IndexSchema] from a given [IndexDef] and `table_id`.
    pub fn from_def(table_id: TableId, index: IndexDef) -> Self {
        IndexSchema {
            index_id: IndexId(0), // Set to 0 as it may be assigned later.
            table_id,
            index_type: index.index_type,
            columns: index.columns,
        }
    }
    */
}

/// A struct representing the schema of a database column.
#[derive(Debug, Clone, PartialEq, Eq, Ord, PartialOrd)]
pub struct ColumnSchema {
    pub table_id: TableId,
    /// Position of the column within the table.
    pub col_pos: ColId,
    pub col_name: Box<str>,
    pub col_type: AlgebraicType,
}

impl ColumnSchema {
    /// Constructs a [ColumnSchema] from a given [ColumnDef] and `table_id`.
    ///
    /// # Arguments
    ///
    /// * `table_id`: Identifier of the table to which the column belongs.
    /// * `col_pos`: Position of the column within the table.
    /// * `column`: The `ColumnDef` containing column information.
    pub fn from_def(table_id: TableId, col_pos: ColId, column: ColumnDef) -> Self {
        ColumnSchema {
            table_id,
            col_pos,
            col_name: column.col_name.trim().into(),
            col_type: column.col_type,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UniqueConstraintSchema {
    pub constraint_id: ConstraintId,
    pub table_id: TableId,
    pub columns: ColList,
}

/// A data structure representing the schema of a database table.
///
/// This struct holds information about the table, including its identifier,
/// name, columns, indexes, constraints, sequences, type, and access rights.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TableSchema {
    pub table_id: TableId,
    pub table_name: Box<str>,
    columns: Vec<ColumnSchema>,
    pub indexes: Vec<IndexSchema>,
    pub unique_constraints: Vec<UniqueConstraintSchema>,
    pub sequences: Vec<SequenceSchema>,
    pub table_type: StTableType,
    pub table_access: StAccess,
    /// Cache for `row_type_for_table` in the data store.
    row_type: ProductType,
}

impl TableSchema {
    #[allow(clippy::too_many_arguments, clippy::boxed_local)]
    pub fn new(
        _table_id: TableId,
        _table_name: Box<str>,
        _columns: Vec<ColumnSchema>,
        _indexes: Vec<IndexSchema>,
        _constraints: Vec<UniqueConstraintSchema>,
        _sequences: Vec<SequenceSchema>,
        _table_type: StTableType,
        _table_access: StAccess,
    ) -> Self {
        unimplemented!()
        /*
        let row_type = ProductType::new(
            columns
                .iter()
                .map(|c| ProductTypeElement {
                    name: Some(c.col_name.clone()),
                    algebraic_type: c.col_type.clone(),
                })
                .collect(),
        );

        Self {
            table_id,
            table_name,
            columns,
            indexes,
            constraints,
            sequences,
            table_type,
            table_access,
            row_type,
        }
        */
    }

    pub fn into_columns(self) -> Vec<ColumnSchema> {
        self.columns
    }

    /// IMPORTANT: Ban changes from outside so [Self::row_type] won't get invalidated.
    pub fn columns(&self) -> &[ColumnSchema] {
        &self.columns
    }

    /// Clear all the [Self::indexes], [Self::sequences] & [Self::constraints]
    pub fn clear_adjacent_schemas(&mut self) {
        self.indexes.clear();
        self.sequences.clear();
        self.unique_constraints.clear();
    }

    // Crud operation on adjacent schemas

    /// Add OR replace the [SequenceSchema]
    pub fn update_sequence(&mut self, of: SequenceSchema) {
        if let Some(x) = self.sequences.iter_mut().find(|x| x.sequence_id == of.sequence_id) {
            *x = of;
        } else {
            self.sequences.push(of);
        }
    }

    /// Removes the given `sequence_id`
    pub fn remove_sequence(&mut self, sequence_id: SequenceId) {
        self.sequences.retain(|x| x.sequence_id != sequence_id)
    }

    /// Add OR replace the [IndexSchema]
    pub fn update_index(&mut self, of: IndexSchema) {
        if let Some(x) = self.indexes.iter_mut().find(|x| x.index_id == of.index_id) {
            *x = of;
        } else {
            self.indexes.push(of);
        }
    }

    /// Removes the given `index_id`
    pub fn remove_index(&mut self, index_id: IndexId) {
        self.indexes.retain(|x| x.index_id != index_id)
    }

    /*
    /// Add OR replace the [ConstraintSchema]
    pub fn update_constraint(&mut self, of: TableConstraintSchema) {
        if let Some(x) = self
            .constraints
            .iter_mut()
            .find(|x| x.constraint_id() == of.constraint_id())
        {
            *x = of;
        } else {
            self.constraints.push(of);
        }
    }

    /// Removes the given `index_id`
    pub fn remove_constraint(&mut self, constraint_id: ConstraintId) {
        self.constraints.retain(|x| x.constraint_id() != constraint_id)
    }
    */

    /// Check if the specified `field` exists in this [TableSchema].
    ///
    /// # Warning
    ///
    /// This function ignores the `table_id` when searching for a column.
    pub fn get_column_by_field(&self, field: FieldName) -> Option<&ColumnSchema> {
        self.get_column(field.col.idx())
    }

    pub fn get_columns(&self, columns: &ColList) -> Vec<(ColId, Option<&ColumnSchema>)> {
        columns.iter().map(|col| (col, self.columns.get(col.idx()))).collect()
    }

    /// Get a reference to a column by its position (`pos`) in the table.
    pub fn get_column(&self, pos: usize) -> Option<&ColumnSchema> {
        self.columns.get(pos)
    }

    /// Check if the `col_name` exist on this [TableSchema]
    ///
    /// Warning: It ignores the `table_name`
    pub fn get_column_by_name(&self, col_name: &str) -> Option<&ColumnSchema> {
        self.columns.iter().find(|x| &*x.col_name == col_name)
    }

    /// Project the fields from the supplied `indexes`.
    pub fn project(&self, indexes: impl Iterator<Item = ColId>) -> Result<Vec<&ColumnSchema>, InvalidFieldError> {
        indexes
            .map(|index| self.get_column(index.0 as usize).ok_or_else(|| index.into()))
            .collect()
    }

    /// Utility for project the fields from the supplied `indexes` that is a [ColList],
    /// used for when the list of field indexes have at least one value.
    pub fn project_not_empty(&self, indexes: ColList) -> Result<Vec<&ColumnSchema>, InvalidFieldError> {
        self.project(indexes.iter())
    }

    /// IMPORTANT: Is required to have this cached to avoid a perf drop on datastore operations
    pub fn get_row_type(&self) -> &ProductType {
        &self.row_type
    }

    /// Utility to avoid cloning in `row_type_for_table`
    pub fn into_row_type(self) -> ProductType {
        self.row_type
    }

    /// Create a new [TableSchema] from a [TableDef] and a `table_id`.
    ///
    /// # Parameters
    ///
    /// - `table_id`: The unique identifier for the table.
    /// - `schema`: The `TableDef` containing the schema information.
    pub fn from_def(_table_id: TableId, _schema: TableDef) -> Self {
        unimplemented!()

        /*
        TableSchema::new(
            table_id,
            schema.table_name.trim().into(),
            schema
                .columns
                .into_iter()
                .enumerate()
                .map(|(col_pos, x)| ColumnSchema::from_def(table_id, col_pos.into(), x))
                .collect(),
            schema
                .indexes
                .into_iter()
                .chain(indexes)
                .sorted_by_key(|x| x.columns.clone())
                .map(|x| IndexSchema::from_def(table_id, x))
                .collect(),
            schema
                .constraints
                .into_iter()
                .chain(constraints)
                .sorted()
                .map(|x| TableConstraintSchema::from_def(table_id, x))
                .collect(),
            schema
                .sequences
                .into_iter()
                .chain(sequences)
                .sorted_by_key(|x| x.col_pos)
                .map(|x| SequenceSchema::from_def(table_id, x))
                .collect(),
            schema.table_type,
            schema.table_access,
        )
        */
    }
}

impl From<&TableSchema> for ProductType {
    fn from(value: &TableSchema) -> Self {
        ProductType::new(
            value
                .columns
                .iter()
                .map(|c| ProductTypeElement {
                    name: Some(c.col_name.clone()),
                    algebraic_type: c.col_type.clone(),
                })
                .collect(),
        )
    }
}

impl From<&TableSchema> for DbTable {
    fn from(value: &TableSchema) -> Self {
        DbTable::new(
            Arc::new(value.into()),
            value.table_id,
            value.table_type,
            value.table_access,
        )
    }
}

impl From<&TableSchema> for Header {
    fn from(value: &TableSchema) -> Self {
        let fields = value
            .columns
            .iter()
            .map(|x| Column::new(FieldName::new(value.table_id, x.col_pos), x.col_type.clone()))
            .collect();

        let unique_constraints = value.unique_constraints.iter().map(|x| x.columns.clone()).collect();

        Header::new(value.table_id, value.table_name.clone(), fields, unique_constraints)
    }
}
