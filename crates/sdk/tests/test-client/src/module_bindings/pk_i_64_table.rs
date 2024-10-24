// THIS FILE IS AUTOMATICALLY GENERATED BY SPACETIMEDB. EDITS TO THIS FILE
// WILL NOT BE SAVED. MODIFY TABLES IN RUST INSTEAD.

#![allow(unused)]
use super::pk_i_64_type::PkI64;
use spacetimedb_sdk::{
    self as __sdk,
    anyhow::{self as __anyhow, Context as _},
    lib as __lib, sats as __sats, ws_messages as __ws,
};

/// Table handle for the table `pk_i64`.
///
/// Obtain a handle from the [`PkI64TableAccess::pk_i_64`] method on [`super::RemoteTables`],
/// like `ctx.db.pk_i_64()`.
///
/// Users are encouraged not to explicitly reference this type,
/// but to directly chain method calls,
/// like `ctx.db.pk_i_64().on_insert(...)`.
pub struct PkI64TableHandle<'ctx> {
    imp: __sdk::client_cache::TableHandle<PkI64>,
    ctx: std::marker::PhantomData<&'ctx super::RemoteTables>,
}

#[allow(non_camel_case_types)]
/// Extension trait for access to the table `pk_i64`.
///
/// Implemented for [`super::RemoteTables`].
pub trait PkI64TableAccess {
    #[allow(non_snake_case)]
    /// Obtain a [`PkI64TableHandle`], which mediates access to the table `pk_i64`.
    fn pk_i_64(&self) -> PkI64TableHandle<'_>;
}

impl PkI64TableAccess for super::RemoteTables {
    fn pk_i_64(&self) -> PkI64TableHandle<'_> {
        PkI64TableHandle {
            imp: self.imp.get_table::<PkI64>("pk_i64"),
            ctx: std::marker::PhantomData,
        }
    }
}

pub struct PkI64InsertCallbackId(__sdk::callbacks::CallbackId);
pub struct PkI64DeleteCallbackId(__sdk::callbacks::CallbackId);

impl<'ctx> __sdk::table::Table for PkI64TableHandle<'ctx> {
    type Row = PkI64;
    type EventContext = super::EventContext;

    fn count(&self) -> u64 {
        self.imp.count()
    }
    fn iter(&self) -> impl Iterator<Item = PkI64> + '_ {
        self.imp.iter()
    }

    type InsertCallbackId = PkI64InsertCallbackId;

    fn on_insert(
        &self,
        callback: impl FnMut(&Self::EventContext, &Self::Row) + Send + 'static,
    ) -> PkI64InsertCallbackId {
        PkI64InsertCallbackId(self.imp.on_insert(Box::new(callback)))
    }

    fn remove_on_insert(&self, callback: PkI64InsertCallbackId) {
        self.imp.remove_on_insert(callback.0)
    }

    type DeleteCallbackId = PkI64DeleteCallbackId;

    fn on_delete(
        &self,
        callback: impl FnMut(&Self::EventContext, &Self::Row) + Send + 'static,
    ) -> PkI64DeleteCallbackId {
        PkI64DeleteCallbackId(self.imp.on_delete(Box::new(callback)))
    }

    fn remove_on_delete(&self, callback: PkI64DeleteCallbackId) {
        self.imp.remove_on_delete(callback.0)
    }
}

pub struct PkI64UpdateCallbackId(__sdk::callbacks::CallbackId);

impl<'ctx> __sdk::table::TableWithPrimaryKey for PkI64TableHandle<'ctx> {
    type UpdateCallbackId = PkI64UpdateCallbackId;

    fn on_update(
        &self,
        callback: impl FnMut(&Self::EventContext, &Self::Row, &Self::Row) + Send + 'static,
    ) -> PkI64UpdateCallbackId {
        PkI64UpdateCallbackId(self.imp.on_update(Box::new(callback)))
    }

    fn remove_on_update(&self, callback: PkI64UpdateCallbackId) {
        self.imp.remove_on_update(callback.0)
    }
}

#[doc(hidden)]
pub(super) fn parse_table_update(
    raw_updates: __ws::TableUpdate<__ws::BsatnFormat>,
) -> __anyhow::Result<__sdk::spacetime_module::TableUpdate<PkI64>> {
    __sdk::spacetime_module::TableUpdate::parse_table_update_with_primary_key::<i64>(raw_updates, |row: &PkI64| &row.n)
        .context("Failed to parse table update for table \"pk_i64\"")
}

/// Access to the `n` unique index on the table `pk_i64`,
/// which allows point queries on the field of the same name
/// via the [`PkI64NUnique::find`] method.
///
/// Users are encouraged not to explicitly reference this type,
/// but to directly chain method calls,
/// like `ctx.db.pk_i_64().n().find(...)`.
pub struct PkI64NUnique<'ctx> {
    imp: __sdk::client_cache::UniqueConstraintHandle<PkI64, i64>,
    phantom: std::marker::PhantomData<&'ctx super::RemoteTables>,
}

impl<'ctx> PkI64TableHandle<'ctx> {
    /// Get a handle on the `n` unique index on the table `pk_i64`.
    pub fn n(&self) -> PkI64NUnique<'ctx> {
        PkI64NUnique {
            imp: self.imp.get_unique_constraint::<i64>("n", |row| &row.n),
            phantom: std::marker::PhantomData,
        }
    }
}

impl<'ctx> PkI64NUnique<'ctx> {
    /// Find the subscribed row whose `n` column value is equal to `col_val`,
    /// if such a row is present in the client cache.
    pub fn find(&self, col_val: &i64) -> Option<PkI64> {
        self.imp.find(col_val)
    }
}
