// THIS FILE IS AUTOMATICALLY GENERATED BY SPACETIMEDB. EDITS TO THIS FILE
// WILL NOT BE SAVED. MODIFY TABLES IN RUST INSTEAD.

#![allow(unused, clippy::all)]
use super::vec_bool_type::VecBool;
use spacetimedb_sdk::__codegen::{
    self as __sdk, __lib, __sats, __ws,
    anyhow::{self as __anyhow, Context as _},
};

/// Table handle for the table `vec_bool`.
///
/// Obtain a handle from the [`VecBoolTableAccess::vec_bool`] method on [`super::RemoteTables`],
/// like `ctx.db.vec_bool()`.
///
/// Users are encouraged not to explicitly reference this type,
/// but to directly chain method calls,
/// like `ctx.db.vec_bool().on_insert(...)`.
pub struct VecBoolTableHandle<'ctx> {
    imp: __sdk::TableHandle<VecBool>,
    ctx: std::marker::PhantomData<&'ctx super::RemoteTables>,
}

#[allow(non_camel_case_types)]
/// Extension trait for access to the table `vec_bool`.
///
/// Implemented for [`super::RemoteTables`].
pub trait VecBoolTableAccess {
    #[allow(non_snake_case)]
    /// Obtain a [`VecBoolTableHandle`], which mediates access to the table `vec_bool`.
    fn vec_bool(&self) -> VecBoolTableHandle<'_>;
}

impl VecBoolTableAccess for super::RemoteTables {
    fn vec_bool(&self) -> VecBoolTableHandle<'_> {
        VecBoolTableHandle {
            imp: self.imp.get_table::<VecBool>("vec_bool"),
            ctx: std::marker::PhantomData,
        }
    }
}

pub struct VecBoolInsertCallbackId(__sdk::CallbackId);
pub struct VecBoolDeleteCallbackId(__sdk::CallbackId);

impl<'ctx> __sdk::Table for VecBoolTableHandle<'ctx> {
    type Row = VecBool;
    type EventContext = super::EventContext;

    fn count(&self) -> u64 {
        self.imp.count()
    }
    fn iter(&self) -> impl Iterator<Item = VecBool> + '_ {
        self.imp.iter()
    }

    type InsertCallbackId = VecBoolInsertCallbackId;

    fn on_insert(
        &self,
        callback: impl FnMut(&Self::EventContext, &Self::Row) + Send + 'static,
    ) -> VecBoolInsertCallbackId {
        VecBoolInsertCallbackId(self.imp.on_insert(Box::new(callback)))
    }

    fn remove_on_insert(&self, callback: VecBoolInsertCallbackId) {
        self.imp.remove_on_insert(callback.0)
    }

    type DeleteCallbackId = VecBoolDeleteCallbackId;

    fn on_delete(
        &self,
        callback: impl FnMut(&Self::EventContext, &Self::Row) + Send + 'static,
    ) -> VecBoolDeleteCallbackId {
        VecBoolDeleteCallbackId(self.imp.on_delete(Box::new(callback)))
    }

    fn remove_on_delete(&self, callback: VecBoolDeleteCallbackId) {
        self.imp.remove_on_delete(callback.0)
    }
}

#[doc(hidden)]
pub(super) fn register_table(client_cache: &mut __sdk::ClientCache<super::RemoteModule>) {
    let _table = client_cache.get_or_make_table::<VecBool>("vec_bool");
}
#[doc(hidden)]
pub(super) fn parse_table_update(
    raw_updates: __ws::TableUpdate<__ws::BsatnFormat>,
) -> __anyhow::Result<__sdk::TableUpdate<VecBool>> {
    __sdk::TableUpdate::parse_table_update_no_primary_key(raw_updates)
        .context("Failed to parse table update for table \"vec_bool\"")
}
