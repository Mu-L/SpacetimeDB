// THIS FILE IS AUTOMATICALLY GENERATED BY SPACETIMEDB. EDITS TO THIS FILE
// WILL NOT BE SAVED. MODIFY TABLES IN RUST INSTEAD.

#![allow(unused)]
use super::pk_i_128_type::PkI128;
use spacetimedb_sdk::{
    self as __sdk,
    anyhow::{self as __anyhow, Context as _},
    lib as __lib, sats as __sats, ws_messages as __ws,
};

/// Table handle for the table `pk_i128`.
///
/// Obtain a handle from the [`PkI128TableAccess::pk_i_128`] method on [`super::RemoteTables`],
/// like `ctx.db.pk_i_128()`.
///
/// Users are encouraged not to explicitly reference this type,
/// but to directly chain method calls,
/// like `ctx.db.pk_i_128().on_insert(...)`.
pub struct PkI128TableHandle<'ctx> {
    imp: __sdk::client_cache::TableHandle<PkI128>,
    ctx: std::marker::PhantomData<&'ctx super::RemoteTables>,
}

#[allow(non_camel_case_types)]
/// Extension trait for access to the table `pk_i128`.
///
/// Implemented for [`super::RemoteTables`].
pub trait PkI128TableAccess {
    #[allow(non_snake_case)]
    /// Obtain a [`PkI128TableHandle`], which mediates access to the table `pk_i128`.
    fn pk_i_128(&self) -> PkI128TableHandle<'_>;
}

impl PkI128TableAccess for super::RemoteTables {
    fn pk_i_128(&self) -> PkI128TableHandle<'_> {
        PkI128TableHandle {
            imp: self.imp.get_table::<PkI128>("pk_i128"),
            ctx: std::marker::PhantomData,
        }
    }
}

pub struct PkI128InsertCallbackId(__sdk::callbacks::CallbackId);
pub struct PkI128DeleteCallbackId(__sdk::callbacks::CallbackId);

impl<'ctx> __sdk::table::Table for PkI128TableHandle<'ctx> {
    type Row = PkI128;
    type EventContext = super::EventContext;

    fn count(&self) -> u64 {
        self.imp.count()
    }
    fn iter(&self) -> impl Iterator<Item = PkI128> + '_ {
        self.imp.iter()
    }

    type InsertCallbackId = PkI128InsertCallbackId;

    fn on_insert(
        &self,
        callback: impl FnMut(&Self::EventContext, &Self::Row) + Send + 'static,
    ) -> PkI128InsertCallbackId {
        PkI128InsertCallbackId(self.imp.on_insert(Box::new(callback)))
    }

    fn remove_on_insert(&self, callback: PkI128InsertCallbackId) {
        self.imp.remove_on_insert(callback.0)
    }

    type DeleteCallbackId = PkI128DeleteCallbackId;

    fn on_delete(
        &self,
        callback: impl FnMut(&Self::EventContext, &Self::Row) + Send + 'static,
    ) -> PkI128DeleteCallbackId {
        PkI128DeleteCallbackId(self.imp.on_delete(Box::new(callback)))
    }

    fn remove_on_delete(&self, callback: PkI128DeleteCallbackId) {
        self.imp.remove_on_delete(callback.0)
    }
}

pub struct PkI128UpdateCallbackId(__sdk::callbacks::CallbackId);

impl<'ctx> __sdk::table::TableWithPrimaryKey for PkI128TableHandle<'ctx> {
    type UpdateCallbackId = PkI128UpdateCallbackId;

    fn on_update(
        &self,
        callback: impl FnMut(&Self::EventContext, &Self::Row, &Self::Row) + Send + 'static,
    ) -> PkI128UpdateCallbackId {
        PkI128UpdateCallbackId(self.imp.on_update(Box::new(callback)))
    }

    fn remove_on_update(&self, callback: PkI128UpdateCallbackId) {
        self.imp.remove_on_update(callback.0)
    }
}

#[doc(hidden)]
pub(super) fn parse_table_update(
    raw_updates: __ws::TableUpdate<__ws::BsatnFormat>,
) -> __anyhow::Result<__sdk::spacetime_module::TableUpdate<PkI128>> {
    __sdk::spacetime_module::TableUpdate::parse_table_update_with_primary_key::<i128>(raw_updates, |row: &PkI128| {
        &row.n
    })
    .context("Failed to parse table update for table \"pk_i128\"")
}

/// Access to the `n` unique index on the table `pk_i128`,
/// which allows point queries on the field of the same name
/// via the [`PkI128NUnique::find`] method.
///
/// Users are encouraged not to explicitly reference this type,
/// but to directly chain method calls,
/// like `ctx.db.pk_i_128().n().find(...)`.
pub struct PkI128NUnique<'ctx> {
    imp: __sdk::client_cache::UniqueConstraintHandle<PkI128, i128>,
    phantom: std::marker::PhantomData<&'ctx super::RemoteTables>,
}

impl<'ctx> PkI128TableHandle<'ctx> {
    /// Get a handle on the `n` unique index on the table `pk_i128`.
    pub fn n(&self) -> PkI128NUnique<'ctx> {
        PkI128NUnique {
            imp: self.imp.get_unique_constraint::<i128>("n", |row| &row.n),
            phantom: std::marker::PhantomData,
        }
    }
}

impl<'ctx> PkI128NUnique<'ctx> {
    /// Find the subscribed row whose `n` column value is equal to `col_val`,
    /// if such a row is present in the client cache.
    pub fn find(&self, col_val: &i128) -> Option<PkI128> {
        self.imp.find(col_val)
    }
}
