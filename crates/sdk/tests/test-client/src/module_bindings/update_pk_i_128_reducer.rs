// THIS FILE IS AUTOMATICALLY GENERATED BY SPACETIMEDB. EDITS TO THIS FILE
// WILL NOT BE SAVED. MODIFY TABLES IN RUST INSTEAD.

#![allow(unused, clippy::all)]
use spacetimedb_sdk::__codegen::{
    self as __sdk, __lib, __sats, __ws,
    anyhow::{self as __anyhow, Context as _},
};

#[derive(__lib::ser::Serialize, __lib::de::Deserialize, Clone, PartialEq, Debug)]
#[sats(crate = __lib)]
pub(super) struct UpdatePkI128Args {
    pub n: i128,
    pub data: i32,
}

impl From<UpdatePkI128Args> for super::Reducer {
    fn from(args: UpdatePkI128Args) -> Self {
        Self::UpdatePkI128 {
            n: args.n,
            data: args.data,
        }
    }
}

impl __sdk::InModule for UpdatePkI128Args {
    type Module = super::RemoteModule;
}

pub struct UpdatePkI128CallbackId(__sdk::CallbackId);

#[allow(non_camel_case_types)]
/// Extension trait for access to the reducer `update_pk_i128`.
///
/// Implemented for [`super::RemoteReducers`].
pub trait update_pk_i_128 {
    /// Request that the remote module invoke the reducer `update_pk_i128` to run as soon as possible.
    ///
    /// This method returns immediately, and errors only if we are unable to send the request.
    /// The reducer will run asynchronously in the future,
    ///  and its status can be observed by listening for [`Self::on_update_pk_i_128`] callbacks.
    fn update_pk_i_128(&self, n: i128, data: i32) -> __anyhow::Result<()>;
    /// Register a callback to run whenever we are notified of an invocation of the reducer `update_pk_i128`.
    ///
    /// The [`super::EventContext`] passed to the `callback`
    /// will always have [`__sdk::Event::Reducer`] as its `event`,
    /// but it may or may not have terminated successfully and been committed.
    /// Callbacks should inspect the [`__sdk::ReducerEvent`] contained in the [`super::EventContext`]
    /// to determine the reducer's status.
    ///
    /// The returned [`UpdatePkI128CallbackId`] can be passed to [`Self::remove_on_update_pk_i_128`]
    /// to cancel the callback.
    fn on_update_pk_i_128(
        &self,
        callback: impl FnMut(&super::EventContext, &i128, &i32) + Send + 'static,
    ) -> UpdatePkI128CallbackId;
    /// Cancel a callback previously registered by [`Self::on_update_pk_i_128`],
    /// causing it not to run in the future.
    fn remove_on_update_pk_i_128(&self, callback: UpdatePkI128CallbackId);
}

impl update_pk_i_128 for super::RemoteReducers {
    fn update_pk_i_128(&self, n: i128, data: i32) -> __anyhow::Result<()> {
        self.imp.call_reducer("update_pk_i128", UpdatePkI128Args { n, data })
    }
    fn on_update_pk_i_128(
        &self,
        mut callback: impl FnMut(&super::EventContext, &i128, &i32) + Send + 'static,
    ) -> UpdatePkI128CallbackId {
        UpdatePkI128CallbackId(self.imp.on_reducer(
            "update_pk_i128",
            Box::new(move |ctx: &super::EventContext| {
                let super::EventContext {
                    event:
                        __sdk::Event::Reducer(__sdk::ReducerEvent {
                            reducer: super::Reducer::UpdatePkI128 { n, data },
                            ..
                        }),
                    ..
                } = ctx
                else {
                    unreachable!()
                };
                callback(ctx, n, data)
            }),
        ))
    }
    fn remove_on_update_pk_i_128(&self, callback: UpdatePkI128CallbackId) {
        self.imp.remove_on_reducer("update_pk_i128", callback.0)
    }
}

#[allow(non_camel_case_types)]
#[doc(hidden)]
/// Extension trait for setting the call-flags for the reducer `update_pk_i128`.
///
/// Implemented for [`super::SetReducerFlags`].
///
/// This type is currently unstable and may be removed without a major version bump.
pub trait set_flags_for_update_pk_i_128 {
    /// Set the call-reducer flags for the reducer `update_pk_i128` to `flags`.
    ///
    /// This type is currently unstable and may be removed without a major version bump.
    fn update_pk_i_128(&self, flags: __ws::CallReducerFlags);
}

impl set_flags_for_update_pk_i_128 for super::SetReducerFlags {
    fn update_pk_i_128(&self, flags: __ws::CallReducerFlags) {
        self.imp.set_call_reducer_flags("update_pk_i128", flags);
    }
}
