// THIS FILE IS AUTOMATICALLY GENERATED BY SPACETIMEDB. EDITS TO THIS FILE
// WILL NOT BE SAVED. MODIFY TABLES IN RUST INSTEAD.

use super::byte_struct::ByteStruct;
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
pub struct VecByteStruct {
    pub s: Vec<ByteStruct>,
}

impl TableType for VecByteStruct {
    const TABLE_NAME: &'static str = "VecByteStruct";
    type ReducerEvent = super::ReducerEvent;
}

impl VecByteStruct {
    #[allow(unused)]
    pub fn filter_by_s(s: Vec<ByteStruct>) -> TableIter<Self> {
        Self::filter(|row| row.s == s)
    }
}
