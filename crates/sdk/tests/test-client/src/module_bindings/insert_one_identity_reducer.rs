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
pub struct InsertOneIdentityArgs {
    pub i: Identity,
}

impl Reducer for InsertOneIdentityArgs {
    const REDUCER_NAME: &'static str = "insert_one_identity";
}

#[allow(unused)]
pub fn insert_one_identity(i: Identity) {
    InsertOneIdentityArgs { i }.invoke();
}

#[allow(unused)]
pub fn on_insert_one_identity(
    mut __callback: impl FnMut(&Identity, Option<Address>, &Status, &Identity) + Send + 'static,
) -> ReducerCallbackId<InsertOneIdentityArgs> {
    InsertOneIdentityArgs::on_reducer(move |__identity, __addr, __status, __args| {
        let InsertOneIdentityArgs { i } = __args;
        __callback(__identity, __addr, __status, i);
    })
}

#[allow(unused)]
pub fn once_on_insert_one_identity(
    __callback: impl FnOnce(&Identity, Option<Address>, &Status, &Identity) + Send + 'static,
) -> ReducerCallbackId<InsertOneIdentityArgs> {
    InsertOneIdentityArgs::once_on_reducer(move |__identity, __addr, __status, __args| {
        let InsertOneIdentityArgs { i } = __args;
        __callback(__identity, __addr, __status, i);
    })
}

#[allow(unused)]
pub fn remove_on_insert_one_identity(id: ReducerCallbackId<InsertOneIdentityArgs>) {
    InsertOneIdentityArgs::remove_on_reducer(id);
}
