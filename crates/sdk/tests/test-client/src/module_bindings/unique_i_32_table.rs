// THIS FILE IS AUTOMATICALLY GENERATED BY SPACETIMEDB. EDITS TO THIS FILE
// WILL NOT BE SAVED. MODIFY TABLES IN RUST INSTEAD.

#![allow(unused)]
use super::unique_i_32_type::UniqueI32;
use spacetimedb_sdk::__codegen::{
    self as __sdk, __lib, __sats, __ws,
    anyhow::{self as __anyhow, Context as _},
};

/// Table handle for the table `unique_i32`.
///
/// Obtain a handle from the [`UniqueI32TableAccess::unique_i_32`] method on [`super::RemoteTables`],
/// like `ctx.db.unique_i_32()`.
///
/// Users are encouraged not to explicitly reference this type,
/// but to directly chain method calls,
/// like `ctx.db.unique_i_32().on_insert(...)`.
pub struct UniqueI32TableHandle<'ctx> {
    imp: __sdk::TableHandle<UniqueI32>,
    ctx: std::marker::PhantomData<&'ctx super::RemoteTables>,
}

#[allow(non_camel_case_types)]
/// Extension trait for access to the table `unique_i32`.
///
/// Implemented for [`super::RemoteTables`].
pub trait UniqueI32TableAccess {
    #[allow(non_snake_case)]
    /// Obtain a [`UniqueI32TableHandle`], which mediates access to the table `unique_i32`.
    fn unique_i_32(&self) -> UniqueI32TableHandle<'_>;
}

impl UniqueI32TableAccess for super::RemoteTables {
    fn unique_i_32(&self) -> UniqueI32TableHandle<'_> {
        UniqueI32TableHandle {
            imp: self.imp.get_table::<UniqueI32>("unique_i32"),
            ctx: std::marker::PhantomData,
        }
    }
}

pub struct UniqueI32InsertCallbackId(__sdk::CallbackId);
pub struct UniqueI32DeleteCallbackId(__sdk::CallbackId);

impl<'ctx> __sdk::Table for UniqueI32TableHandle<'ctx> {
    type Row = UniqueI32;
    type EventContext = super::EventContext;

    fn count(&self) -> u64 {
        self.imp.count()
    }
    fn iter(&self) -> impl Iterator<Item = UniqueI32> + '_ {
        self.imp.iter()
    }

    type InsertCallbackId = UniqueI32InsertCallbackId;

    fn on_insert(
        &self,
        callback: impl FnMut(&Self::EventContext, &Self::Row) + Send + 'static,
    ) -> UniqueI32InsertCallbackId {
        UniqueI32InsertCallbackId(self.imp.on_insert(Box::new(callback)))
    }

    fn remove_on_insert(&self, callback: UniqueI32InsertCallbackId) {
        self.imp.remove_on_insert(callback.0)
    }

    type DeleteCallbackId = UniqueI32DeleteCallbackId;

    fn on_delete(
        &self,
        callback: impl FnMut(&Self::EventContext, &Self::Row) + Send + 'static,
    ) -> UniqueI32DeleteCallbackId {
        UniqueI32DeleteCallbackId(self.imp.on_delete(Box::new(callback)))
    }

    fn remove_on_delete(&self, callback: UniqueI32DeleteCallbackId) {
        self.imp.remove_on_delete(callback.0)
    }
}

#[doc(hidden)]
pub(super) fn parse_table_update(
    raw_updates: __ws::TableUpdate<__ws::BsatnFormat>,
) -> __anyhow::Result<__sdk::TableUpdate<UniqueI32>> {
    __sdk::TableUpdate::parse_table_update_no_primary_key(raw_updates)
        .context("Failed to parse table update for table \"unique_i32\"")
}

/// Access to the `n` unique index on the table `unique_i32`,
/// which allows point queries on the field of the same name
/// via the [`UniqueI32NUnique::find`] method.
///
/// Users are encouraged not to explicitly reference this type,
/// but to directly chain method calls,
/// like `ctx.db.unique_i_32().n().find(...)`.
pub struct UniqueI32NUnique<'ctx> {
    imp: __sdk::UniqueConstraintHandle<UniqueI32, i32>,
    phantom: std::marker::PhantomData<&'ctx super::RemoteTables>,
}

impl<'ctx> UniqueI32TableHandle<'ctx> {
    /// Get a handle on the `n` unique index on the table `unique_i32`.
    pub fn n(&self) -> UniqueI32NUnique<'ctx> {
        UniqueI32NUnique {
            imp: self.imp.get_unique_constraint::<i32>("n", |row| &row.n),
            phantom: std::marker::PhantomData,
        }
    }
}

impl<'ctx> UniqueI32NUnique<'ctx> {
    /// Find the subscribed row whose `n` column value is equal to `col_val`,
    /// if such a row is present in the client cache.
    pub fn find(&self, col_val: &i32) -> Option<UniqueI32> {
        self.imp.find(col_val)
    }
}
