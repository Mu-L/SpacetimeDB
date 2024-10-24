// THIS FILE IS AUTOMATICALLY GENERATED BY SPACETIMEDB. EDITS TO THIS FILE
// WILL NOT BE SAVED. MODIFY TABLES IN RUST INSTEAD.

#![allow(unused)]
use super::unique_i_16_type::UniqueI16;
use spacetimedb_sdk::__codegen::{
    self as __sdk, __lib, __sats, __ws,
    anyhow::{self as __anyhow, Context as _},
};

/// Table handle for the table `unique_i16`.
///
/// Obtain a handle from the [`UniqueI16TableAccess::unique_i_16`] method on [`super::RemoteTables`],
/// like `ctx.db.unique_i_16()`.
///
/// Users are encouraged not to explicitly reference this type,
/// but to directly chain method calls,
/// like `ctx.db.unique_i_16().on_insert(...)`.
pub struct UniqueI16TableHandle<'ctx> {
    imp: __sdk::TableHandle<UniqueI16>,
    ctx: std::marker::PhantomData<&'ctx super::RemoteTables>,
}

#[allow(non_camel_case_types)]
/// Extension trait for access to the table `unique_i16`.
///
/// Implemented for [`super::RemoteTables`].
pub trait UniqueI16TableAccess {
    #[allow(non_snake_case)]
    /// Obtain a [`UniqueI16TableHandle`], which mediates access to the table `unique_i16`.
    fn unique_i_16(&self) -> UniqueI16TableHandle<'_>;
}

impl UniqueI16TableAccess for super::RemoteTables {
    fn unique_i_16(&self) -> UniqueI16TableHandle<'_> {
        UniqueI16TableHandle {
            imp: self.imp.get_table::<UniqueI16>("unique_i16"),
            ctx: std::marker::PhantomData,
        }
    }
}

pub struct UniqueI16InsertCallbackId(__sdk::CallbackId);
pub struct UniqueI16DeleteCallbackId(__sdk::CallbackId);

impl<'ctx> __sdk::Table for UniqueI16TableHandle<'ctx> {
    type Row = UniqueI16;
    type EventContext = super::EventContext;

    fn count(&self) -> u64 {
        self.imp.count()
    }
    fn iter(&self) -> impl Iterator<Item = UniqueI16> + '_ {
        self.imp.iter()
    }

    type InsertCallbackId = UniqueI16InsertCallbackId;

    fn on_insert(
        &self,
        callback: impl FnMut(&Self::EventContext, &Self::Row) + Send + 'static,
    ) -> UniqueI16InsertCallbackId {
        UniqueI16InsertCallbackId(self.imp.on_insert(Box::new(callback)))
    }

    fn remove_on_insert(&self, callback: UniqueI16InsertCallbackId) {
        self.imp.remove_on_insert(callback.0)
    }

    type DeleteCallbackId = UniqueI16DeleteCallbackId;

    fn on_delete(
        &self,
        callback: impl FnMut(&Self::EventContext, &Self::Row) + Send + 'static,
    ) -> UniqueI16DeleteCallbackId {
        UniqueI16DeleteCallbackId(self.imp.on_delete(Box::new(callback)))
    }

    fn remove_on_delete(&self, callback: UniqueI16DeleteCallbackId) {
        self.imp.remove_on_delete(callback.0)
    }
}

#[doc(hidden)]
pub(super) fn parse_table_update(
    raw_updates: __ws::TableUpdate<__ws::BsatnFormat>,
) -> __anyhow::Result<__sdk::TableUpdate<UniqueI16>> {
    __sdk::TableUpdate::parse_table_update_no_primary_key(raw_updates)
        .context("Failed to parse table update for table \"unique_i16\"")
}

/// Access to the `n` unique index on the table `unique_i16`,
/// which allows point queries on the field of the same name
/// via the [`UniqueI16NUnique::find`] method.
///
/// Users are encouraged not to explicitly reference this type,
/// but to directly chain method calls,
/// like `ctx.db.unique_i_16().n().find(...)`.
pub struct UniqueI16NUnique<'ctx> {
    imp: __sdk::UniqueConstraintHandle<UniqueI16, i16>,
    phantom: std::marker::PhantomData<&'ctx super::RemoteTables>,
}

impl<'ctx> UniqueI16TableHandle<'ctx> {
    /// Get a handle on the `n` unique index on the table `unique_i16`.
    pub fn n(&self) -> UniqueI16NUnique<'ctx> {
        UniqueI16NUnique {
            imp: self.imp.get_unique_constraint::<i16>("n", |row| &row.n),
            phantom: std::marker::PhantomData,
        }
    }
}

impl<'ctx> UniqueI16NUnique<'ctx> {
    /// Find the subscribed row whose `n` column value is equal to `col_val`,
    /// if such a row is present in the client cache.
    pub fn find(&self, col_val: &i16) -> Option<UniqueI16> {
        self.imp.find(col_val)
    }
}
