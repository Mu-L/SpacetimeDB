// THIS FILE IS AUTOMATICALLY GENERATED BY SPACETIMEDB. EDITS TO THIS FILE
// WILL NOT BE SAVED. MODIFY TABLES IN RUST INSTEAD.

#![allow(unused)]
use super::one_bool_type::OneBool;
use spacetimedb_sdk::__codegen::{
    self as __sdk, __lib, __sats, __ws,
    anyhow::{self as __anyhow, Context as _},
};

/// Table handle for the table `one_bool`.
///
/// Obtain a handle from the [`OneBoolTableAccess::one_bool`] method on [`super::RemoteTables`],
/// like `ctx.db.one_bool()`.
///
/// Users are encouraged not to explicitly reference this type,
/// but to directly chain method calls,
/// like `ctx.db.one_bool().on_insert(...)`.
pub struct OneBoolTableHandle<'ctx> {
    imp: __sdk::TableHandle<OneBool>,
    ctx: std::marker::PhantomData<&'ctx super::RemoteTables>,
}

#[allow(non_camel_case_types)]
/// Extension trait for access to the table `one_bool`.
///
/// Implemented for [`super::RemoteTables`].
pub trait OneBoolTableAccess {
    #[allow(non_snake_case)]
    /// Obtain a [`OneBoolTableHandle`], which mediates access to the table `one_bool`.
    fn one_bool(&self) -> OneBoolTableHandle<'_>;
}

impl OneBoolTableAccess for super::RemoteTables {
    fn one_bool(&self) -> OneBoolTableHandle<'_> {
        OneBoolTableHandle {
            imp: self.imp.get_table::<OneBool>("one_bool"),
            ctx: std::marker::PhantomData,
        }
    }
}

pub struct OneBoolInsertCallbackId(__sdk::CallbackId);
pub struct OneBoolDeleteCallbackId(__sdk::CallbackId);

impl<'ctx> __sdk::Table for OneBoolTableHandle<'ctx> {
    type Row = OneBool;
    type EventContext = super::EventContext;

    fn count(&self) -> u64 {
        self.imp.count()
    }
    fn iter(&self) -> impl Iterator<Item = OneBool> + '_ {
        self.imp.iter()
    }

    type InsertCallbackId = OneBoolInsertCallbackId;

    fn on_insert(
        &self,
        callback: impl FnMut(&Self::EventContext, &Self::Row) + Send + 'static,
    ) -> OneBoolInsertCallbackId {
        OneBoolInsertCallbackId(self.imp.on_insert(Box::new(callback)))
    }

    fn remove_on_insert(&self, callback: OneBoolInsertCallbackId) {
        self.imp.remove_on_insert(callback.0)
    }

    type DeleteCallbackId = OneBoolDeleteCallbackId;

    fn on_delete(
        &self,
        callback: impl FnMut(&Self::EventContext, &Self::Row) + Send + 'static,
    ) -> OneBoolDeleteCallbackId {
        OneBoolDeleteCallbackId(self.imp.on_delete(Box::new(callback)))
    }

    fn remove_on_delete(&self, callback: OneBoolDeleteCallbackId) {
        self.imp.remove_on_delete(callback.0)
    }
}

#[doc(hidden)]
pub(super) fn parse_table_update(
    raw_updates: __ws::TableUpdate<__ws::BsatnFormat>,
) -> __anyhow::Result<__sdk::TableUpdate<OneBool>> {
    __sdk::TableUpdate::parse_table_update_no_primary_key(raw_updates)
        .context("Failed to parse table update for table \"one_bool\"")
}
