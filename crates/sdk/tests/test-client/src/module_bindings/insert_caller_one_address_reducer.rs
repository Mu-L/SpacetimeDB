// THIS FILE IS AUTOMATICALLY GENERATED BY SPACETIMEDB. EDITS TO THIS FILE
// WILL NOT BE SAVED. MODIFY TABLES IN RUST INSTEAD.

#![allow(unused)]
use spacetimedb_sdk::__codegen::{
    self as __sdk, __lib, __sats, __ws,
    anyhow::{self as __anyhow, Context as _},
};

#[derive(__lib::ser::Serialize, __lib::de::Deserialize, Clone, PartialEq, Debug)]
#[sats(crate = __lib)]
pub struct InsertCallerOneAddress {}

impl __sdk::InModule for InsertCallerOneAddress {
    type Module = super::RemoteModule;
}

pub struct InsertCallerOneAddressCallbackId(__sdk::CallbackId);

#[allow(non_camel_case_types)]
/// Extension trait for access to the reducer `insert_caller_one_address`.
///
/// Implemented for [`super::RemoteReducers`].
pub trait insert_caller_one_address {
    /// Request that the remote module invoke the reducer `insert_caller_one_address` to run as soon as possible.
    ///
    /// This method returns immediately, and errors only if we are unable to send the request.
    /// The reducer will run asynchronously in the future,
    ///  and its status can be observed by listening for [`Self::on_insert_caller_one_address`] callbacks.
    fn insert_caller_one_address(&self) -> __anyhow::Result<()>;
    /// Register a callback to run whenever we are notified of an invocation of the reducer `insert_caller_one_address`.
    ///
    /// The [`super::EventContext`] passed to the `callback`
    /// will always have [`__sdk::Event::Reducer`] as its `event`,
    /// but it may or may not have terminated successfully and been committed.
    /// Callbacks should inspect the [`__sdk::ReducerEvent`] contained in the [`super::EventContext`]
    /// to determine the reducer's status.
    ///
    /// The returned [`InsertCallerOneAddressCallbackId`] can be passed to [`Self::remove_on_insert_caller_one_address`]
    /// to cancel the callback.
    fn on_insert_caller_one_address(
        &self,
        callback: impl FnMut(&super::EventContext) + Send + 'static,
    ) -> InsertCallerOneAddressCallbackId;
    /// Cancel a callback previously registered by [`Self::on_insert_caller_one_address`],
    /// causing it not to run in the future.
    fn remove_on_insert_caller_one_address(&self, callback: InsertCallerOneAddressCallbackId);
}

impl insert_caller_one_address for super::RemoteReducers {
    fn insert_caller_one_address(&self) -> __anyhow::Result<()> {
        self.imp
            .call_reducer("insert_caller_one_address", InsertCallerOneAddress {})
    }
    fn on_insert_caller_one_address(
        &self,
        mut callback: impl FnMut(&super::EventContext) + Send + 'static,
    ) -> InsertCallerOneAddressCallbackId {
        InsertCallerOneAddressCallbackId(self.imp.on_reducer::<InsertCallerOneAddress>(
            "insert_caller_one_address",
            Box::new(move |ctx: &super::EventContext, args: &InsertCallerOneAddress| callback(ctx)),
        ))
    }
    fn remove_on_insert_caller_one_address(&self, callback: InsertCallerOneAddressCallbackId) {
        self.imp
            .remove_on_reducer::<InsertCallerOneAddress>("insert_caller_one_address", callback.0)
    }
}
