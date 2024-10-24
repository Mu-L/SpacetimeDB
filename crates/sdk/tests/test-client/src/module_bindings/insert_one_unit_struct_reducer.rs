// THIS FILE IS AUTOMATICALLY GENERATED BY SPACETIMEDB. EDITS TO THIS FILE
// WILL NOT BE SAVED. MODIFY TABLES IN RUST INSTEAD.

#![allow(unused)]
use spacetimedb_sdk::__codegen::{
    self as __sdk, __lib, __sats, __ws,
    anyhow::{self as __anyhow, Context as _},
};

use super::unit_struct_type::UnitStruct;

#[derive(__lib::ser::Serialize, __lib::de::Deserialize, Clone, PartialEq, Debug)]
#[sats(crate = __lib)]
pub struct InsertOneUnitStruct {
    pub s: UnitStruct,
}

impl __sdk::InModule for InsertOneUnitStruct {
    type Module = super::RemoteModule;
}

pub struct InsertOneUnitStructCallbackId(__sdk::CallbackId);

#[allow(non_camel_case_types)]
/// Extension trait for access to the reducer `insert_one_unit_struct`.
///
/// Implemented for [`super::RemoteReducers`].
pub trait insert_one_unit_struct {
    /// Request that the remote module invoke the reducer `insert_one_unit_struct` to run as soon as possible.
    ///
    /// This method returns immediately, and errors only if we are unable to send the request.
    /// The reducer will run asynchronously in the future,
    ///  and its status can be observed by listening for [`Self::on_insert_one_unit_struct`] callbacks.
    fn insert_one_unit_struct(&self, s: UnitStruct) -> __anyhow::Result<()>;
    /// Register a callback to run whenever we are notified of an invocation of the reducer `insert_one_unit_struct`.
    ///
    /// The [`super::EventContext`] passed to the `callback`
    /// will always have [`__sdk::Event::Reducer`] as its `event`,
    /// but it may or may not have terminated successfully and been committed.
    /// Callbacks should inspect the [`__sdk::ReducerEvent`] contained in the [`super::EventContext`]
    /// to determine the reducer's status.
    ///
    /// The returned [`InsertOneUnitStructCallbackId`] can be passed to [`Self::remove_on_insert_one_unit_struct`]
    /// to cancel the callback.
    fn on_insert_one_unit_struct(
        &self,
        callback: impl FnMut(&super::EventContext, &UnitStruct) + Send + 'static,
    ) -> InsertOneUnitStructCallbackId;
    /// Cancel a callback previously registered by [`Self::on_insert_one_unit_struct`],
    /// causing it not to run in the future.
    fn remove_on_insert_one_unit_struct(&self, callback: InsertOneUnitStructCallbackId);
}

impl insert_one_unit_struct for super::RemoteReducers {
    fn insert_one_unit_struct(&self, s: UnitStruct) -> __anyhow::Result<()> {
        self.imp
            .call_reducer("insert_one_unit_struct", InsertOneUnitStruct { s })
    }
    fn on_insert_one_unit_struct(
        &self,
        mut callback: impl FnMut(&super::EventContext, &UnitStruct) + Send + 'static,
    ) -> InsertOneUnitStructCallbackId {
        InsertOneUnitStructCallbackId(self.imp.on_reducer::<InsertOneUnitStruct>(
            "insert_one_unit_struct",
            Box::new(move |ctx: &super::EventContext, args: &InsertOneUnitStruct| callback(ctx, &args.s)),
        ))
    }
    fn remove_on_insert_one_unit_struct(&self, callback: InsertOneUnitStructCallbackId) {
        self.imp
            .remove_on_reducer::<InsertOneUnitStruct>("insert_one_unit_struct", callback.0)
    }
}
