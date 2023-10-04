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
pub struct InsertOneI128Args {
    pub n: i128,
}

impl Reducer for InsertOneI128Args {
    const REDUCER_NAME: &'static str = "insert_one_i128";
}

#[allow(unused)]
pub fn insert_one_i_128(n: i128) {
    InsertOneI128Args { n }.invoke();
}

#[allow(unused)]
pub fn on_insert_one_i_128(
    mut __callback: impl FnMut(&Identity, Option<Address>, &Status, &i128) + Send + 'static,
) -> ReducerCallbackId<InsertOneI128Args> {
    InsertOneI128Args::on_reducer(move |__identity, __addr, __status, __args| {
        let InsertOneI128Args { n } = __args;
        __callback(__identity, __addr, __status, n);
    })
}

#[allow(unused)]
pub fn once_on_insert_one_i_128(
    __callback: impl FnOnce(&Identity, Option<Address>, &Status, &i128) + Send + 'static,
) -> ReducerCallbackId<InsertOneI128Args> {
    InsertOneI128Args::once_on_reducer(move |__identity, __addr, __status, __args| {
        let InsertOneI128Args { n } = __args;
        __callback(__identity, __addr, __status, n);
    })
}

#[allow(unused)]
pub fn remove_on_insert_one_i_128(id: ReducerCallbackId<InsertOneI128Args>) {
    InsertOneI128Args::remove_on_reducer(id);
}
