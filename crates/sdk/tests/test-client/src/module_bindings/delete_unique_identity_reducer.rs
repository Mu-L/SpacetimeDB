// THIS FILE IS AUTOMATICALLY GENERATED BY SPACETIMEDB. EDITS TO THIS FILE
// WILL NOT BE SAVED. MODIFY TABLES IN RUST INSTEAD.

#![allow(unused)]
use spacetimedb_sdk::__codegen::{
    self as __sdk, __lib, __sats, __ws,
    anyhow::{self as __anyhow, Context as _},
};

#[derive(__lib::ser::Serialize, __lib::de::Deserialize, Clone, PartialEq, Debug)]
#[sats(crate = __lib)]
pub struct DeleteUniqueIdentity {
    pub i: __sdk::Identity,
}

impl __sdk::InModule for DeleteUniqueIdentity {
    type Module = super::RemoteModule;
}

pub struct DeleteUniqueIdentityCallbackId(__sdk::CallbackId);

#[allow(non_camel_case_types)]
/// Extension trait for access to the reducer `delete_unique_identity`.
///
/// Implemented for [`super::RemoteReducers`].
pub trait delete_unique_identity {
    /// Request that the remote module invoke the reducer `delete_unique_identity` to run as soon as possible.
    ///
    /// This method returns immediately, and errors only if we are unable to send the request.
    /// The reducer will run asynchronously in the future,
    ///  and its status can be observed by listening for [`Self::on_delete_unique_identity`] callbacks.
    fn delete_unique_identity(&self, i: __sdk::Identity) -> __anyhow::Result<()>;
    /// Register a callback to run whenever we are notified of an invocation of the reducer `delete_unique_identity`.
    ///
    /// The [`super::EventContext`] passed to the `callback`
    /// will always have [`__sdk::Event::Reducer`] as its `event`,
    /// but it may or may not have terminated successfully and been committed.
    /// Callbacks should inspect the [`__sdk::ReducerEvent`] contained in the [`super::EventContext`]
    /// to determine the reducer's status.
    ///
    /// The returned [`DeleteUniqueIdentityCallbackId`] can be passed to [`Self::remove_on_delete_unique_identity`]
    /// to cancel the callback.
    fn on_delete_unique_identity(
        &self,
        callback: impl FnMut(&super::EventContext, &__sdk::Identity) + Send + 'static,
    ) -> DeleteUniqueIdentityCallbackId;
    /// Cancel a callback previously registered by [`Self::on_delete_unique_identity`],
    /// causing it not to run in the future.
    fn remove_on_delete_unique_identity(&self, callback: DeleteUniqueIdentityCallbackId);
}

impl delete_unique_identity for super::RemoteReducers {
    fn delete_unique_identity(&self, i: __sdk::Identity) -> __anyhow::Result<()> {
        self.imp
            .call_reducer("delete_unique_identity", DeleteUniqueIdentity { i })
    }
    fn on_delete_unique_identity(
        &self,
        mut callback: impl FnMut(&super::EventContext, &__sdk::Identity) + Send + 'static,
    ) -> DeleteUniqueIdentityCallbackId {
        DeleteUniqueIdentityCallbackId(self.imp.on_reducer::<DeleteUniqueIdentity>(
            "delete_unique_identity",
            Box::new(move |ctx: &super::EventContext, args: &DeleteUniqueIdentity| callback(ctx, &args.i)),
        ))
    }
    fn remove_on_delete_unique_identity(&self, callback: DeleteUniqueIdentityCallbackId) {
        self.imp
            .remove_on_reducer::<DeleteUniqueIdentity>("delete_unique_identity", callback.0)
    }
}
