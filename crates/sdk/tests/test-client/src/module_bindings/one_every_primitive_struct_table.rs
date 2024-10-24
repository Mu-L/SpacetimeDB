// THIS FILE IS AUTOMATICALLY GENERATED BY SPACETIMEDB. EDITS TO THIS FILE
// WILL NOT BE SAVED. MODIFY TABLES IN RUST INSTEAD.

#![allow(unused)]
use super::every_primitive_struct_type::EveryPrimitiveStruct;
use super::one_every_primitive_struct_type::OneEveryPrimitiveStruct;
use spacetimedb_sdk::{
    self as __sdk,
    anyhow::{self as __anyhow, Context as _},
    lib as __lib, sats as __sats, ws_messages as __ws,
};

/// Table handle for the table `one_every_primitive_struct`.
///
/// Obtain a handle from the [`OneEveryPrimitiveStructTableAccess::one_every_primitive_struct`] method on [`super::RemoteTables`],
/// like `ctx.db.one_every_primitive_struct()`.
///
/// Users are encouraged not to explicitly reference this type,
/// but to directly chain method calls,
/// like `ctx.db.one_every_primitive_struct().on_insert(...)`.
pub struct OneEveryPrimitiveStructTableHandle<'ctx> {
    imp: __sdk::client_cache::TableHandle<OneEveryPrimitiveStruct>,
    ctx: std::marker::PhantomData<&'ctx super::RemoteTables>,
}

#[allow(non_camel_case_types)]
/// Extension trait for access to the table `one_every_primitive_struct`.
///
/// Implemented for [`super::RemoteTables`].
pub trait OneEveryPrimitiveStructTableAccess {
    #[allow(non_snake_case)]
    /// Obtain a [`OneEveryPrimitiveStructTableHandle`], which mediates access to the table `one_every_primitive_struct`.
    fn one_every_primitive_struct(&self) -> OneEveryPrimitiveStructTableHandle<'_>;
}

impl OneEveryPrimitiveStructTableAccess for super::RemoteTables {
    fn one_every_primitive_struct(&self) -> OneEveryPrimitiveStructTableHandle<'_> {
        OneEveryPrimitiveStructTableHandle {
            imp: self
                .imp
                .get_table::<OneEveryPrimitiveStruct>("one_every_primitive_struct"),
            ctx: std::marker::PhantomData,
        }
    }
}

pub struct OneEveryPrimitiveStructInsertCallbackId(__sdk::callbacks::CallbackId);
pub struct OneEveryPrimitiveStructDeleteCallbackId(__sdk::callbacks::CallbackId);

impl<'ctx> __sdk::table::Table for OneEveryPrimitiveStructTableHandle<'ctx> {
    type Row = OneEveryPrimitiveStruct;
    type EventContext = super::EventContext;

    fn count(&self) -> u64 {
        self.imp.count()
    }
    fn iter(&self) -> impl Iterator<Item = OneEveryPrimitiveStruct> + '_ {
        self.imp.iter()
    }

    type InsertCallbackId = OneEveryPrimitiveStructInsertCallbackId;

    fn on_insert(
        &self,
        callback: impl FnMut(&Self::EventContext, &Self::Row) + Send + 'static,
    ) -> OneEveryPrimitiveStructInsertCallbackId {
        OneEveryPrimitiveStructInsertCallbackId(self.imp.on_insert(Box::new(callback)))
    }

    fn remove_on_insert(&self, callback: OneEveryPrimitiveStructInsertCallbackId) {
        self.imp.remove_on_insert(callback.0)
    }

    type DeleteCallbackId = OneEveryPrimitiveStructDeleteCallbackId;

    fn on_delete(
        &self,
        callback: impl FnMut(&Self::EventContext, &Self::Row) + Send + 'static,
    ) -> OneEveryPrimitiveStructDeleteCallbackId {
        OneEveryPrimitiveStructDeleteCallbackId(self.imp.on_delete(Box::new(callback)))
    }

    fn remove_on_delete(&self, callback: OneEveryPrimitiveStructDeleteCallbackId) {
        self.imp.remove_on_delete(callback.0)
    }
}

#[doc(hidden)]
pub(super) fn parse_table_update(
    raw_updates: __ws::TableUpdate<__ws::BsatnFormat>,
) -> __anyhow::Result<__sdk::spacetime_module::TableUpdate<OneEveryPrimitiveStruct>> {
    __sdk::spacetime_module::TableUpdate::parse_table_update_no_primary_key(raw_updates)
        .context("Failed to parse table update for table \"one_every_primitive_struct\"")
}
