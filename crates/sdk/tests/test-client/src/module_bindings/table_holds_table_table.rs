// THIS FILE IS AUTOMATICALLY GENERATED BY SPACETIMEDB. EDITS TO THIS FILE
// WILL NOT BE SAVED. MODIFY TABLES IN YOUR MODULE SOURCE CODE INSTEAD.

#![allow(unused, clippy::all)]
use super::one_u_8_type::OneU8;
use super::table_holds_table_type::TableHoldsTable;
use super::vec_u_8_type::VecU8;
use spacetimedb_sdk::__codegen::{self as __sdk, __lib, __sats, __ws};

/// Table handle for the table `table_holds_table`.
///
/// Obtain a handle from the [`TableHoldsTableTableAccess::table_holds_table`] method on [`super::RemoteTables`],
/// like `ctx.db.table_holds_table()`.
///
/// Users are encouraged not to explicitly reference this type,
/// but to directly chain method calls,
/// like `ctx.db.table_holds_table().on_insert(...)`.
pub struct TableHoldsTableTableHandle<'ctx> {
    imp: __sdk::TableHandle<TableHoldsTable>,
    ctx: std::marker::PhantomData<&'ctx super::RemoteTables>,
}

#[allow(non_camel_case_types)]
/// Extension trait for access to the table `table_holds_table`.
///
/// Implemented for [`super::RemoteTables`].
pub trait TableHoldsTableTableAccess {
    #[allow(non_snake_case)]
    /// Obtain a [`TableHoldsTableTableHandle`], which mediates access to the table `table_holds_table`.
    fn table_holds_table(&self) -> TableHoldsTableTableHandle<'_>;
}

impl TableHoldsTableTableAccess for super::RemoteTables {
    fn table_holds_table(&self) -> TableHoldsTableTableHandle<'_> {
        TableHoldsTableTableHandle {
            imp: self.imp.get_table::<TableHoldsTable>("table_holds_table"),
            ctx: std::marker::PhantomData,
        }
    }
}

pub struct TableHoldsTableInsertCallbackId(__sdk::CallbackId);
pub struct TableHoldsTableDeleteCallbackId(__sdk::CallbackId);

impl<'ctx> __sdk::Table for TableHoldsTableTableHandle<'ctx> {
    type Row = TableHoldsTable;
    type EventContext = super::EventContext;

    fn count(&self) -> u64 {
        self.imp.count()
    }
    fn iter(&self) -> impl Iterator<Item = TableHoldsTable> + '_ {
        self.imp.iter()
    }

    type InsertCallbackId = TableHoldsTableInsertCallbackId;

    fn on_insert(
        &self,
        callback: impl FnMut(&Self::EventContext, &Self::Row) + Send + 'static,
    ) -> TableHoldsTableInsertCallbackId {
        TableHoldsTableInsertCallbackId(self.imp.on_insert(Box::new(callback)))
    }

    fn remove_on_insert(&self, callback: TableHoldsTableInsertCallbackId) {
        self.imp.remove_on_insert(callback.0)
    }

    type DeleteCallbackId = TableHoldsTableDeleteCallbackId;

    fn on_delete(
        &self,
        callback: impl FnMut(&Self::EventContext, &Self::Row) + Send + 'static,
    ) -> TableHoldsTableDeleteCallbackId {
        TableHoldsTableDeleteCallbackId(self.imp.on_delete(Box::new(callback)))
    }

    fn remove_on_delete(&self, callback: TableHoldsTableDeleteCallbackId) {
        self.imp.remove_on_delete(callback.0)
    }
}

#[doc(hidden)]
pub(super) fn register_table(client_cache: &mut __sdk::ClientCache<super::RemoteModule>) {
    let _table = client_cache.get_or_make_table::<TableHoldsTable>("table_holds_table");
}
#[doc(hidden)]
pub(super) fn parse_table_update(
    raw_updates: __ws::TableUpdate<__ws::BsatnFormat>,
) -> __sdk::Result<__sdk::TableUpdate<TableHoldsTable>> {
    __sdk::TableUpdate::parse_table_update_no_primary_key(raw_updates).map_err(|e| {
        __sdk::InternalError::failed_parse("TableUpdate<TableHoldsTable>", "TableUpdate")
            .with_cause(e)
            .into()
    })
}
