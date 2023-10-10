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
pub struct InsertUniqueU8Args {
    pub n: u8,
    pub data: i32,
}

impl Reducer for InsertUniqueU8Args {
    const REDUCER_NAME: &'static str = "insert_unique_u8";
}

#[allow(unused)]
pub fn insert_unique_u_8(n: u8, data: i32) {
    InsertUniqueU8Args { n, data }.invoke();
}

#[allow(unused)]
pub fn on_insert_unique_u_8(
    mut __callback: impl FnMut(&Identity, Option<Address>, &Status, &u8, &i32) + Send + 'static,
) -> ReducerCallbackId<InsertUniqueU8Args> {
    InsertUniqueU8Args::on_reducer(move |__identity, __addr, __status, __args| {
        let InsertUniqueU8Args { n, data } = __args;
        __callback(__identity, __addr, __status, n, data);
    })
}

#[allow(unused)]
pub fn once_on_insert_unique_u_8(
    __callback: impl FnOnce(&Identity, Option<Address>, &Status, &u8, &i32) + Send + 'static,
) -> ReducerCallbackId<InsertUniqueU8Args> {
    InsertUniqueU8Args::once_on_reducer(move |__identity, __addr, __status, __args| {
        let InsertUniqueU8Args { n, data } = __args;
        __callback(__identity, __addr, __status, n, data);
    })
}

#[allow(unused)]
pub fn remove_on_insert_unique_u_8(id: ReducerCallbackId<InsertUniqueU8Args>) {
    InsertUniqueU8Args::remove_on_reducer(id);
}
