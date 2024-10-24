// THIS FILE IS AUTOMATICALLY GENERATED BY SPACETIMEDB. EDITS TO THIS FILE
// WILL NOT BE SAVED. MODIFY TABLES IN RUST INSTEAD.

#![allow(unused)]
use super::one_i_128_type::OneI128;
use spacetimedb_sdk::__codegen::{
    self as __sdk, __lib, __sats, __ws,
    anyhow::{self as __anyhow, Context as _},
};

/// Table handle for the table `one_i128`.
///
/// Obtain a handle from the [`OneI128TableAccess::one_i_128`] method on [`super::RemoteTables`],
/// like `ctx.db.one_i_128()`.
///
/// Users are encouraged not to explicitly reference this type,
/// but to directly chain method calls,
/// like `ctx.db.one_i_128().on_insert(...)`.
pub struct OneI128TableHandle<'ctx> {
    imp: __sdk::TableHandle<OneI128>,
    ctx: std::marker::PhantomData<&'ctx super::RemoteTables>,
}

#[allow(non_camel_case_types)]
/// Extension trait for access to the table `one_i128`.
///
/// Implemented for [`super::RemoteTables`].
pub trait OneI128TableAccess {
    #[allow(non_snake_case)]
    /// Obtain a [`OneI128TableHandle`], which mediates access to the table `one_i128`.
    fn one_i_128(&self) -> OneI128TableHandle<'_>;
}

impl OneI128TableAccess for super::RemoteTables {
    fn one_i_128(&self) -> OneI128TableHandle<'_> {
        OneI128TableHandle {
            imp: self.imp.get_table::<OneI128>("one_i128"),
            ctx: std::marker::PhantomData,
        }
    }
}

pub struct OneI128InsertCallbackId(__sdk::CallbackId);
pub struct OneI128DeleteCallbackId(__sdk::CallbackId);

impl<'ctx> __sdk::Table for OneI128TableHandle<'ctx> {
    type Row = OneI128;
    type EventContext = super::EventContext;

    fn count(&self) -> u64 {
        self.imp.count()
    }
    fn iter(&self) -> impl Iterator<Item = OneI128> + '_ {
        self.imp.iter()
    }

    type InsertCallbackId = OneI128InsertCallbackId;

    fn on_insert(
        &self,
        callback: impl FnMut(&Self::EventContext, &Self::Row) + Send + 'static,
    ) -> OneI128InsertCallbackId {
        OneI128InsertCallbackId(self.imp.on_insert(Box::new(callback)))
    }

    fn remove_on_insert(&self, callback: OneI128InsertCallbackId) {
        self.imp.remove_on_insert(callback.0)
    }

    type DeleteCallbackId = OneI128DeleteCallbackId;

    fn on_delete(
        &self,
        callback: impl FnMut(&Self::EventContext, &Self::Row) + Send + 'static,
    ) -> OneI128DeleteCallbackId {
        OneI128DeleteCallbackId(self.imp.on_delete(Box::new(callback)))
    }

    fn remove_on_delete(&self, callback: OneI128DeleteCallbackId) {
        self.imp.remove_on_delete(callback.0)
    }
}

#[doc(hidden)]
pub(super) fn parse_table_update(
    raw_updates: __ws::TableUpdate<__ws::BsatnFormat>,
) -> __anyhow::Result<__sdk::TableUpdate<OneI128>> {
    __sdk::TableUpdate::parse_table_update_no_primary_key(raw_updates)
        .context("Failed to parse table update for table \"one_i128\"")
}
