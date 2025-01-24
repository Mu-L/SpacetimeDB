// THIS FILE IS AUTOMATICALLY GENERATED BY SPACETIMEDB. EDITS TO THIS FILE
// WILL NOT BE SAVED. MODIFY TABLES IN RUST INSTEAD.

#![allow(unused, clippy::all)]
use spacetimedb_sdk::__codegen::{self as __sdk, __lib, __sats, __ws};

pub mod identity_connected_reducer;
pub mod identity_disconnected_reducer;
pub mod init_reducer;
pub mod message_table;
pub mod message_type;
pub mod send_message_reducer;
pub mod set_name_reducer;
pub mod user_table;
pub mod user_type;

pub use identity_connected_reducer::{
    identity_connected, set_flags_for_identity_connected, IdentityConnectedCallbackId,
};
pub use identity_disconnected_reducer::{
    identity_disconnected, set_flags_for_identity_disconnected, IdentityDisconnectedCallbackId,
};
pub use init_reducer::{init, set_flags_for_init, InitCallbackId};
pub use message_table::*;
pub use message_type::Message;
pub use send_message_reducer::{send_message, set_flags_for_send_message, SendMessageCallbackId};
pub use set_name_reducer::{set_flags_for_set_name, set_name, SetNameCallbackId};
pub use user_table::*;
pub use user_type::User;

#[derive(Clone, PartialEq, Debug)]

/// One of the reducers defined by this module.
///
/// Contained within a [`__sdk::ReducerEvent`] in [`EventContext`]s for reducer events
/// to indicate which reducer caused the event.

pub enum Reducer {
    IdentityConnected,
    IdentityDisconnected,
    Init,
    SendMessage { text: String },
    SetName { name: String },
}

impl __sdk::InModule for Reducer {
    type Module = RemoteModule;
}

impl __sdk::Reducer for Reducer {
    fn reducer_name(&self) -> &'static str {
        match self {
            Reducer::IdentityConnected => "identity_connected",
            Reducer::IdentityDisconnected => "identity_disconnected",
            Reducer::Init => "init",
            Reducer::SendMessage { .. } => "send_message",
            Reducer::SetName { .. } => "set_name",
        }
    }
}
impl TryFrom<__ws::ReducerCallInfo<__ws::BsatnFormat>> for Reducer {
    type Error = __sdk::Error;
    fn try_from(value: __ws::ReducerCallInfo<__ws::BsatnFormat>) -> __sdk::Result<Self> {
        match &value.reducer_name[..] {
            "identity_connected" => Ok(
                __sdk::parse_reducer_args::<identity_connected_reducer::IdentityConnectedArgs>(
                    "identity_connected",
                    &value.args,
                )?
                .into(),
            ),
            "identity_disconnected" => Ok(__sdk::parse_reducer_args::<
                identity_disconnected_reducer::IdentityDisconnectedArgs,
            >("identity_disconnected", &value.args)?
            .into()),
            "init" => Ok(__sdk::parse_reducer_args::<init_reducer::InitArgs>("init", &value.args)?.into()),
            "send_message" => Ok(__sdk::parse_reducer_args::<send_message_reducer::SendMessageArgs>(
                "send_message",
                &value.args,
            )?
            .into()),
            "set_name" => {
                Ok(__sdk::parse_reducer_args::<set_name_reducer::SetNameArgs>("set_name", &value.args)?.into())
            }
            unknown => Err(__sdk::InternalError::unknown_name("reducer", unknown, "ReducerCallInfo").into()),
        }
    }
}

#[derive(Default)]
#[allow(non_snake_case)]
#[doc(hidden)]
pub struct DbUpdate {
    message: __sdk::TableUpdate<Message>,
    user: __sdk::TableUpdate<User>,
}

impl TryFrom<__ws::DatabaseUpdate<__ws::BsatnFormat>> for DbUpdate {
    type Error = __sdk::Error;
    fn try_from(raw: __ws::DatabaseUpdate<__ws::BsatnFormat>) -> Result<Self, Self::Error> {
        let mut db_update = DbUpdate::default();
        for table_update in raw.tables {
            match &table_update.table_name[..] {
                "message" => db_update.message = message_table::parse_table_update(table_update)?,
                "user" => db_update.user = user_table::parse_table_update(table_update)?,

                unknown => {
                    return Err(__sdk::InternalError::unknown_name("table", unknown, "DatabaseUpdate").into());
                }
            }
        }
        Ok(db_update)
    }
}

