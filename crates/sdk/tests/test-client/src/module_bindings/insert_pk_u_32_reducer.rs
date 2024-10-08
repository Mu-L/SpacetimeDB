// THIS FILE IS AUTOMATICALLY GENERATED BY SPACETIMEDB. EDITS TO THIS FILE
// WILL NOT BE SAVED. MODIFY TABLES IN RUST INSTEAD.

#![allow(unused)]
use spacetimedb_sdk::{
    self as __sdk,
    anyhow::{self as __anyhow, Context as _},
    lib as __lib, sats as __sats, ws_messages as __ws,
};

#[derive(__lib::ser::Serialize, __lib::de::Deserialize, Clone, PartialEq, Debug)]
#[sats(crate = __lib)]
pub struct InsertPkU32 {
    pub n: u32,
    pub data: i32,
}

impl __sdk::spacetime_module::InModule for InsertPkU32 {
    type Module = super::RemoteModule;
}

pub struct InsertPkU32CallbackId(__sdk::callbacks::CallbackId);

#[allow(non_camel_case_types)]
/// Extension trait for access to the reducer `insert_pk_u32`.
///
/// Implemented for [`super::RemoteReducers`].
pub trait insert_pk_u_32 {
    /// Request that the remote module invoke the reducer `insert_pk_u32` to run as soon as possible.
    ///
    /// This method returns immediately, and errors only if we are unable to send the request.
    /// The reducer will run asynchronously in the future,
    ///  and its status can be observed by listening for [`Self::on_insert_pk_u_32`] callbacks.
    fn insert_pk_u_32(&self, n: u32, data: i32) -> __anyhow::Result<()>;
    /// Register a callback to run whenever we are notified of an invocation of the reducer `insert_pk_u32`.
    ///
    /// The [`super::EventContext`] passed to the `callback`
    /// will always have [`__sdk::Event::Reducer`] as its `event`,
    /// but it may or may not have terminated successfully and been committed.
    /// Callbacks should inspect the [`__sdk::ReducerEvent`] contained in the [`super::EventContext`]
    /// to determine the reducer's status.
    ///
    /// The returned [`InsertPkU32CallbackId`] can be passed to [`Self::remove_on_insert_pk_u_32`]
    /// to cancel the callback.
    fn on_insert_pk_u_32(
        &self,
        callback: impl FnMut(&super::EventContext, &u32, &i32) + Send + 'static,
    ) -> InsertPkU32CallbackId;
    /// Cancel a callback previously registered by [`Self::on_insert_pk_u_32`],
    /// causing it not to run in the future.
    fn remove_on_insert_pk_u_32(&self, callback: InsertPkU32CallbackId);
}

impl insert_pk_u_32 for super::RemoteReducers {
    fn insert_pk_u_32(&self, n: u32, data: i32) -> __anyhow::Result<()> {
        self.imp.call_reducer("insert_pk_u32", InsertPkU32 { n, data })
    }
    fn on_insert_pk_u_32(
        &self,
        mut callback: impl FnMut(&super::EventContext, &u32, &i32) + Send + 'static,
    ) -> InsertPkU32CallbackId {
        InsertPkU32CallbackId(self.imp.on_reducer::<InsertPkU32>(
            "insert_pk_u32",
            Box::new(move |ctx: &super::EventContext, args: &InsertPkU32| callback(ctx, &args.n, &args.data)),
        ))
    }
    fn remove_on_insert_pk_u_32(&self, callback: InsertPkU32CallbackId) {
        self.imp.remove_on_reducer::<InsertPkU32>("insert_pk_u32", callback.0)
    }
}

#[allow(non_camel_case_types)]
#[doc(hidden)]
/// Extension trait for setting the call-flags for the reducer `insert_pk_u32`.
///
/// Implemented for [`super::SetReducerFlags`].
///
/// This type is currently unstable and may be removed without a major version bump.
/// The ability to send over call-reducer flags in the protocol will however remain.
pub trait set_flags_for_insert_pk_u_32 {
    /// Set the call-reducer flags for the reducer `insert_pk_u32` to `flags`.
    ///
    /// This type is currently unstable and may be removed without a major version bump.
    /// The ability to send over call-reducer flags in the protocol will however remain.
    fn insert_pk_u_32(&self, flags: __ws::CallReducerFlags);
}

impl set_flags_for_insert_pk_u_32 for super::SetReducerFlags {
    fn insert_pk_u_32(&self, flags: __ws::CallReducerFlags) {
        self.imp.set_call_reducer_flags("insert_pk_u32", flags);
    }
}
