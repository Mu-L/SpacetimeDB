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
pub struct InsertVecU128 {
    pub n: Vec<u128>,
}

impl __sdk::spacetime_module::InModule for InsertVecU128 {
    type Module = super::RemoteModule;
}

pub struct InsertVecU128CallbackId(__sdk::callbacks::CallbackId);

#[allow(non_camel_case_types)]
/// Extension trait for access to the reducer `insert_vec_u128`.
///
/// Implemented for [`super::RemoteReducers`].
pub trait insert_vec_u_128 {
    /// Request that the remote module invoke the reducer `insert_vec_u128` to run as soon as possible.
    ///
    /// This method returns immediately, and errors only if we are unable to send the request.
    /// The reducer will run asynchronously in the future,
    ///  and its status can be observed by listening for [`Self::on_insert_vec_u_128`] callbacks.
    fn insert_vec_u_128(&self, n: Vec<u128>) -> __anyhow::Result<()>;
    /// Register a callback to run whenever we are notified of an invocation of the reducer `insert_vec_u128`.
    ///
    /// The [`super::EventContext`] passed to the `callback`
    /// will always have [`__sdk::Event::Reducer`] as its `event`,
    /// but it may or may not have terminated successfully and been committed.
    /// Callbacks should inspect the [`__sdk::ReducerEvent`] contained in the [`super::EventContext`]
    /// to determine the reducer's status.
    ///
    /// The returned [`InsertVecU128CallbackId`] can be passed to [`Self::remove_on_insert_vec_u_128`]
    /// to cancel the callback.
    fn on_insert_vec_u_128(
        &self,
        callback: impl FnMut(&super::EventContext, &Vec<u128>) + Send + 'static,
    ) -> InsertVecU128CallbackId;
    /// Cancel a callback previously registered by [`Self::on_insert_vec_u_128`],
    /// causing it not to run in the future.
    fn remove_on_insert_vec_u_128(&self, callback: InsertVecU128CallbackId);
}

impl insert_vec_u_128 for super::RemoteReducers {
    fn insert_vec_u_128(&self, n: Vec<u128>) -> __anyhow::Result<()> {
        self.imp.call_reducer("insert_vec_u128", InsertVecU128 { n })
    }
    fn on_insert_vec_u_128(
        &self,
        mut callback: impl FnMut(&super::EventContext, &Vec<u128>) + Send + 'static,
    ) -> InsertVecU128CallbackId {
        InsertVecU128CallbackId(self.imp.on_reducer::<InsertVecU128>(
            "insert_vec_u128",
            Box::new(move |ctx: &super::EventContext, args: &InsertVecU128| callback(ctx, &args.n)),
        ))
    }
    fn remove_on_insert_vec_u_128(&self, callback: InsertVecU128CallbackId) {
        self.imp
            .remove_on_reducer::<InsertVecU128>("insert_vec_u128", callback.0)
    }
}
