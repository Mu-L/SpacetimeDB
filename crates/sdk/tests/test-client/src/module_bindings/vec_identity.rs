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
pub struct VecIdentity {
    pub i: Vec<Identity>,
}

impl TableType for VecIdentity {
    const TABLE_NAME: &'static str = "VecIdentity";
    type ReducerEvent = super::ReducerEvent;
}

impl VecIdentity {
    #[allow(unused)]
    pub fn filter_by_i(i: Vec<Identity>) -> TableIter<Self> {
        Self::filter(|row| row.i == i)
    }
}
