// THIS FILE IS AUTOMATICALLY GENERATED BY SPACETIMEDB. EDITS TO THIS FILE
// WILL NOT BE SAVED. MODIFY TABLES IN RUST INSTEAD.

#![allow(unused, clippy::all)]
use super::option_i_32_type::OptionI32;
use spacetimedb_sdk::__codegen::{self as __sdk, __lib, __sats, __ws};

/// Table handle for the table `option_i32`.
///
/// Obtain a handle from the [`OptionI32TableAccess::option_i_32`] method on [`super::RemoteTables`],
/// like `ctx.db.option_i_32()`.
///
/// Users are encouraged not to explicitly reference this type,
/// but to directly chain method calls,
/// like `ctx.db.option_i_32().on_insert(...)`.
pub struct OptionI32TableHandle<'ctx> {
    imp: __sdk::TableHandle<OptionI32>,
    ctx: std::marker::PhantomData<&'ctx super::RemoteTables>,
}

#[allow(non_camel_case_types)]
/// Extension trait for access to the table `option_i32`.
///
/// Implemented for [`super::RemoteTables`].
pub trait OptionI32TableAccess {
    #[allow(non_snake_case)]
    /// Obtain a [`OptionI32TableHandle`], which mediates access to the table `option_i32`.
    fn option_i_32(&self) -> OptionI32TableHandle<'_>;
}

impl OptionI32TableAccess for super::RemoteTables {
    fn option_i_32(&self) -> OptionI32TableHandle<'_> {
        OptionI32TableHandle {
            imp: self.imp.get_table::<OptionI32>("option_i32"),
            ctx: std::marker::PhantomData,
        }
    }
}

pub struct OptionI32InsertCallbackId(__sdk::CallbackId);
pub struct OptionI32DeleteCallbackId(__sdk::CallbackId);

impl<'ctx> __sdk::Table for OptionI32TableHandle<'ctx> {
    type Row = OptionI32;
    type EventContext = super::EventContext;

    fn count(&self) -> u64 {
        self.imp.count()
    }
    fn iter(&self) -> impl Iterator<Item = OptionI32> + '_ {
        self.imp.iter()
    }

    type InsertCallbackId = OptionI32InsertCallbackId;

    fn on_insert(
        &self,
        callback: impl FnMut(&Self::EventContext, &Self::Row) + Send + 'static,
    ) -> OptionI32InsertCallbackId {
        OptionI32InsertCallbackId(self.imp.on_insert(Box::new(callback)))
    }

    fn remove_on_insert(&self, callback: OptionI32InsertCallbackId) {
        self.imp.remove_on_insert(callback.0)
    }

    type DeleteCallbackId = OptionI32DeleteCallbackId;

    fn on_delete(
        &self,
        callback: impl FnMut(&Self::EventContext, &Self::Row) + Send + 'static,
    ) -> OptionI32DeleteCallbackId {
        OptionI32DeleteCallbackId(self.imp.on_delete(Box::new(callback)))
    }

    fn remove_on_delete(&self, callback: OptionI32DeleteCallbackId) {
        self.imp.remove_on_delete(callback.0)
    }
}

#[doc(hidden)]
pub(super) fn register_table(client_cache: &mut __sdk::ClientCache<super::RemoteModule>) {
    let _table = client_cache.get_or_make_table::<OptionI32>("option_i32");
}
#[doc(hidden)]
pub(super) fn parse_table_update(
    raw_updates: __ws::TableUpdate<__ws::BsatnFormat>,
) -> __sdk::Result<__sdk::TableUpdate<OptionI32>> {
    __sdk::TableUpdate::parse_table_update_no_primary_key(raw_updates).map_err(|e| {
        __sdk::InternalError::failed_parse("TableUpdate<OptionI32>", "TableUpdate")
            .with_cause(e)
            .into()
    })
}
