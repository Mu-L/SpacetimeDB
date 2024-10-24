// THIS FILE IS AUTOMATICALLY GENERATED BY SPACETIMEDB. EDITS TO THIS FILE
// WILL NOT BE SAVED. MODIFY TABLES IN RUST INSTEAD.

#![allow(unused)]
use spacetimedb_sdk::__codegen::{
    self as __sdk, __lib, __sats, __ws,
    anyhow::{self as __anyhow, Context as _},
};

#[derive(__lib::ser::Serialize, __lib::de::Deserialize, Clone, PartialEq, Debug)]
#[sats(crate = __lib)]
pub struct InsertOptionString {
    pub s: Option<String>,
}

impl __sdk::InModule for InsertOptionString {
    type Module = super::RemoteModule;
}

pub struct InsertOptionStringCallbackId(__sdk::CallbackId);

#[allow(non_camel_case_types)]
/// Extension trait for access to the reducer `insert_option_string`.
///
/// Implemented for [`super::RemoteReducers`].
pub trait insert_option_string {
    /// Request that the remote module invoke the reducer `insert_option_string` to run as soon as possible.
    ///
    /// This method returns immediately, and errors only if we are unable to send the request.
    /// The reducer will run asynchronously in the future,
    ///  and its status can be observed by listening for [`Self::on_insert_option_string`] callbacks.
    fn insert_option_string(&self, s: Option<String>) -> __anyhow::Result<()>;
    /// Register a callback to run whenever we are notified of an invocation of the reducer `insert_option_string`.
    ///
    /// The [`super::EventContext`] passed to the `callback`
    /// will always have [`__sdk::Event::Reducer`] as its `event`,
    /// but it may or may not have terminated successfully and been committed.
    /// Callbacks should inspect the [`__sdk::ReducerEvent`] contained in the [`super::EventContext`]
    /// to determine the reducer's status.
    ///
    /// The returned [`InsertOptionStringCallbackId`] can be passed to [`Self::remove_on_insert_option_string`]
    /// to cancel the callback.
    fn on_insert_option_string(
        &self,
        callback: impl FnMut(&super::EventContext, &Option<String>) + Send + 'static,
    ) -> InsertOptionStringCallbackId;
    /// Cancel a callback previously registered by [`Self::on_insert_option_string`],
    /// causing it not to run in the future.
    fn remove_on_insert_option_string(&self, callback: InsertOptionStringCallbackId);
}

impl insert_option_string for super::RemoteReducers {
    fn insert_option_string(&self, s: Option<String>) -> __anyhow::Result<()> {
        self.imp.call_reducer("insert_option_string", InsertOptionString { s })
    }
    fn on_insert_option_string(
        &self,
        mut callback: impl FnMut(&super::EventContext, &Option<String>) + Send + 'static,
    ) -> InsertOptionStringCallbackId {
        InsertOptionStringCallbackId(self.imp.on_reducer::<InsertOptionString>(
            "insert_option_string",
            Box::new(move |ctx: &super::EventContext, args: &InsertOptionString| callback(ctx, &args.s)),
        ))
    }
    fn remove_on_insert_option_string(&self, callback: InsertOptionStringCallbackId) {
        self.imp
            .remove_on_reducer::<InsertOptionString>("insert_option_string", callback.0)
    }
}
