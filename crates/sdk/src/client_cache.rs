use crate::callbacks::DbCallbacks;
use crate::client_api_messages;
use crate::reducer::AnyReducerEvent;
use crate::spacetime_module::SpacetimeModule;
use crate::table::{TableType, TableWithPrimaryKey};
use anymap::{
    any::{Any, CloneAny},
    Map,
};
use hashbrown::HashMap as StdHashMap;
use im::HashMap;
use spacetimedb_sats::bsatn;
use std::sync::Arc;

/// A local mirror of the subscribed rows of one table in the database.
///
/// `T` should be a `TableType`.
///
/// References to this struct are autogenerated in the `handle_table_update` and
/// `handle_resubscribe` functions. Users should not reference this struct directly.
#[derive(Clone)]
pub struct TableCache<T: TableType> {
    /// A map of row-bytes to rows.
    ///
    /// The keys are BSATN-serialized representations of the values.
    /// Storing both the bytes and the deserialized rows allows us to have a `HashMap`
    /// even when `T` is not `Hash + Eq`, e.g. for row types which contain floats.
    /// We also suspect that hashing and equality comparisons for byte arrays
    /// are more efficient than for domain types,
    /// as they can be implemented directly via SIMD without skipping padding
    /// or branching on enum variants.
    ///
    /// Note that this is an [`im::HashMap`], and so can be shared efficiently.
    entries: HashMap<Vec<u8>, T>,
}

// In order to be resilient against future extensions to the protocol,
// Protobuf/Prost does not deserialize `enum` fields directly into a Rust `enum`.
// Instead, it leaves the message field as an `i32`,
// which we must then compare against the enum variants.
// These helper functions do that comparison.

/// Is `op` the `Delete` operation?
///
/// `op` will be the `op` field of a `client_api_messages::TableRowOperation`.
fn op_is_delete(op: i32) -> bool {
    (client_api_messages::table_row_operation::OperationType::Delete as i32) == op
}

/// Is `op` the `Insert` operation?
///
/// `op` will be the `op` field of a `client_api_messages::TableRowOperation`.
fn op_is_insert(op: i32) -> bool {
    (client_api_messages::table_row_operation::OperationType::Insert as i32) == op
}

impl<T: TableType> TableCache<T> {
    /// Returns the number of rows resident in the client cache for this `TableType`,
    /// i.e. the number of subscribed rows.
    pub(crate) fn count_subscribed_rows(&self) -> usize {
        self.entries.len()
    }

    /// Insert `value` into the cache and invoke any on-insert callbacks.
    ///
    /// `row_bytes` should be the BSATN-serialized representation of `value`.
    /// It will be used as the key in a `HashMap`.
    fn insert(&mut self, callbacks: &mut Vec<RowCallback<T>>, row_bytes: Vec<u8>, value: T) {
        callbacks.push(RowCallback::Insert(value.clone()));

        if self.entries.insert(row_bytes, value).is_some() {
            log::warn!("Inserting a row already presint in table {:?}", T::TABLE_NAME);
        }
    }

    /// Delete `value` from the cache and invoke any on-delete callbacks.
    ///
    /// `row_bytes` should be the BSATN-serialized representation of `value`.
    /// It will be used as the key in a `HashMap`.
    fn delete(&mut self, callbacks: &mut Vec<RowCallback<T>>, row_bytes: Vec<u8>, value: T) {
        callbacks.push(RowCallback::Delete(value));

        if self.entries.remove(&row_bytes).is_none() {
            log::error!(
                "Received delete for table {:?} row we weren't subscribed to",
                T::TABLE_NAME
            );
        };
    }

    fn new() -> TableCache<T> {
        TableCache {
            entries: HashMap::new(),
        }
    }

