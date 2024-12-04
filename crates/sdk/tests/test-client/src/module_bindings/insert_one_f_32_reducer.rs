// THIS FILE IS AUTOMATICALLY GENERATED BY SPACETIMEDB. EDITS TO THIS FILE
// WILL NOT BE SAVED. MODIFY TABLES IN RUST INSTEAD.

#![allow(unused)]
use spacetimedb_sdk::__codegen::{
    self as __sdk, __lib, __sats, __ws,
    anyhow::{self as __anyhow, Context as _},
};

#[derive(__lib::ser::Serialize, __lib::de::Deserialize, Clone, PartialEq, Debug)]
#[sats(crate = __lib)]
pub(super) struct InsertOneF32Args {
    pub f: f32,
}

impl From<InsertOneF32Args> for super::Reducer {
    fn from(args: InsertOneF32Args) -> Self {
        Self::InsertOneF32 { f: args.f }
    }
}

impl __sdk::InModule for InsertOneF32Args {
    type Module = super::RemoteModule;
}

pub struct InsertOneF32CallbackId(__sdk::CallbackId);

#[allow(non_camel_case_types)]
/// Extension trait for access to the reducer `insert_one_f32`.
///
/// Implemented for [`super::RemoteReducers`].
pub trait insert_one_f_32 {
    /// Request that the remote module invoke the reducer `insert_one_f32` to run as soon as possible.
    ///
    /// This method returns immediately, and errors only if we are unable to send the request.
    /// The reducer will run asynchronously in the future,
    ///  and its status can be observed by listening for [`Self::on_insert_one_f_32`] callbacks.
    fn insert_one_f_32(&self, f: f32) -> __anyhow::Result<()>;
    /// Register a callback to run whenever we are notified of an invocation of the reducer `insert_one_f32`.
    ///
    /// The [`super::EventContext`] passed to the `callback`
    /// will always have [`__sdk::Event::Reducer`] as its `event`,
    /// but it may or may not have terminated successfully and been committed.
    /// Callbacks should inspect the [`__sdk::ReducerEvent`] contained in the [`super::EventContext`]
    /// to determine the reducer's status.
    ///
    /// The returned [`InsertOneF32CallbackId`] can be passed to [`Self::remove_on_insert_one_f_32`]
    /// to cancel the callback.
    fn on_insert_one_f_32(
        &self,
        callback: impl FnMut(&super::EventContext, &f32) + Send + 'static,
    ) -> InsertOneF32CallbackId;
    /// Cancel a callback previously registered by [`Self::on_insert_one_f_32`],
    /// causing it not to run in the future.
    fn remove_on_insert_one_f_32(&self, callback: InsertOneF32CallbackId);
}

impl insert_one_f_32 for super::RemoteReducers {
    fn insert_one_f_32(&self, f: f32) -> __anyhow::Result<()> {
        self.imp.call_reducer("insert_one_f32", InsertOneF32Args { f })
    }
    fn on_insert_one_f_32(
        &self,
        mut callback: impl FnMut(&super::EventContext, &f32) + Send + 'static,
    ) -> InsertOneF32CallbackId {
        InsertOneF32CallbackId(self.imp.on_reducer(
            "insert_one_f32",
            Box::new(move |ctx: &super::EventContext| {
                let super::EventContext {
                    event:
                        __sdk::Event::Reducer(__sdk::ReducerEvent {
                            reducer: super::Reducer::InsertOneF32 { f },
                            ..
                        }),
                    ..
                } = ctx
                else {
                    unreachable!()
                };
                callback(ctx, f)
            }),
        ))
    }
    fn remove_on_insert_one_f_32(&self, callback: InsertOneF32CallbackId) {
        self.imp.remove_on_reducer("insert_one_f32", callback.0)
    }
}

#[allow(non_camel_case_types)]
#[doc(hidden)]
/// Extension trait for setting the call-flags for the reducer `insert_one_f32`.
///
/// Implemented for [`super::SetReducerFlags`].
///
/// This type is currently unstable and may be removed without a major version bump.
pub trait set_flags_for_insert_one_f_32 {
    /// Set the call-reducer flags for the reducer `insert_one_f32` to `flags`.
    ///
    /// This type is currently unstable and may be removed without a major version bump.
    fn insert_one_f_32(&self, flags: __ws::CallReducerFlags);
}

impl set_flags_for_insert_one_f_32 for super::SetReducerFlags {
    fn insert_one_f_32(&self, flags: __ws::CallReducerFlags) {
        self.imp.set_call_reducer_flags("insert_one_f32", flags);
    }
}
