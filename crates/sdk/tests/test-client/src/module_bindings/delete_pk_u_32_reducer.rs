// THIS FILE IS AUTOMATICALLY GENERATED BY SPACETIMEDB. EDITS TO THIS FILE
// WILL NOT BE SAVED. MODIFY TABLES IN RUST INSTEAD.

#![allow(unused)]
use spacetimedb_sdk::__codegen::{
    self as __sdk, __lib, __sats, __ws,
    anyhow::{self as __anyhow, Context as _},
};

#[derive(__lib::ser::Serialize, __lib::de::Deserialize, Clone, PartialEq, Debug)]
#[sats(crate = __lib)]
pub struct DeletePkU32 {
    pub n: u32,
}

impl __sdk::InModule for DeletePkU32 {
    type Module = super::RemoteModule;
}

pub struct DeletePkU32CallbackId(__sdk::CallbackId);

#[allow(non_camel_case_types)]
/// Extension trait for access to the reducer `delete_pk_u32`.
///
/// Implemented for [`super::RemoteReducers`].
pub trait delete_pk_u_32 {
    /// Request that the remote module invoke the reducer `delete_pk_u32` to run as soon as possible.
    ///
    /// This method returns immediately, and errors only if we are unable to send the request.
    /// The reducer will run asynchronously in the future,
    ///  and its status can be observed by listening for [`Self::on_delete_pk_u_32`] callbacks.
    fn delete_pk_u_32(&self, n: u32) -> __anyhow::Result<()>;
    /// Register a callback to run whenever we are notified of an invocation of the reducer `delete_pk_u32`.
    ///
    /// The [`super::EventContext`] passed to the `callback`
    /// will always have [`__sdk::Event::Reducer`] as its `event`,
    /// but it may or may not have terminated successfully and been committed.
    /// Callbacks should inspect the [`__sdk::ReducerEvent`] contained in the [`super::EventContext`]
    /// to determine the reducer's status.
    ///
    /// The returned [`DeletePkU32CallbackId`] can be passed to [`Self::remove_on_delete_pk_u_32`]
    /// to cancel the callback.
    fn on_delete_pk_u_32(
        &self,
        callback: impl FnMut(&super::EventContext, &u32) + Send + 'static,
    ) -> DeletePkU32CallbackId;
    /// Cancel a callback previously registered by [`Self::on_delete_pk_u_32`],
    /// causing it not to run in the future.
    fn remove_on_delete_pk_u_32(&self, callback: DeletePkU32CallbackId);
}

impl delete_pk_u_32 for super::RemoteReducers {
    fn delete_pk_u_32(&self, n: u32) -> __anyhow::Result<()> {
        self.imp.call_reducer("delete_pk_u32", DeletePkU32 { n })
    }
    fn on_delete_pk_u_32(
        &self,
        mut callback: impl FnMut(&super::EventContext, &u32) + Send + 'static,
    ) -> DeletePkU32CallbackId {
        DeletePkU32CallbackId(self.imp.on_reducer::<DeletePkU32>(
            "delete_pk_u32",
            Box::new(move |ctx: &super::EventContext, args: &DeletePkU32| callback(ctx, &args.n)),
        ))
    }
    fn remove_on_delete_pk_u_32(&self, callback: DeletePkU32CallbackId) {
        self.imp.remove_on_reducer::<DeletePkU32>("delete_pk_u32", callback.0)
    }
}