    /// Decode an instance of `T`, i.e. a row, from the `row` field of the `row_op`, and
    /// dispatch on the `op` field of the `row_op` to determine the appropriate action:
    /// `self.delete` or `self.insert`.
    fn handle_row_update(
        &mut self,
        callbacks: &mut Vec<RowCallback<T>>,
        row_op: client_api_messages::TableRowOperation,
    ) {
        let client_api_messages::TableRowOperation { op, row } = row_op;
        match bsatn::from_slice(&row) {
            Err(e) => {
                log::error!(
                    "Error while deserializing row from TableRowOperation: {:?}. Row is {:?}",
                    e,
                    row
                );
            }
            Ok(value) => {
                if op_is_delete(op) {
                    log::trace!("Got delete event for {:?} row {:?}", T::TABLE_NAME, value);
                    self.delete(callbacks, row, value);
                } else if op_is_insert(op) {
                    log::trace!("Got insert event for {:?} row {:?}", T::TABLE_NAME, value);
                    self.insert(callbacks, row, value);
                } else {
                    log::error!("Unknown table_row_operation::OperationType {}", op);
                }
            }
        }
    }

    /// For each `TableRowOperation` in the `table_update`, insert or remove the row into
    /// or from the cache as appropriate. Do not handle primary keys, and do not generate
    /// `on_update` methods.
    fn handle_table_update_no_primary_key(
        &mut self,
        callbacks: &mut Vec<RowCallback<T>>,
        table_update: client_api_messages::TableUpdate,
    ) {
        for row_update in table_update.table_row_operations.into_iter() {
            self.handle_row_update(callbacks, row_update);
        }
    }

    // Implementing iteration over the client cache stinks.
    //
    // We want to provide `fn iter() -> impl Iterator<Item = Self>` (or `&Self`, or whatever)
    // as a method on the spacetimedb_client_sdk::TableType trait, analogous to the method
    // on spacetimedb_bindings::TableType.
    //
    // On the client, a table is effectively a global `Mutex<HashMap<_, T: TableType>>`.
    //
    // This means that the iterator returned by `iter` must hold a `MutexGuard` on the
    // table alongside a `HashMap` iterator into that mutex guard, i.e.:
    //
    // ```
    // struct TableIter<T> {
    //   lock: MutexGuard<'static, HashMap<Vec<u8>, T>>,
    //   iter: Cloned<Values<Vec<u8>, T>>,
    // }
    // ```
    //
    // But `iter` borrows from `lock`, and rustc doesn't like self-referential
    // structs. There's no actual interior pointer into `lock` involved, just a pointer to
    // the `HashMap` derived from `lock`, but rustc doesn't know that. We could hack
    // around this using `unsafe`, or [Rental](https://crates.io/crates/rental), but
    // Rental is unmaintained and `unsafe` is scary. Mazdak wrote a [sample version using
    // self_cell](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=e0eba2569b1cbfb00a646cdb26b737a9),
    // but it's can't be generic over `T`. So until we figure out a better way, we just
    // copy out of the `HashMap` into a `Vec`, and `into_iter` that.
    pub(crate) fn values(&self) -> Vec<T> {
        self.entries.values().cloned().collect()
    }

    pub(crate) fn filter(&self, mut test: impl FnMut(&T) -> bool) -> Vec<T> {
        self.entries.values().filter(|&t| test(t)).cloned().collect()
    }

    pub(crate) fn find(&self, mut test: impl FnMut(&T) -> bool) -> Option<T> {
        self.entries.values().find(|&t| test(t)).cloned()
    }

