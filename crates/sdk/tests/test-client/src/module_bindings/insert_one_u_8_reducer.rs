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
pub struct InsertOneU8 {
    pub n: u8,
}

impl __sdk::spacetime_module::InModule for InsertOneU8 {
    type Module = super::RemoteModule;
}

pub struct InsertOneU8CallbackId(__sdk::callbacks::CallbackId);

#[allow(non_camel_case_types)]
/// Extension trait for access to the reducer `insert_one_u8`.
///
/// Implemented for [`super::RemoteReducers`].
pub trait insert_one_u_8 {
    /// Request that the remote module invoke the reducer `insert_one_u8` to run as soon as possible.
    ///
    /// This method returns immediately, and errors only if we are unable to send the request.
    /// The reducer will run asynchronously in the future,
    ///  and its status can be observed by listening for [`Self::on_insert_one_u_8`] callbacks.
    fn insert_one_u_8(&self, n: u8) -> __anyhow::Result<()>;
    /// Register a callback to run whenever we are notified of an invocation of the reducer `insert_one_u8`.
    ///
    /// The [`super::EventContext`] passed to the `callback`
    /// will always have [`__sdk::Event::Reducer`] as its `event`,
    /// but it may or may not have terminated successfully and been committed.
    /// Callbacks should inspect the [`__sdk::ReducerEvent`] contained in the [`super::EventContext`]
    /// to determine the reducer's status.
    ///
    /// The returned [`InsertOneU8CallbackId`] can be passed to [`Self::remove_on_insert_one_u_8`]
    /// to cancel the callback.
    fn on_insert_one_u_8(
        &self,
        callback: impl FnMut(&super::EventContext, &u8) + Send + 'static,
    ) -> InsertOneU8CallbackId;
    /// Cancel a callback previously registered by [`Self::on_insert_one_u_8`],
    /// causing it not to run in the future.
    fn remove_on_insert_one_u_8(&self, callback: InsertOneU8CallbackId);
}

impl insert_one_u_8 for super::RemoteReducers {
    fn insert_one_u_8(&self, n: u8) -> __anyhow::Result<()> {
        self.imp.call_reducer("insert_one_u8", InsertOneU8 { n })
    }
    fn on_insert_one_u_8(
        &self,
        mut callback: impl FnMut(&super::EventContext, &u8) + Send + 'static,
    ) -> InsertOneU8CallbackId {
        InsertOneU8CallbackId(self.imp.on_reducer::<InsertOneU8>(
            "insert_one_u8",
            Box::new(move |ctx: &super::EventContext, args: &InsertOneU8| callback(ctx, &args.n)),
        ))
    }
    fn remove_on_insert_one_u_8(&self, callback: InsertOneU8CallbackId) {
        self.imp.remove_on_reducer::<InsertOneU8>("insert_one_u8", callback.0)
    }
}

#[allow(non_camel_case_types)]
#[doc(hidden)]
/// Extension trait for setting the call-flags for the reducer `insert_one_u8`.
///
/// Implemented for [`super::SetReducerFlags`].
///
/// This type is currently unstable and may be removed without a major version bump.
/// The ability to send over call-reducer flags in the protocol will however remain.
pub trait set_flags_for_insert_one_u_8 {
    /// Set the call-reducer flags for the reducer `insert_one_u8` to `flags`.
    ///
    /// This type is currently unstable and may be removed without a major version bump.
    /// The ability to send over call-reducer flags in the protocol will however remain.
    fn insert_one_u_8(&self, flags: __ws::CallReducerFlags);
}

impl set_flags_for_insert_one_u_8 for super::SetReducerFlags {
    fn insert_one_u_8(&self, flags: __ws::CallReducerFlags) {
        self.imp.set_call_reducer_flags("insert_one_u8", flags);
    }
}