impl __sdk::InModule for DbUpdate {
    type Module = RemoteModule;
}

impl __sdk::DbUpdate for DbUpdate {
    fn apply_to_client_cache(&self, cache: &mut __sdk::ClientCache<RemoteModule>) {
        cache.apply_diff_to_table::<Message>("message", &self.message);
        cache.apply_diff_to_table::<User>("user", &self.user);
    }
    fn invoke_row_callbacks(&self, event: &EventContext, callbacks: &mut __sdk::DbCallbacks<RemoteModule>) {
        callbacks.invoke_table_row_callbacks::<Message>("message", &self.message, event);
        callbacks.invoke_table_row_callbacks::<User>("user", &self.user, event);
    }
}

#[doc(hidden)]
pub struct RemoteModule;

impl __sdk::InModule for RemoteModule {
    type Module = Self;
}

/// The `reducers` field of [`EventContext`] and [`DbConnection`],
/// with methods provided by extension traits for each reducer defined by the module.
pub struct RemoteReducers {
    imp: __sdk::DbContextImpl<RemoteModule>,
}

impl __sdk::InModule for RemoteReducers {
    type Module = RemoteModule;
}

#[doc(hidden)]
/// The `set_reducer_flags` field of [`DbConnection`],
/// with methods provided by extension traits for each reducer defined by the module.
/// Each method sets the flags for the reducer with the same name.
///
/// This type is currently unstable and may be removed without a major version bump.
pub struct SetReducerFlags {
    imp: __sdk::DbContextImpl<RemoteModule>,
}

impl __sdk::InModule for SetReducerFlags {
    type Module = RemoteModule;
}

/// The `db` field of [`EventContext`] and [`DbConnection`],
/// with methods provided by extension traits for each table defined by the module.
pub struct RemoteTables {
    imp: __sdk::DbContextImpl<RemoteModule>,
}

impl __sdk::InModule for RemoteTables {
    type Module = RemoteModule;
}

/// A connection to a remote module, including a materialized view of a subset of the database.
///
/// Connect to a remote module by calling [`DbConnection::builder`]
/// and using the [`__sdk::DbConnectionBuilder`] builder-pattern constructor.
///
/// You must explicitly advance the connection by calling any one of:
///
/// - [`DbConnection::frame_tick`].
/// - [`DbConnection::run_threaded`].
/// - [`DbConnection::run_async`].
/// - [`DbConnection::advance_one_message`].
/// - [`DbConnection::advance_one_message_blocking`].
/// - [`DbConnection::advance_one_message_async`].
///
/// Which of these methods you should call depends on the specific needs of your application,
/// but you must call one of them, or else the connection will never progress.
pub struct DbConnection {
    /// Access to tables defined by the module via extension traits implemented for [`RemoteTables`].
    pub db: RemoteTables,
    /// Access to reducers defined by the module via extension traits implemented for [`RemoteReducers`].
    pub reducers: RemoteReducers,
    #[doc(hidden)]
    /// Access to setting the call-flags of each reducer defined for each reducer defined by the module
    /// via extension traits implemented for [`SetReducerFlags`].
    ///
    /// This type is currently unstable and may be removed without a major version bump.
    pub set_reducer_flags: SetReducerFlags,

    imp: __sdk::DbContextImpl<RemoteModule>,
}

impl __sdk::InModule for DbConnection {
    type Module = RemoteModule;
}

impl __sdk::DbContext for DbConnection {
    type DbView = RemoteTables;
    type Reducers = RemoteReducers;
    type SetReducerFlags = SetReducerFlags;

    fn db(&self) -> &Self::DbView {
        &self.db
    }
    fn reducers(&self) -> &Self::Reducers {
        &self.reducers
    }
    fn set_reducer_flags(&self) -> &Self::SetReducerFlags {
        &self.set_reducer_flags
    }

    fn is_active(&self) -> bool {
        self.imp.is_active()
    }

    fn disconnect(&self) -> __sdk::Result<()> {
        self.imp.disconnect()
    }

    type SubscriptionBuilder = __sdk::SubscriptionBuilder<RemoteModule>;

    fn subscription_builder(&self) -> Self::SubscriptionBuilder {
        __sdk::SubscriptionBuilder::new(&self.imp)
    }