    /// For each previously-subscribed row not in the `new_subs`, delete it from the cache
    /// and issue an `on_delete` event for it. For each new row in the `new_subs` not
    /// already in the cache, add it to the cache and issue an `on_insert` event for it.
    fn reinitialize_for_new_subscribed_set(
        &mut self,
        callbacks: &mut Vec<RowCallback<T>>,
        new_subs: client_api_messages::TableUpdate,
    ) {
        // TODO: there should be a fast path where `self` is empty prior to this
        //       operation, where we avoid building a diff and just insert all the
        //       `new_subs`.
        enum DiffEntry<T> {
            Insert(T),
            Delete(T),
            NoChange(T),
        }

        let prev_subs = std::mem::take(&mut self.entries);

        let mut diff: StdHashMap<Vec<u8>, DiffEntry<T>> = StdHashMap::with_capacity(
            // pre-allocate plenty of space to avoid hash conflicts
            (new_subs.table_row_operations.len() + prev_subs.len()) * 2,
        );

        for (row_bytes, value) in prev_subs.into_iter() {
            log::trace!(
                "Initalizing table {:?}: row previously resident: {:?}",
                T::TABLE_NAME,
                value,
            );
            if diff.insert(row_bytes, DiffEntry::Delete(value)).is_some() {
                // This should be impossible, but just in case...
                log::error!("Found duplicate row in existing `TableCache` for {:?}", T::TABLE_NAME);
            }
        }

        for row_op in new_subs.table_row_operations.into_iter() {
            let client_api_messages::TableRowOperation { op, row: row_bytes } = row_op;

            if !op_is_insert(op) {
                log::error!(
                    "Received non-`Insert` `TableRowOperation` for {:?} in new set",
                    T::TABLE_NAME,
                );
                continue;
            }

            // TODO: It would be cool to be able to do this with the `HashMap::entry` api,
            //       but I couldn't get the upgrade branch (i.e. the `DiffEntry::Delete`
            //       pattern) to borrowck, since `Entry` only provides `and_modify`, not
            //       `map`, and we need to move the row of the `Delete` into the
            //       `NoChange`.
            match diff.remove(&row_bytes) {
                None => match bsatn::from_slice(&row_bytes) {
                    Err(e) => {
                        log::error!(
                            "Error while deserializing row from `TableRowOperation`: {:?}. Row is {:?}",
                            e,
                            row_bytes,
                        );
                        continue;
                    }
                    Ok(row) => {
                        log::trace!("Initializing table {:?}: got new row {:?}.", T::TABLE_NAME, row);
                        diff.insert(row_bytes, DiffEntry::Insert(row));
                    }
                },

                Some(diff_entry @ (DiffEntry::Insert(_) | DiffEntry::NoChange(_))) => {
                    log::warn!("Received duplicate `Insert` for {:?} in new set", T::TABLE_NAME);
                    diff.insert(row_bytes, diff_entry);
                }
                Some(DiffEntry::Delete(row)) => {
                    log::trace!("Initializing table {:?}: row {:?} remains present", T::TABLE_NAME, row);
                    diff.insert(row_bytes, DiffEntry::NoChange(row));
                }
            };
        }

        for (row_bytes, diff_entry) in diff.into_iter() {
            match diff_entry {
                DiffEntry::Delete(row) => {
                    // Invoke `on_delete` callbacks; the row was previously resident but
                    // is going away.
                    callbacks.push(RowCallback::Delete(row));
                }
                DiffEntry::NoChange(row) => {
                    // Insert into the new cache table, but do not invoke `on_insert`
                    // callbacks; the row was already resident.
                    self.entries.insert(row_bytes, row);
                }
                DiffEntry::Insert(row) => {
                    // Insert into the new cache table and invoke `on_insert` callbacks;
                    // the row is new.
                    self.insert(callbacks, row_bytes, row);
                }
            }
        }
    }
}

