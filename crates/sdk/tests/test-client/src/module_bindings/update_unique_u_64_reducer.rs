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
pub struct UpdateUniqueU64 {
    pub n: u64,
    pub data: i32,
}

impl __sdk::spacetime_module::InModule for UpdateUniqueU64 {
    type Module = super::RemoteModule;
}

pub struct UpdateUniqueU64CallbackId(__sdk::callbacks::CallbackId);

#[allow(non_camel_case_types)]
/// Extension trait for access to the reducer `update_unique_u64`.
///
/// Implemented for [`super::RemoteReducers`].
pub trait update_unique_u_64 {
    /// Request that the remote module invoke the reducer `update_unique_u64` to run as soon as possible.
    ///
    /// This method returns immediately, and errors only if we are unable to send the request.
    /// The reducer will run asynchronously in the future,
    ///  and its status can be observed by listening for [`Self::on_update_unique_u_64`] callbacks.
    fn update_unique_u_64(&self, n: u64, data: i32) -> __anyhow::Result<()>;
    /// Register a callback to run whenever we are notified of an invocation of the reducer `update_unique_u64`.
    ///
    /// The [`super::EventContext`] passed to the `callback`
    /// will always have [`__sdk::Event::Reducer`] as its `event`,
    /// but it may or may not have terminated successfully and been committed.
    /// Callbacks should inspect the [`__sdk::ReducerEvent`] contained in the [`super::EventContext`]
    /// to determine the reducer's status.
    ///
    /// The returned [`UpdateUniqueU64CallbackId`] can be passed to [`Self::remove_on_update_unique_u_64`]
    /// to cancel the callback.
    fn on_update_unique_u_64(
        &self,
        callback: impl FnMut(&super::EventContext, &u64, &i32) + Send + 'static,
    ) -> UpdateUniqueU64CallbackId;
    /// Cancel a callback previously registered by [`Self::on_update_unique_u_64`],
    /// causing it not to run in the future.
    fn remove_on_update_unique_u_64(&self, callback: UpdateUniqueU64CallbackId);
}

impl update_unique_u_64 for super::RemoteReducers {
    fn update_unique_u_64(&self, n: u64, data: i32) -> __anyhow::Result<()> {
        self.imp.call_reducer("update_unique_u64", UpdateUniqueU64 { n, data })
    }
    fn on_update_unique_u_64(
        &self,
        mut callback: impl FnMut(&super::EventContext, &u64, &i32) + Send + 'static,
    ) -> UpdateUniqueU64CallbackId {
        UpdateUniqueU64CallbackId(self.imp.on_reducer::<UpdateUniqueU64>(
            "update_unique_u64",
            Box::new(move |ctx: &super::EventContext, args: &UpdateUniqueU64| callback(ctx, &args.n, &args.data)),
        ))
    }
    fn remove_on_update_unique_u_64(&self, callback: UpdateUniqueU64CallbackId) {
        self.imp
            .remove_on_reducer::<UpdateUniqueU64>("update_unique_u64", callback.0)
    }
}

#[allow(non_camel_case_types)]
#[doc(hidden)]
/// Extension trait for setting the call-flags for the reducer `update_unique_u64`.
///
/// Implemented for [`super::SetReducerFlags`].
///
/// This type is currently unstable and may be removed without a major version bump.
pub trait set_flags_for_update_unique_u_64 {
    /// Set the call-reducer flags for the reducer `update_unique_u64` to `flags`.
    ///
    /// This type is currently unstable and may be removed without a major version bump.
    fn update_unique_u_64(&self, flags: __ws::CallReducerFlags);
}

impl set_flags_for_update_unique_u_64 for super::SetReducerFlags {
    fn update_unique_u_64(&self, flags: __ws::CallReducerFlags) {
        self.imp.set_call_reducer_flags("update_unique_u64", flags);
    }
}
