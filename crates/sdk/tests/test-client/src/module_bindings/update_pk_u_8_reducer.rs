// THIS FILE IS AUTOMATICALLY GENERATED BY SPACETIMEDB. EDITS TO THIS FILE
// WILL NOT BE SAVED. MODIFY TABLES IN RUST INSTEAD.

#![allow(unused)]
use spacetimedb_sdk::__codegen::{
    self as __sdk, __lib, __sats, __ws,
    anyhow::{self as __anyhow, Context as _},
};

#[derive(__lib::ser::Serialize, __lib::de::Deserialize, Clone, PartialEq, Debug)]
#[sats(crate = __lib)]
pub struct UpdatePkU8 {
    pub n: u8,
    pub data: i32,
}

impl __sdk::InModule for UpdatePkU8 {
    type Module = super::RemoteModule;
}

pub struct UpdatePkU8CallbackId(__sdk::CallbackId);

#[allow(non_camel_case_types)]
/// Extension trait for access to the reducer `update_pk_u8`.
///
/// Implemented for [`super::RemoteReducers`].
pub trait update_pk_u_8 {
    /// Request that the remote module invoke the reducer `update_pk_u8` to run as soon as possible.
    ///
    /// This method returns immediately, and errors only if we are unable to send the request.
    /// The reducer will run asynchronously in the future,
    ///  and its status can be observed by listening for [`Self::on_update_pk_u_8`] callbacks.
    fn update_pk_u_8(&self, n: u8, data: i32) -> __anyhow::Result<()>;
    /// Register a callback to run whenever we are notified of an invocation of the reducer `update_pk_u8`.
    ///
    /// The [`super::EventContext`] passed to the `callback`
    /// will always have [`__sdk::Event::Reducer`] as its `event`,
    /// but it may or may not have terminated successfully and been committed.
    /// Callbacks should inspect the [`__sdk::ReducerEvent`] contained in the [`super::EventContext`]
    /// to determine the reducer's status.
    ///
    /// The returned [`UpdatePkU8CallbackId`] can be passed to [`Self::remove_on_update_pk_u_8`]
    /// to cancel the callback.
    fn on_update_pk_u_8(
        &self,
        callback: impl FnMut(&super::EventContext, &u8, &i32) + Send + 'static,
    ) -> UpdatePkU8CallbackId;
    /// Cancel a callback previously registered by [`Self::on_update_pk_u_8`],
    /// causing it not to run in the future.
    fn remove_on_update_pk_u_8(&self, callback: UpdatePkU8CallbackId);
}

impl update_pk_u_8 for super::RemoteReducers {
    fn update_pk_u_8(&self, n: u8, data: i32) -> __anyhow::Result<()> {
        self.imp.call_reducer("update_pk_u8", UpdatePkU8 { n, data })
    }
    fn on_update_pk_u_8(
        &self,
        mut callback: impl FnMut(&super::EventContext, &u8, &i32) + Send + 'static,
    ) -> UpdatePkU8CallbackId {
        UpdatePkU8CallbackId(self.imp.on_reducer::<UpdatePkU8>(
            "update_pk_u8",
            Box::new(move |ctx: &super::EventContext, args: &UpdatePkU8| callback(ctx, &args.n, &args.data)),
        ))
    }
    fn remove_on_update_pk_u_8(&self, callback: UpdatePkU8CallbackId) {
        self.imp.remove_on_reducer::<UpdatePkU8>("update_pk_u8", callback.0)
    }
}
