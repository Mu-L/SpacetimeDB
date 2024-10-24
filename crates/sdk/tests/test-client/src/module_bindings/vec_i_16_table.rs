// THIS FILE IS AUTOMATICALLY GENERATED BY SPACETIMEDB. EDITS TO THIS FILE
// WILL NOT BE SAVED. MODIFY TABLES IN RUST INSTEAD.

#![allow(unused)]
use super::vec_i_16_type::VecI16;
use spacetimedb_sdk::{
    self as __sdk,
    anyhow::{self as __anyhow, Context as _},
    lib as __lib, sats as __sats, ws_messages as __ws,
};

/// Table handle for the table `vec_i16`.
///
/// Obtain a handle from the [`VecI16TableAccess::vec_i_16`] method on [`super::RemoteTables`],
/// like `ctx.db.vec_i_16()`.
///
/// Users are encouraged not to explicitly reference this type,
/// but to directly chain method calls,
/// like `ctx.db.vec_i_16().on_insert(...)`.
pub struct VecI16TableHandle<'ctx> {
    imp: __sdk::client_cache::TableHandle<VecI16>,
    ctx: std::marker::PhantomData<&'ctx super::RemoteTables>,
}

#[allow(non_camel_case_types)]
/// Extension trait for access to the table `vec_i16`.
///
/// Implemented for [`super::RemoteTables`].
pub trait VecI16TableAccess {
    #[allow(non_snake_case)]
    /// Obtain a [`VecI16TableHandle`], which mediates access to the table `vec_i16`.
    fn vec_i_16(&self) -> VecI16TableHandle<'_>;
}

impl VecI16TableAccess for super::RemoteTables {
    fn vec_i_16(&self) -> VecI16TableHandle<'_> {
        VecI16TableHandle {
            imp: self.imp.get_table::<VecI16>("vec_i16"),
            ctx: std::marker::PhantomData,
        }
    }
}

pub struct VecI16InsertCallbackId(__sdk::callbacks::CallbackId);
pub struct VecI16DeleteCallbackId(__sdk::callbacks::CallbackId);

impl<'ctx> __sdk::table::Table for VecI16TableHandle<'ctx> {
    type Row = VecI16;
    type EventContext = super::EventContext;

    fn count(&self) -> u64 {
        self.imp.count()
    }
    fn iter(&self) -> impl Iterator<Item = VecI16> + '_ {
        self.imp.iter()
    }

    type InsertCallbackId = VecI16InsertCallbackId;

    fn on_insert(
        &self,
        callback: impl FnMut(&Self::EventContext, &Self::Row) + Send + 'static,
    ) -> VecI16InsertCallbackId {
        VecI16InsertCallbackId(self.imp.on_insert(Box::new(callback)))
    }

    fn remove_on_insert(&self, callback: VecI16InsertCallbackId) {
        self.imp.remove_on_insert(callback.0)
    }

    type DeleteCallbackId = VecI16DeleteCallbackId;

    fn on_delete(
        &self,
        callback: impl FnMut(&Self::EventContext, &Self::Row) + Send + 'static,
    ) -> VecI16DeleteCallbackId {
        VecI16DeleteCallbackId(self.imp.on_delete(Box::new(callback)))
    }

    fn remove_on_delete(&self, callback: VecI16DeleteCallbackId) {
        self.imp.remove_on_delete(callback.0)
    }
}

#[doc(hidden)]
pub(super) fn parse_table_update(
    raw_updates: __ws::TableUpdate<__ws::BsatnFormat>,
) -> __anyhow::Result<__sdk::spacetime_module::TableUpdate<VecI16>> {
    __sdk::spacetime_module::TableUpdate::parse_table_update_no_primary_key(raw_updates)
        .context("Failed to parse table update for table \"vec_i16\"")
}
