// THIS FILE IS AUTOMATICALLY GENERATED BY SPACETIMEDB. EDITS TO THIS FILE
// WILL NOT BE SAVED. MODIFY TABLES IN RUST INSTEAD.

#[allow(unused)]
use spacetimedb_sdk::{
    anyhow::{anyhow, Result},
    identity::Identity,
    reducer::{Reducer, ReducerCallbackId, Status},
    sats::{de::Deserialize, ser::Serialize},
    spacetimedb_lib,
    table::{TableIter, TableType, TableWithPrimaryKey},
    Address,
};

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct OneI8 {
    pub n: i8,
}

impl TableType for OneI8 {
    const TABLE_NAME: &'static str = "OneI8";
    type ReducerEvent = super::ReducerEvent;
}

impl OneI8 {
    #[allow(unused)]
    pub fn filter_by_n(n: i8) -> TableIter<Self> {
        Self::filter(|row| row.n == n)
    }
}