impl<T: TableWithPrimaryKey> TableCache<T> {
    /// Generate a diff from the `TableRowOperation`s in the `table_update` in order to
    /// merge `delete` and `insert` operations into `update`s, then perform the operations
    /// specified in the diff and invoke callbacks as appropriate.
    fn handle_table_update_with_primary_key(
        &mut self,
        callbacks: &mut Vec<RowCallback<T>>,
        table_update: client_api_messages::TableUpdate,
    ) {
        log::trace!("Handling TableUpdate for table {:?} with primary key", T::TABLE_NAME);

        enum DiffEntry<T> {
            Insert(Vec<u8>, T),
            Delete(Vec<u8>, T),
            Update {
                old: T,
                old_bytes: Vec<u8>,
                new: T,
                new_bytes: Vec<u8>,
            },
        }

        fn merge_diff_entries<T: std::fmt::Debug>(left: DiffEntry<T>, right: Option<DiffEntry<T>>) -> DiffEntry<T> {
            match (left, right) {
                (left, None) => left,
                (_, Some(u @ DiffEntry::Update { .. })) => {
                    log::warn!("Received a third `TableRowOperation` for a row which already has an `Update` within one `TableUpdate`");
                    u
                }
                (DiffEntry::Insert(new_bytes, new), Some(DiffEntry::Delete(old_bytes, old)))
                | (DiffEntry::Delete(old_bytes, old), Some(DiffEntry::Insert(new_bytes, new))) => DiffEntry::Update {
                    old,
                    old_bytes,
                    new,
                    new_bytes,
                },
                (DiffEntry::Insert(left_bytes, left), Some(DiffEntry::Insert(_, right))) => {
                    log::warn!(
                        "Received duplicate insert operations for a row within one `TableUpdate`: {:?}; {:?}",
                        left,
                        right,
                    );
                    DiffEntry::Insert(left_bytes, left)
                }
                (DiffEntry::Delete(left_bytes, left), Some(DiffEntry::Delete(_, right))) => {
                    log::warn!(
                        "Received duplicate delete operations for a row within one `TableUpdate`: {:?}; {:?}",
                        left,
                        right,
                    );
                    DiffEntry::Delete(left_bytes, left)
                }
                (DiffEntry::Update { .. }, _) => unreachable!(),
            }
        }

        fn parse_diff_entry<T: TableWithPrimaryKey>(
            client_api_messages::TableRowOperation { op, row: row_bytes }: client_api_messages::TableRowOperation,
        ) -> Option<DiffEntry<T>> {
            match bsatn::from_slice(&row_bytes) {
                Err(e) => {
                    log::error!(
                        "Error while deserializing row from `TableRowOperation`: {:?}. Row is {:?}",
                        e,
                        row_bytes,
                    );
                    None
                }
                Ok(row) => {
                    if op_is_delete(op) {
                        log::trace!("Got delete event for {:?} row {:?}", T::TABLE_NAME, row);
                        Some(DiffEntry::Delete(row_bytes, row))
                    } else if op_is_insert(op) {
                        log::trace!("Got insert event for {:?} row {:?}", T::TABLE_NAME, row);
                        Some(DiffEntry::Insert(row_bytes, row))
                    } else {
                        log::error!("Unknown table_row_operation::OperationType {}", op);
                        None
                    }
                }
            }
        }

        fn primary_key<T: TableWithPrimaryKey>(entry: &DiffEntry<T>) -> &T::PrimaryKey {
            match entry {
                DiffEntry::Insert(_, new) => new.primary_key(),
                DiffEntry::Delete(_, old) => old.primary_key(),
                DiffEntry::Update { new, .. } => new.primary_key(),
            }
        }

        let mut diff: StdHashMap<T::PrimaryKey, DiffEntry<T>> = StdHashMap::with_capacity(
            // Pre-allocate plenty of space to minimize hash collisions.
            table_update.table_row_operations.len() * 2,
        );

        // Traverse the `table_update` to construct a diff, merging duplicated `Insert`
        // and `Delete` into `Update`.
        for row_op in table_update.table_row_operations.into_iter() {
            if let Some(diff_entry) = parse_diff_entry(row_op) {
                let pk: T::PrimaryKey = <T::PrimaryKey as Clone>::clone(primary_key(&diff_entry));
                let existing_entry = diff.remove(&pk);
                let new_entry = merge_diff_entries(diff_entry, existing_entry);
                diff.insert(pk, new_entry);
            }
        }

        // Apply the `diff`.
        for diff_entry in diff.into_values() {
            match diff_entry {
                DiffEntry::Insert(row_bytes, row) => self.insert(callbacks, row_bytes, row),
                DiffEntry::Delete(row_bytes, row) => self.delete(callbacks, row_bytes, row),
                DiffEntry::Update {
                    new,
                    new_bytes,
                    old,
                    old_bytes,
                } => self.update(callbacks, old_bytes, old, new_bytes, new),
            }
        }
    }

    /// Remove `old` from the cache and replace it with `new`,
    /// and invoke any on-update callbacks.
    fn update(&mut self, callbacks: &mut Vec<RowCallback<T>>, old_bytes: Vec<u8>, old: T, new_bytes: Vec<u8>, new: T) {
        callbacks.push(RowCallback::Update(old, new.clone()));

        if self.entries.remove(&old_bytes).is_none() {
            log::warn!(
                "Received update for not previously resident row in table {:?}",
                T::TABLE_NAME,
            );
        }
        if self.entries.insert(new_bytes, new).is_some() {
            log::warn!(
                "Received update with already present new row in table {:?}",
                T::TABLE_NAME
            );
        }
    }
}

