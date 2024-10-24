// THIS FILE IS AUTOMATICALLY GENERATED BY SPACETIMEDB. EDITS TO THIS FILE
// WILL NOT BE SAVED. MODIFY TABLES IN RUST INSTEAD.

#![allow(unused)]
use super::vec_address_type::VecAddress;
use spacetimedb_sdk::__codegen::{
    self as __sdk, __lib, __sats, __ws,
    anyhow::{self as __anyhow, Context as _},
};

/// Table handle for the table `vec_address`.
///
/// Obtain a handle from the [`VecAddressTableAccess::vec_address`] method on [`super::RemoteTables`],
/// like `ctx.db.vec_address()`.
///
/// Users are encouraged not to explicitly reference this type,
/// but to directly chain method calls,
/// like `ctx.db.vec_address().on_insert(...)`.
pub struct VecAddressTableHandle<'ctx> {
    imp: __sdk::TableHandle<VecAddress>,
    ctx: std::marker::PhantomData<&'ctx super::RemoteTables>,
}

#[allow(non_camel_case_types)]
/// Extension trait for access to the table `vec_address`.
///
/// Implemented for [`super::RemoteTables`].
pub trait VecAddressTableAccess {
    #[allow(non_snake_case)]
    /// Obtain a [`VecAddressTableHandle`], which mediates access to the table `vec_address`.
    fn vec_address(&self) -> VecAddressTableHandle<'_>;
}

impl VecAddressTableAccess for super::RemoteTables {
    fn vec_address(&self) -> VecAddressTableHandle<'_> {
        VecAddressTableHandle {
            imp: self.imp.get_table::<VecAddress>("vec_address"),
            ctx: std::marker::PhantomData,
        }
    }
}

pub struct VecAddressInsertCallbackId(__sdk::CallbackId);
pub struct VecAddressDeleteCallbackId(__sdk::CallbackId);

impl<'ctx> __sdk::Table for VecAddressTableHandle<'ctx> {
    type Row = VecAddress;
    type EventContext = super::EventContext;

    fn count(&self) -> u64 {
        self.imp.count()
    }
    fn iter(&self) -> impl Iterator<Item = VecAddress> + '_ {
        self.imp.iter()
    }

    type InsertCallbackId = VecAddressInsertCallbackId;

    fn on_insert(
        &self,
        callback: impl FnMut(&Self::EventContext, &Self::Row) + Send + 'static,
    ) -> VecAddressInsertCallbackId {
        VecAddressInsertCallbackId(self.imp.on_insert(Box::new(callback)))
    }

    fn remove_on_insert(&self, callback: VecAddressInsertCallbackId) {
        self.imp.remove_on_insert(callback.0)
    }

    type DeleteCallbackId = VecAddressDeleteCallbackId;

    fn on_delete(
        &self,
        callback: impl FnMut(&Self::EventContext, &Self::Row) + Send + 'static,
    ) -> VecAddressDeleteCallbackId {
        VecAddressDeleteCallbackId(self.imp.on_delete(Box::new(callback)))
    }

    fn remove_on_delete(&self, callback: VecAddressDeleteCallbackId) {
        self.imp.remove_on_delete(callback.0)
    }
}

#[doc(hidden)]
pub(super) fn parse_table_update(
    raw_updates: __ws::TableUpdate<__ws::BsatnFormat>,
) -> __anyhow::Result<__sdk::TableUpdate<VecAddress>> {
    __sdk::TableUpdate::parse_table_update_no_primary_key(raw_updates)
        .context("Failed to parse table update for table \"vec_address\"")
}