    fn try_identity(&self) -> Option<__sdk::Identity> {
        self.imp.try_identity()
    }
    fn address(&self) -> __sdk::Address {
        self.imp.address()
    }
}

impl DbConnection {
    /// Builder-pattern constructor for a connection to a remote module.
    ///
    /// See [`__sdk::DbConnectionBuilder`] for required and optional configuration for the new connection.
    pub fn builder() -> __sdk::DbConnectionBuilder<RemoteModule> {
        __sdk::DbConnectionBuilder::new()
    }

    /// If any WebSocket messages are waiting, process one of them.
    ///
    /// Returns `true` if a message was processed, or `false` if the queue is empty.
    /// Callers should invoke this message in a loop until it returns `false`
    /// or for as much time is available to process messages.
    ///
    /// Returns an error if the connection is disconnected.
    /// If the disconnection in question was normal,
    ///  i.e. the result of a call to [`__sdk::DbContext::disconnect`],
    /// the returned error will be downcastable to [`__sdk::DisconnectedError`].
    ///
    /// This is a low-level primitive exposed for power users who need significant control over scheduling.
    /// Most applications should call [`Self::frame_tick`] each frame
    /// to fully exhaust the queue whenever time is available.
    pub fn advance_one_message(&self) -> __sdk::Result<bool> {
        self.imp.advance_one_message()
    }

    /// Process one WebSocket message, potentially blocking the current thread until one is received.
    ///
    /// Returns an error if the connection is disconnected.
    /// If the disconnection in question was normal,
    ///  i.e. the result of a call to [`__sdk::DbContext::disconnect`],
    /// the returned error will be downcastable to [`__sdk::DisconnectedError`].
    ///
    /// This is a low-level primitive exposed for power users who need significant control over scheduling.
    /// Most applications should call [`Self::run_threaded`] to spawn a thread
    /// which advances the connection automatically.
    pub fn advance_one_message_blocking(&self) -> __sdk::Result<()> {
        self.imp.advance_one_message_blocking()
    }

    /// Process one WebSocket message, `await`ing until one is received.
    ///
    /// Returns an error if the connection is disconnected.
    /// If the disconnection in question was normal,
    ///  i.e. the result of a call to [`__sdk::DbContext::disconnect`],
    /// the returned error will be downcastable to [`__sdk::DisconnectedError`].
    ///
    /// This is a low-level primitive exposed for power users who need significant control over scheduling.
    /// Most applications should call [`Self::run_async`] to run an `async` loop
    /// which advances the connection when polled.
    pub async fn advance_one_message_async(&self) -> __sdk::Result<()> {
        self.imp.advance_one_message_async().await
    }

    /// Process all WebSocket messages waiting in the queue,
    /// then return without `await`ing or blocking the current thread.
    pub fn frame_tick(&self) -> __sdk::Result<()> {
        self.imp.frame_tick()
    }

    /// Spawn a thread which processes WebSocket messages as they are received.
    pub fn run_threaded(&self) -> std::thread::JoinHandle<()> {
        self.imp.run_threaded()
    }

    /// Run an `async` loop which processes WebSocket messages when polled.
    pub async fn run_async(&self) -> __sdk::Result<()> {
        self.imp.run_async().await
    }
}

impl __sdk::DbConnection for DbConnection {
    fn new(imp: __sdk::DbContextImpl<RemoteModule>) -> Self {
        Self {
            db: RemoteTables { imp: imp.clone() },
            reducers: RemoteReducers { imp: imp.clone() },
            set_reducer_flags: SetReducerFlags { imp: imp.clone() },
            imp,
        }
    }
}

/// A [`DbConnection`] augmented with an [`__sdk::Event`],
/// passed to various callbacks invoked by the SDK.
pub struct EventContext {
    /// Access to tables defined by the module via extension traits implemented for [`RemoteTables`].
    pub db: RemoteTables,
    /// Access to reducers defined by the module via extension traits implemented for [`RemoteReducers`].
    pub reducers: RemoteReducers,
    /// Access to setting the call-flags of each reducer defined for each reducer defined by the module
    /// via extension traits implemented for [`SetReducerFlags`].
    ///
    /// This type is currently unstable and may be removed without a major version bump.
    pub set_reducer_flags: SetReducerFlags,
    /// The event which caused these callbacks to run.
    pub event: __sdk::Event<Reducer>,
    imp: __sdk::DbContextImpl<RemoteModule>,
}