/// A single row callback saved in a `RowCallbackReminders`,
/// to be run after applying all row updates in the transaction.
enum RowCallback<T> {
    Insert(T),
    Delete(T),
    Update(T, T),
}

/// A collection of `RowCallback`s for each table,
/// accumulated during a transaction.
///
/// When invoking a callback, we need to pass a `db_state: Arc<ClientCache>`
/// as that callback's view of the database.
/// That `db_state` is the result of applying all of the row updates in a `SubscriptionUpdate`,
/// so we can't have it ready to go already at the point when we apply an individual row update.
/// Whenever we apply a row update, rather than immediately invoking the appropriate callback,
/// we add the callback to the transaction's `RowCallbackReminder`,
/// and then invoke all of them afterwards, once the post-transaction state is ready.
///
/// References to this struct are autogenerated by SpacetimeDB to handle messages from the database.
/// Users should not reference this struct directly.
pub struct RowCallbackReminders {
    /// "keyed" on the type `Vec<RowCallback<T>> where T: TableType`.
    /// For each table, a vec of row callbacks accumulated during this transaction.
    table_row_callbacks: Map<dyn Any>,
}

impl RowCallbackReminders {
    /// Construct a `RowCallbackReminder` with capacity
    /// appropriate for the number of table updates in `subs`.
    pub(crate) fn new_for_subscription_update(subs: &client_api_messages::SubscriptionUpdate) -> RowCallbackReminders {
        RowCallbackReminders {
            table_row_callbacks: Map::with_capacity(subs.table_updates.len()),
        }
    }

    fn find_table_callback_reminders<T: TableType>(&mut self) -> &mut Vec<RowCallback<T>> {
        self.table_row_callbacks
            .entry::<Vec<RowCallback<T>>>()
            // TODO: Vec::with_capacity equal to the number of row updates in the `TableUpdate`.
            .or_insert(Vec::new())
    }

    /// Drain all of the callback reminders for `T` from `self` and pass them to `callbacks` for invocation.
    ///
    /// Calls to this method are autogenerated by SpacetimeDB with appropriate generic parameters
    /// in the `invoke_row_callbacks` function in `mod.rs`.
    /// Users should not call this method directly.
    pub fn invoke_callbacks<T: TableType>(
        &mut self,
        callbacks: &mut DbCallbacks,
        reducer_event: &Option<Arc<AnyReducerEvent>>,
        db_state: &ClientCacheView,
    ) {
        if let Some(callback_reminders) = self.table_row_callbacks.remove::<Vec<RowCallback<T>>>() {
            let table_callbacks = callbacks.find_table::<T>();
            for callback in callback_reminders.into_iter() {
                let db_state_handle = db_state.clone();
                match callback {
                    RowCallback::Insert(row) => {
                        table_callbacks.invoke_on_insert(row, reducer_event.clone(), db_state_handle)
                    }
                    RowCallback::Delete(row) => {
                        table_callbacks.invoke_on_delete(row, reducer_event.clone(), db_state_handle)
                    }
                    RowCallback::Update(old, new) => {
                        table_callbacks.invoke_on_update(old, new, reducer_event.clone(), db_state_handle)
                    }
                }
            }
        }
    }
}

/// A local mirror of the subscribed subset of the database.
///
/// References to this struct are autogenerated in the `handle_row_update` function.
/// Users should not reference this struct directly.
#[derive(Clone)]
pub struct ClientCache {
    /// "keyed" on the type `TableCache<T> where T: TableType`.
    tables: Map<dyn CloneAny + Send + Sync>,

    /// Contains functions autogenerated by the CLI,
    /// which handle dispatching on table names
    /// to select appropriate type parameters for various methods.
    module: Arc<dyn SpacetimeModule>,
}

impl ClientCache {
    /// Look up a type-specific `TableCache` for `T`, creating it if it does not exist.
    pub(crate) fn find_table<T: TableType>(&mut self) -> &mut TableCache<T> {
        self.tables
            .entry::<TableCache<T>>()
            .or_insert_with(|| TableCache::new())
    }

