// THIS FILE IS AUTOMATICALLY GENERATED BY SPACETIMEDB. EDITS TO THIS FILE
// WILL NOT BE SAVED. MODIFY TABLES IN RUST INSTEAD.

#![allow(unused)]
use super::unique_u_256_type::UniqueU256;
use spacetimedb_sdk::{
    self as __sdk,
    anyhow::{self as __anyhow, Context as _},
    lib as __lib, sats as __sats, ws_messages as __ws,
};

/// Table handle for the table `unique_u256`.
///
/// Obtain a handle from the [`UniqueU256TableAccess::unique_u_256`] method on [`super::RemoteTables`],
/// like `ctx.db.unique_u_256()`.
///
/// Users are encouraged not to explicitly reference this type,
/// but to directly chain method calls,
/// like `ctx.db.unique_u_256().on_insert(...)`.
pub struct UniqueU256TableHandle<'ctx> {
    imp: __sdk::db_connection::TableHandle<UniqueU256>,
    ctx: std::marker::PhantomData<&'ctx super::RemoteTables>,
}

#[allow(non_camel_case_types)]
/// Extension trait for access to the table `unique_u256`.
///
/// Implemented for [`super::RemoteTables`].
pub trait UniqueU256TableAccess {
    #[allow(non_snake_case)]
    /// Obtain a [`UniqueU256TableHandle`], which mediates access to the table `unique_u256`.
    fn unique_u_256(&self) -> UniqueU256TableHandle<'_>;
}

impl UniqueU256TableAccess for super::RemoteTables {
    fn unique_u_256(&self) -> UniqueU256TableHandle<'_> {
        UniqueU256TableHandle {
            imp: self.imp.get_table::<UniqueU256>("unique_u256"),
            ctx: std::marker::PhantomData,
        }
    }
}

pub struct UniqueU256InsertCallbackId(__sdk::callbacks::CallbackId);
pub struct UniqueU256DeleteCallbackId(__sdk::callbacks::CallbackId);

impl<'ctx> __sdk::table::Table for UniqueU256TableHandle<'ctx> {
    type Row = UniqueU256;
    type EventContext = super::EventContext;

    fn count(&self) -> u64 {
        self.imp.count()
    }
    fn iter(&self) -> impl Iterator<Item = UniqueU256> + '_ {
        self.imp.iter()
    }

    type InsertCallbackId = UniqueU256InsertCallbackId;

    fn on_insert(
        &self,
        callback: impl FnMut(&Self::EventContext, &Self::Row) + Send + 'static,
    ) -> UniqueU256InsertCallbackId {
        UniqueU256InsertCallbackId(self.imp.on_insert(Box::new(callback)))
    }

    fn remove_on_insert(&self, callback: UniqueU256InsertCallbackId) {
        self.imp.remove_on_insert(callback.0)
    }

    type DeleteCallbackId = UniqueU256DeleteCallbackId;

    fn on_delete(
        &self,
        callback: impl FnMut(&Self::EventContext, &Self::Row) + Send + 'static,
    ) -> UniqueU256DeleteCallbackId {
        UniqueU256DeleteCallbackId(self.imp.on_delete(Box::new(callback)))
    }

    fn remove_on_delete(&self, callback: UniqueU256DeleteCallbackId) {
        self.imp.remove_on_delete(callback.0)
    }
}

#[doc(hidden)]
pub(super) fn parse_table_update(
    raw_updates: __ws::TableUpdate<__ws::BsatnFormat>,
) -> __anyhow::Result<__sdk::spacetime_module::TableUpdate<UniqueU256>> {
    __sdk::spacetime_module::TableUpdate::parse_table_update_no_primary_key(raw_updates)
        .context("Failed to parse table update for table \"unique_u256\"")
}

/// Access to the `n` unique index on the table `unique_u256`,
/// which allows point queries on the field of the same name
/// via the [`UniqueU256NUnique::find`] method.
///
/// Users are encouraged not to explicitly reference this type,
/// but to directly chain method calls,
/// like `ctx.db.unique_u_256().n().find(...)`.
pub struct UniqueU256NUnique<'ctx> {
    imp: __sdk::client_cache::UniqueConstraint<UniqueU256, __sats::u256>,
    phantom: std::marker::PhantomData<&'ctx super::RemoteTables>,
}

impl<'ctx> UniqueU256TableHandle<'ctx> {
    /// Get a handle on the `n` unique index on the table `unique_u256`.
    pub fn n(&self) -> UniqueU256NUnique<'ctx> {
        UniqueU256NUnique {
            imp: self.imp.get_unique_constraint::<__sats::u256>("n", |row| &row.n),
            phantom: std::marker::PhantomData,
        }
    }
}

impl<'ctx> UniqueU256NUnique<'ctx> {
    /// Find the subscribed row whose `n` column value is equal to `col_val`,
    /// if such a row is present in the client cache.
    pub fn find(&self, col_val: &__sats::u256) -> Option<UniqueU256> {
        self.imp.find(col_val)
    }
}
