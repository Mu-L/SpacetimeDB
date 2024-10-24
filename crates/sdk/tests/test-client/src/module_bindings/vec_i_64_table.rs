// THIS FILE IS AUTOMATICALLY GENERATED BY SPACETIMEDB. EDITS TO THIS FILE
// WILL NOT BE SAVED. MODIFY TABLES IN RUST INSTEAD.

#![allow(unused)]
use super::vec_i_64_type::VecI64;
use spacetimedb_sdk::__codegen::{
    self as __sdk, __lib, __sats, __ws,
    anyhow::{self as __anyhow, Context as _},
};

/// Table handle for the table `vec_i64`.
///
/// Obtain a handle from the [`VecI64TableAccess::vec_i_64`] method on [`super::RemoteTables`],
/// like `ctx.db.vec_i_64()`.
///
/// Users are encouraged not to explicitly reference this type,
/// but to directly chain method calls,
/// like `ctx.db.vec_i_64().on_insert(...)`.
pub struct VecI64TableHandle<'ctx> {
    imp: __sdk::TableHandle<VecI64>,
    ctx: std::marker::PhantomData<&'ctx super::RemoteTables>,
}

#[allow(non_camel_case_types)]
/// Extension trait for access to the table `vec_i64`.
///
/// Implemented for [`super::RemoteTables`].
pub trait VecI64TableAccess {
    #[allow(non_snake_case)]
    /// Obtain a [`VecI64TableHandle`], which mediates access to the table `vec_i64`.
    fn vec_i_64(&self) -> VecI64TableHandle<'_>;
}

impl VecI64TableAccess for super::RemoteTables {
    fn vec_i_64(&self) -> VecI64TableHandle<'_> {
        VecI64TableHandle {
            imp: self.imp.get_table::<VecI64>("vec_i64"),
            ctx: std::marker::PhantomData,
        }
    }
}

pub struct VecI64InsertCallbackId(__sdk::CallbackId);
pub struct VecI64DeleteCallbackId(__sdk::CallbackId);

impl<'ctx> __sdk::Table for VecI64TableHandle<'ctx> {
    type Row = VecI64;
    type EventContext = super::EventContext;

    fn count(&self) -> u64 {
        self.imp.count()
    }
    fn iter(&self) -> impl Iterator<Item = VecI64> + '_ {
        self.imp.iter()
    }

    type InsertCallbackId = VecI64InsertCallbackId;

    fn on_insert(
        &self,
        callback: impl FnMut(&Self::EventContext, &Self::Row) + Send + 'static,
    ) -> VecI64InsertCallbackId {
        VecI64InsertCallbackId(self.imp.on_insert(Box::new(callback)))
    }

    fn remove_on_insert(&self, callback: VecI64InsertCallbackId) {
        self.imp.remove_on_insert(callback.0)
    }

    type DeleteCallbackId = VecI64DeleteCallbackId;

    fn on_delete(
        &self,
        callback: impl FnMut(&Self::EventContext, &Self::Row) + Send + 'static,
    ) -> VecI64DeleteCallbackId {
        VecI64DeleteCallbackId(self.imp.on_delete(Box::new(callback)))
    }

    fn remove_on_delete(&self, callback: VecI64DeleteCallbackId) {
        self.imp.remove_on_delete(callback.0)
    }
}

#[doc(hidden)]
pub(super) fn parse_table_update(
    raw_updates: __ws::TableUpdate<__ws::BsatnFormat>,
) -> __anyhow::Result<__sdk::TableUpdate<VecI64>> {
    __sdk::TableUpdate::parse_table_update_no_primary_key(raw_updates)
        .context("Failed to parse table update for table \"vec_i64\"")
}
