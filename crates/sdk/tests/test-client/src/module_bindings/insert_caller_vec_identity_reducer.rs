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
pub struct InsertCallerVecIdentity {}

impl __sdk::spacetime_module::InModule for InsertCallerVecIdentity {
    type Module = super::RemoteModule;
}

pub struct InsertCallerVecIdentityCallbackId(__sdk::callbacks::CallbackId);

#[allow(non_camel_case_types)]
/// Extension trait for access to the reducer `insert_caller_vec_identity`.
///
/// Implemented for [`super::RemoteReducers`].
pub trait insert_caller_vec_identity {
    /// Request that the remote module invoke the reducer `insert_caller_vec_identity` to run as soon as possible.
    ///
    /// This method returns immediately, and errors only if we are unable to send the request.
    /// The reducer will run asynchronously in the future,
    ///  and its status can be observed by listening for [`Self::on_insert_caller_vec_identity`] callbacks.
    fn insert_caller_vec_identity(&self) -> __anyhow::Result<()>;
    /// Register a callback to run whenever we are notified of an invocation of the reducer `insert_caller_vec_identity`.
    ///
    /// The [`super::EventContext`] passed to the `callback`
    /// will always have [`__sdk::Event::Reducer`] as its `event`,
    /// but it may or may not have terminated successfully and been committed.
    /// Callbacks should inspect the [`__sdk::ReducerEvent`] contained in the [`super::EventContext`]
    /// to determine the reducer's status.
    ///
    /// The returned [`InsertCallerVecIdentityCallbackId`] can be passed to [`Self::remove_on_insert_caller_vec_identity`]
    /// to cancel the callback.
    fn on_insert_caller_vec_identity(
        &self,
        callback: impl FnMut(&super::EventContext) + Send + 'static,
    ) -> InsertCallerVecIdentityCallbackId;
    /// Cancel a callback previously registered by [`Self::on_insert_caller_vec_identity`],
    /// causing it not to run in the future.
    fn remove_on_insert_caller_vec_identity(&self, callback: InsertCallerVecIdentityCallbackId);
}

impl insert_caller_vec_identity for super::RemoteReducers {
    fn insert_caller_vec_identity(&self) -> __anyhow::Result<()> {
        self.imp
            .call_reducer("insert_caller_vec_identity", InsertCallerVecIdentity {})
    }
    fn on_insert_caller_vec_identity(
        &self,
        mut callback: impl FnMut(&super::EventContext) + Send + 'static,
    ) -> InsertCallerVecIdentityCallbackId {
        InsertCallerVecIdentityCallbackId(self.imp.on_reducer::<InsertCallerVecIdentity>(
            "insert_caller_vec_identity",
            Box::new(move |ctx: &super::EventContext, args: &InsertCallerVecIdentity| callback(ctx)),
        ))
    }
    fn remove_on_insert_caller_vec_identity(&self, callback: InsertCallerVecIdentityCallbackId) {
        self.imp
            .remove_on_reducer::<InsertCallerVecIdentity>("insert_caller_vec_identity", callback.0)
    }
}

#[allow(non_camel_case_types)]
#[doc(hidden)]
/// Extension trait for setting the call-flags for the reducer `insert_caller_vec_identity`.
///
/// Implemented for [`super::SetReducerFlags`].
///
/// This type is currently unstable and may be removed without a major version bump.
pub trait set_flags_for_insert_caller_vec_identity {
    /// Set the call-reducer flags for the reducer `insert_caller_vec_identity` to `flags`.
    ///
    /// This type is currently unstable and may be removed without a major version bump.
    fn insert_caller_vec_identity(&self, flags: __ws::CallReducerFlags);
}

impl set_flags_for_insert_caller_vec_identity for super::SetReducerFlags {
    fn insert_caller_vec_identity(&self, flags: __ws::CallReducerFlags) {
        self.imp.set_call_reducer_flags("insert_caller_vec_identity", flags);
    }
}
