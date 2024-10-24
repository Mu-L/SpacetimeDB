// THIS FILE IS AUTOMATICALLY GENERATED BY SPACETIMEDB. EDITS TO THIS FILE
// WILL NOT BE SAVED. MODIFY TABLES IN RUST INSTEAD.

#![allow(unused)]
use spacetimedb_sdk::__codegen::{
    self as __sdk, __lib, __sats, __ws,
    anyhow::{self as __anyhow, Context as _},
};

#[derive(__lib::ser::Serialize, __lib::de::Deserialize, Clone, PartialEq, Debug)]
#[sats(crate = __lib)]
pub struct InsertVecI8 {
    pub n: Vec<i8>,
}

impl __sdk::InModule for InsertVecI8 {
    type Module = super::RemoteModule;
}

pub struct InsertVecI8CallbackId(__sdk::CallbackId);

#[allow(non_camel_case_types)]
/// Extension trait for access to the reducer `insert_vec_i8`.
///
/// Implemented for [`super::RemoteReducers`].
pub trait insert_vec_i_8 {
    /// Request that the remote module invoke the reducer `insert_vec_i8` to run as soon as possible.
    ///
    /// This method returns immediately, and errors only if we are unable to send the request.
    /// The reducer will run asynchronously in the future,
    ///  and its status can be observed by listening for [`Self::on_insert_vec_i_8`] callbacks.
    fn insert_vec_i_8(&self, n: Vec<i8>) -> __anyhow::Result<()>;
    /// Register a callback to run whenever we are notified of an invocation of the reducer `insert_vec_i8`.
    ///
    /// The [`super::EventContext`] passed to the `callback`
    /// will always have [`__sdk::Event::Reducer`] as its `event`,
    /// but it may or may not have terminated successfully and been committed.
    /// Callbacks should inspect the [`__sdk::ReducerEvent`] contained in the [`super::EventContext`]
    /// to determine the reducer's status.
    ///
    /// The returned [`InsertVecI8CallbackId`] can be passed to [`Self::remove_on_insert_vec_i_8`]
    /// to cancel the callback.
    fn on_insert_vec_i_8(
        &self,
        callback: impl FnMut(&super::EventContext, &Vec<i8>) + Send + 'static,
    ) -> InsertVecI8CallbackId;
    /// Cancel a callback previously registered by [`Self::on_insert_vec_i_8`],
    /// causing it not to run in the future.
    fn remove_on_insert_vec_i_8(&self, callback: InsertVecI8CallbackId);
}

impl insert_vec_i_8 for super::RemoteReducers {
    fn insert_vec_i_8(&self, n: Vec<i8>) -> __anyhow::Result<()> {
        self.imp.call_reducer("insert_vec_i8", InsertVecI8 { n })
    }
    fn on_insert_vec_i_8(
        &self,
        mut callback: impl FnMut(&super::EventContext, &Vec<i8>) + Send + 'static,
    ) -> InsertVecI8CallbackId {
        InsertVecI8CallbackId(self.imp.on_reducer::<InsertVecI8>(
            "insert_vec_i8",
            Box::new(move |ctx: &super::EventContext, args: &InsertVecI8| callback(ctx, &args.n)),
        ))
    }
    fn remove_on_insert_vec_i_8(&self, callback: InsertVecI8CallbackId) {
        self.imp.remove_on_reducer::<InsertVecI8>("insert_vec_i8", callback.0)
    }
}