impl __sdk::InModule for EventContext {
    type Module = RemoteModule;
}

impl __sdk::DbContext for EventContext {
    type DbView = RemoteTables;
    type Reducers = RemoteReducers;
    type SetReducerFlags = SetReducerFlags;

    fn db(&self) -> &Self::DbView {
        &self.db
    }
    fn reducers(&self) -> &Self::Reducers {
        &self.reducers
    }
    fn set_reducer_flags(&self) -> &Self::SetReducerFlags {
        &self.set_reducer_flags
    }

    fn is_active(&self) -> bool {
        self.imp.is_active()
    }

    fn disconnect(&self) -> __sdk::Result<()> {
        self.imp.disconnect()
    }

    type SubscriptionBuilder = __sdk::SubscriptionBuilder<RemoteModule>;

    fn subscription_builder(&self) -> Self::SubscriptionBuilder {
        __sdk::SubscriptionBuilder::new(&self.imp)
    }

    fn try_identity(&self) -> Option<__sdk::Identity> {
        self.imp.try_identity()
    }
    fn address(&self) -> __sdk::Address {
        self.imp.address()
    }
}

impl __sdk::EventContext for EventContext {
    fn event(&self) -> &__sdk::Event<Reducer> {
        &self.event
    }
    fn new(imp: __sdk::DbContextImpl<RemoteModule>, event: __sdk::Event<Reducer>) -> Self {
        Self {
            db: RemoteTables { imp: imp.clone() },
            reducers: RemoteReducers { imp: imp.clone() },
            set_reducer_flags: SetReducerFlags { imp: imp.clone() },
            event,
            imp,
        }
    }
}

/// A handle on a subscribed query.
// TODO: Document this better after implementing the new subscription API.
#[derive(Clone)]
pub struct SubscriptionHandle {
    imp: __sdk::SubscriptionHandleImpl<RemoteModule>,
}

impl __sdk::InModule for SubscriptionHandle {
    type Module = RemoteModule;
}

impl __sdk::SubscriptionHandle for SubscriptionHandle {
    fn new(imp: __sdk::SubscriptionHandleImpl<RemoteModule>) -> Self {
        Self { imp }
    }

    /// Returns true if this subscription has been terminated due to an unsubscribe call or an error.
    fn is_ended(&self) -> bool {
        self.imp.is_ended()
    }

    /// Returns true if this subscription has been applied and has not yet been unsubscribed.
    fn is_active(&self) -> bool {
        self.imp.is_active()
    }

    /// Unsubscribe from the query controlled by this `SubscriptionHandle`,
    /// then run `on_end` when its rows are removed from the client cache.
    fn unsubscribe_then(self, on_end: __sdk::OnEndedCallback<RemoteModule>) -> __sdk::Result<()> {
        self.imp.unsubscribe_then(Some(on_end))
    }

    fn unsubscribe(self) -> __sdk::Result<()> {
        self.imp.unsubscribe_then(None)
    }
}

/// Alias trait for a [`__sdk::DbContext`] connected to this module,
/// with that trait's associated types bounded to this module's concrete types.
///
/// Users can use this trait as a boundary on definitions which should accept
/// either a [`DbConnection`] or an [`EventContext`] and operate on either.
pub trait RemoteDbContext:
    __sdk::DbContext<
    DbView = RemoteTables,
    Reducers = RemoteReducers,
    SetReducerFlags = SetReducerFlags,
    SubscriptionBuilder = __sdk::SubscriptionBuilder<RemoteModule>,
>
{
}
impl<
        Ctx: __sdk::DbContext<
            DbView = RemoteTables,
            Reducers = RemoteReducers,
            SetReducerFlags = SetReducerFlags,
            SubscriptionBuilder = __sdk::SubscriptionBuilder<RemoteModule>,
        >,
    > RemoteDbContext for Ctx
{
}

impl __sdk::SpacetimeModule for RemoteModule {
    type DbConnection = DbConnection;
    type EventContext = EventContext;
    type Reducer = Reducer;
    type DbView = RemoteTables;
    type Reducers = RemoteReducers;
    type SetReducerFlags = SetReducerFlags;
    type DbUpdate = DbUpdate;
    type SubscriptionHandle = SubscriptionHandle;

    fn register_tables(client_cache: &mut __sdk::ClientCache<Self>) {
        message_table::register_table(client_cache);
        user_table::register_table(client_cache);
    }
}