    /// Look up a type-specific `TableCache` for `T`, returning `None` if it does not exist.
    pub(crate) fn get_table<T: TableType>(&self) -> Option<&TableCache<T>> {
        self.tables.get::<TableCache<T>>()
    }

    /// Calls to this method are autogenerated in the `handle_row_update` function,
    /// which handles dispatching on the table's name
    /// to find the appropriate type `T` to `handle_table_update_` with or without `primary_key`.
    /// Users should not call this method directly.
    pub fn handle_table_update_no_primary_key<T: TableType>(
        &mut self,
        callback_reminders: &mut RowCallbackReminders,
        update: client_api_messages::TableUpdate,
    ) {
        let table_cache = self.find_table::<T>();
        let table_callbacks = callback_reminders.find_table_callback_reminders::<T>();
        table_cache.handle_table_update_no_primary_key(table_callbacks, update);
    }

    /// Calls to this method are autogenerated in the `handle_row_update` function,
    /// which handles dispatching on the table's name
    /// to find the appropriate type `T` to `handle_table_update_` with or without `primary_key`.
    /// Users should not call this method directly.
    pub fn handle_table_update_with_primary_key<T: TableWithPrimaryKey>(
        &mut self,
        callback_reminders: &mut RowCallbackReminders,
        update: client_api_messages::TableUpdate,
    ) {
        let table_cache = self.find_table::<T>();
        let table_callbacks = callback_reminders.find_table_callback_reminders::<T>();
        table_cache.handle_table_update_with_primary_key(table_callbacks, update);
    }

    /// Calls to this method are autogenerated in the `handle_resubscribe` function,
    /// which handles dispatching on the table's name
    /// to find the appropriate type `T` to `handle_resubscribe_for_type`.
    /// Users should not call this method directly.
    pub fn handle_resubscribe_for_type<T: TableType>(
        &mut self,
        callback_reminders: &mut RowCallbackReminders,
        new_subs: client_api_messages::TableUpdate,
    ) {
        let table_cache = self.find_table::<T>();
        let table_callbacks = callback_reminders.find_table_callback_reminders::<T>();
        table_cache.reinitialize_for_new_subscribed_set(table_callbacks, new_subs);
    }

    pub(crate) fn new(module: Arc<dyn SpacetimeModule>) -> ClientCache {
        ClientCache {
            tables: Map::new(),
            module,
        }
    }

    /// Invoke the autogenerated `handle_table_update` function
    /// to dispatch on the table name in `table_update`,
    /// and invoke `ClientCache::find_table` with an apprpriate type arg.
    pub(crate) fn handle_table_update(
        &mut self,
        callback_reminders: &mut RowCallbackReminders,
        table_update: client_api_messages::TableUpdate,
    ) {
        self.module
            .clone()
            .handle_table_update(table_update, self, callback_reminders);
    }

    /// Invoke the autogenerated `handle_resubscribe` function
    /// to dispatch on the table name in `new_subs`,
    /// and invoke `ClientCache::find_table` with an appropriate type arg.
    pub(crate) fn handle_table_reinitialize_for_new_subscribed_set(
        &mut self,
        callback_reminders: &mut RowCallbackReminders,
        new_subs: client_api_messages::TableUpdate,
    ) {
        self.module
            .clone()
            .handle_resubscribe(new_subs, self, callback_reminders);
    }

    /// Invoke the autogenerated `invoke_row_callbacks` function
    /// to invoke all callbacks in `callback_reminders`
    /// in the state `self`.
    pub(crate) fn invoke_row_callbacks(
        self: &Arc<Self>,
        callback_reminders: &mut RowCallbackReminders,
        callback_worker: &mut DbCallbacks,
        reducer_event: Option<Arc<AnyReducerEvent>>,
    ) {
        self.module
            .invoke_row_callbacks(callback_reminders, callback_worker, reducer_event, self);
    }
}

/// A shared view into a particular state of the `ClientCache`.
pub(crate) type ClientCacheView = Arc<ClientCache>;
