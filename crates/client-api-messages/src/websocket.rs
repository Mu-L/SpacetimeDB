//! Messages sent over the SpacetimeDB WebSocket protocol.
//!
//! Client -> Server messages are encoded as [`ClientMessage`].
//! Server -> Client messages are encoded as [`ServerMessage`].
//!
//! Any changes to this file must be paired with a change to the WebSocket protocol identifiers
//! defined in `crates/client-api/src/routes/subscribe.rs`,
//! and be paired with changes to all of:
//!
//! - The C# SDK.
//! - The TypeScript SDK.
//! - The SpacetimeDB website.
//!
//! Changes to the Rust SDK are not necessarily required, as it depends on this crate
//! rather than using an external mirror of this schema.

use crate::energy::EnergyQuanta;
use crate::timestamp::Timestamp;
use bytes::Bytes;
use bytestring::ByteString;
use core::{
    fmt::Debug,
    ops::{Deref, Range},
};
use enum_as_inner::EnumAsInner;
use smallvec::SmallVec;
use spacetimedb_lib::{Address, Identity};
use spacetimedb_primitives::TableId;
use spacetimedb_sats::{
    bsatn::{self, ToBsatn},
    de::Deserialize,
    ser::{serde::SerializeWrapper, Serialize},
    SpacetimeType,
};
use std::{
    io::{self, Read as _, Write as _},
    sync::Arc,
};

pub trait RowListLen {
    /// Returns the length of the list.
    fn len(&self) -> usize;
    /// Returns whether the list is empty or not.
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl<T, L: Deref<Target = [T]>> RowListLen for L {
    fn len(&self) -> usize {
        self.deref().len()
    }
    fn is_empty(&self) -> bool {
        self.deref().is_empty()
    }
}

/// A format / codec used by the websocket API.
///
/// This can be e.g., BSATN, JSON.
pub trait WebsocketFormat: Sized {
    /// The type used for the encoding of a single item.
    type Single: SpacetimeType + for<'de> Deserialize<'de> + Serialize + Debug + Clone;

    /// The type used for the encoding of a list of items.
    type List: SpacetimeType + for<'de> Deserialize<'de> + Serialize + RowListLen + Debug + Clone + Default;

    /// Encodes the `elems` to a list in the format and also returns the length of the list.
    fn encode_list<R: ToBsatn + Serialize>(elems: impl Iterator<Item = R>) -> (Self::List, u64);

    /// The type used to encode query updates.
    /// This type exists so that some formats, e.g., BSATN, can compress an update.
    type QueryUpdate: SpacetimeType + for<'de> Deserialize<'de> + Serialize + Debug + Clone + Send;

    /// Convert a `QueryUpdate` into `Self::QueryUpdate`.
    /// This allows some formats to e.g., compress the update.
    fn into_query_update(qu: QueryUpdate<Self>, compression: Compression) -> Self::QueryUpdate;
}

/// Messages sent from the client to the server.
///
/// Parametric over the reducer argument type to enable [`ClientMessage::map_args`].
#[derive(SpacetimeType)]
#[sats(crate = spacetimedb_lib)]
pub enum ClientMessage<Args> {
    /// Request a reducer run.
    CallReducer(CallReducer<Args>),
    /// Register SQL queries on which to receive updates.
    Subscribe(Subscribe),
    /// Send a one-off SQL query without establishing a subscription.
    OneOffQuery(OneOffQuery),
}

impl<Args> ClientMessage<Args> {
    pub fn map_args<Args2>(self, f: impl FnOnce(Args) -> Args2) -> ClientMessage<Args2> {
        match self {
            ClientMessage::CallReducer(CallReducer {
                reducer,
                args,
                request_id,
            }) => ClientMessage::CallReducer(CallReducer {
                reducer,
                args: f(args),
                request_id,
            }),
            ClientMessage::Subscribe(x) => ClientMessage::Subscribe(x),
            ClientMessage::OneOffQuery(x) => ClientMessage::OneOffQuery(x),
        }
    }
}

/// Request a reducer run.
///
/// Parametric over the argument type to enable [`ClientMessage::map_args`].
#[derive(SpacetimeType)]
#[sats(crate = spacetimedb_lib)]
pub struct CallReducer<Args> {
    /// The name of the reducer to call.
    pub reducer: Box<str>,
    /// The arguments to the reducer.
    ///
    /// In the wire format, this will be a [`Bytes`], BSATN or JSON encoded according to the reducer's argument schema
    /// and the enclosing message format.
    pub args: Args,
    /// An identifier for a client request.
    ///
    /// The server will include the same ID in the response [`TransactionUpdate`].
    pub request_id: u32,
}

/// Sent by client to database to register a set of queries, about which the client will
/// receive `TransactionUpdate`s.
///
/// After issuing a `Subscribe` message, the client will receive a single
/// `SubscriptionUpdate` message containing every current row of every table which matches
/// the subscribed queries. Then, after each reducer run which updates one or more
/// subscribed rows, the client will receive a `TransactionUpdate` containing the updates.
///
/// A `Subscribe` message sets or replaces the entire set of queries to which the client
/// is subscribed. If the client is previously subscribed to some set of queries `A`, and
/// then sends a `Subscribe` message to subscribe to a set `B`, afterwards, the client
/// will be subscribed to `B` but not `A`. In this case, the client will receive a
/// `SubscriptionUpdate` containing every existing row that matches `B`, even if some were
/// already in `A`.
#[derive(SpacetimeType)]
#[sats(crate = spacetimedb_lib)]
pub struct Subscribe {
    /// A sequence of SQL queries.
    pub query_strings: Box<[Box<str>]>,
    pub request_id: u32,
}

/// A one-off query submission.
///
/// Query should be a "SELECT * FROM Table WHERE ...". Other types of queries will be rejected.
/// Multiple such semicolon-delimited queries are allowed.
///
/// One-off queries are identified by a client-generated messageID.
/// To avoid data leaks, the server will NOT cache responses to messages based on UUID!
/// It also will not check for duplicate IDs. They are just a way to match responses to messages.
#[derive(SpacetimeType)]
#[sats(crate = spacetimedb_lib)]
pub struct OneOffQuery {
    pub message_id: Box<[u8]>,
    pub query_string: Box<str>,
}

/// The tag recognized by the host and SDKs to mean no compression of a [`ServerMessage`].
pub const SERVER_MSG_COMPRESSION_TAG_NONE: u8 = 0;

/// The tag recognized by the host and SDKs to mean brotli compression  of a [`ServerMessage`].
pub const SERVER_MSG_COMPRESSION_TAG_BROTLI: u8 = 1;

/// The tag recognized by the host and SDKs to mean brotli compression  of a [`ServerMessage`].
pub const SERVER_MSG_COMPRESSION_TAG_GZIP: u8 = 2;

/// Messages sent from the server to the client.
#[derive(SpacetimeType, derive_more::From)]
#[sats(crate = spacetimedb_lib)]
pub enum ServerMessage<F: WebsocketFormat> {
    /// Informs of changes to subscribed rows.
    InitialSubscription(InitialSubscription<F>),
    /// Upon reducer run.
    TransactionUpdate(TransactionUpdate<F>),
    /// After connecting, to inform client of its identity.
    IdentityToken(IdentityToken),
    /// Return results to a one off SQL query.
    OneOffQueryResponse(OneOffQueryResponse<F>),
}

/// Response to [`Subscribe`] containing the initial matching rows.
#[derive(SpacetimeType)]
#[sats(crate = spacetimedb_lib)]
pub struct InitialSubscription<F: WebsocketFormat> {
    /// A [`DatabaseUpdate`] containing only inserts, the rows which match the subscription queries.
    pub database_update: DatabaseUpdate<F>,
    /// An identifier sent by the client in requests.
    /// The server will include the same request_id in the response.
    pub request_id: u32,
    /// The overall time between the server receiving a request and sending the response.
    pub total_host_execution_duration_micros: u64,
}

/// Received by database from client to inform of user's identity, token and client address.
///
/// The database will always send an `IdentityToken` message
/// as the first message for a new WebSocket connection.
/// If the client is re-connecting with existing credentials,
/// the message will include those credentials.
/// If the client connected anonymously,
/// the database will generate new credentials to identify it.
#[derive(SpacetimeType, Debug)]
#[sats(crate = spacetimedb_lib)]
pub struct IdentityToken {
    pub identity: Identity,
    pub token: Box<str>,
    pub address: Address,
}

// TODO: Evaluate if it makes sense for this to also include the
// address of the database this is calling

/// Received by client from database upon a reducer run.
///
/// Clients receive `TransactionUpdate`s only for reducers
/// which update at least one of their subscribed rows,
/// or for their own `Failed` or `OutOfEnergy` reducer invocations.
#[derive(SpacetimeType, Debug)]
#[sats(crate = spacetimedb_lib)]
pub struct TransactionUpdate<F: WebsocketFormat> {
    /// The status of the transaction. Contains the updated rows, if successful.
    pub status: UpdateStatus<F>,
    /// The time when the reducer started, as microseconds since the Unix epoch.
    pub timestamp: Timestamp,
    /// The identity of the user who requested the reducer run. For event-driven and
    /// scheduled reducers, it is the identity of the database owner.
    pub caller_identity: Identity,
    /// The 16-byte address of the user who requested the reducer run.
    /// The all-zeros address is a sentinel which denotes no address.
    /// `init` and `update` reducers will have a `caller_address`
    /// if and only if one was provided to the `publish` HTTP endpoint.
    /// Scheduled reducers will never have a `caller_address`.
    /// Reducers invoked by HTTP will have a `caller_address`
    /// if and only if one was provided to the `call` HTTP endpoint.
    /// Reducers invoked by WebSocket will always have a `caller_address`.
    pub caller_address: Address,
    /// The original CallReducer request that triggered this reducer.
    pub reducer_call: ReducerCallInfo<F>,
    /// The amount of energy credits consumed by running the reducer.
    pub energy_quanta_used: EnergyQuanta,
    /// How long the reducer took to run.
    pub host_execution_duration_micros: u64,
}

/// Contained in a [`TransactionUpdate`], metadata about a reducer invocation.
#[derive(SpacetimeType, Debug)]
#[sats(crate = spacetimedb_lib)]
pub struct ReducerCallInfo<F: WebsocketFormat> {
    /// The name of the reducer that was called.
    ///
    /// NOTE(centril, 1.0): For bandwidth resource constrained clients
    /// this can encourage them to have poor naming of reducers like `a`.
    /// We should consider not sending this at all and instead
    /// having a startup message where the name <-> id bindings
    /// are established between the host and the client.
    pub reducer_name: Box<str>,
    /// The numerical id of the reducer that was called.
    pub reducer_id: u32,
    /// The arguments to the reducer, encoded as BSATN or JSON according to the reducer's argument schema
    /// and the client's requested protocol.
    pub args: F::Single,
    /// An identifier for a client request
    pub request_id: u32,
}

/// The status of a [`TransactionUpdate`].
#[derive(SpacetimeType, Debug)]
#[sats(crate = spacetimedb_lib)]
pub enum UpdateStatus<F: WebsocketFormat> {
    /// The reducer ran successfully and its changes were committed to the database.
    /// The rows altered in the database/ will be recorded in the `DatabaseUpdate`.
    Committed(DatabaseUpdate<F>),
    /// The reducer errored, and any changes it attempted to were rolled back.
    /// This is the error message.
    Failed(Box<str>),
    /// The reducer was interrupted due to insufficient energy/funds,
    /// and any changes it attempted to make were rolled back.
    OutOfEnergy,
}

/// A collection of inserted and deleted rows, contained in a [`TransactionUpdate`] or [`SubscriptionUpdate`].
#[derive(SpacetimeType, Debug, Clone, Default)]
#[sats(crate = spacetimedb_lib)]
pub struct DatabaseUpdate<F: WebsocketFormat> {
    pub tables: Vec<TableUpdate<F>>,
}

impl<F: WebsocketFormat> DatabaseUpdate<F> {
    pub fn is_empty(&self) -> bool {
        self.tables.is_empty()
    }

    pub fn num_rows(&self) -> usize {
        self.tables.iter().map(|t| t.num_rows()).sum()
    }
}

impl<F: WebsocketFormat> FromIterator<TableUpdate<F>> for DatabaseUpdate<F> {
    fn from_iter<T: IntoIterator<Item = TableUpdate<F>>>(iter: T) -> Self {
        DatabaseUpdate {
            tables: iter.into_iter().collect(),
        }
    }
}

/// Part of a [`DatabaseUpdate`] received by client from database for alterations to a single table.
///
/// NOTE(centril): in 0.12 we added `num_rows` and `table_name` to the struct.
/// These inflate the size of messages, which for some customers is the wrong default.
/// We might want to consider `v1.spacetimedb.bsatn.lightweight`
#[derive(SpacetimeType, Debug, Clone)]
#[sats(crate = spacetimedb_lib)]
pub struct TableUpdate<F: WebsocketFormat> {
    /// The id of the table. Clients should prefer `table_name`, as it is a stable part of a module's API,
    /// whereas `table_id` may change between runs.
    pub table_id: TableId,
    /// The name of the table.
    ///
    /// NOTE(centril, 1.0): we might want to remove this and instead
    /// tell clients about changes to table_name <-> table_id mappings.
    pub table_name: Box<str>,
    /// The sum total of rows in `self.updates`,
    pub num_rows: u64,
    /// The actual insert and delete updates for this table.
    pub updates: SmallVec<[F::QueryUpdate; 1]>,
}

impl<F: WebsocketFormat> TableUpdate<F> {
    pub fn new(table_id: TableId, table_name: Box<str>, (update, num_rows): (F::QueryUpdate, u64)) -> Self {
        Self {
            table_id,
            table_name,
            num_rows,
            updates: [update].into(),
        }
    }

    pub fn push(&mut self, (update, num_rows): (F::QueryUpdate, u64)) {
        self.updates.push(update);
        self.num_rows += num_rows;
    }

    pub fn num_rows(&self) -> usize {
        self.num_rows as usize
    }
}

#[derive(SpacetimeType, Debug, Clone, EnumAsInner)]
#[sats(crate = spacetimedb_lib)]
pub enum CompressableQueryUpdate<F: WebsocketFormat> {
    Uncompressed(QueryUpdate<F>),
    Brotli(Bytes),
    Gzip(Bytes),
}

impl CompressableQueryUpdate<BsatnFormat> {
    pub fn maybe_decompress(self) -> QueryUpdate<BsatnFormat> {
        match self {
            Self::Uncompressed(qu) => qu,
            Self::Brotli(bytes) => {
                let bytes = brotli_decompress(&bytes).unwrap();
                bsatn::from_slice(&bytes).unwrap()
            }
            Self::Gzip(bytes) => {
                let bytes = gzip_decompress(&bytes).unwrap();
                bsatn::from_slice(&bytes).unwrap()
            }
        }
    }
}

#[derive(SpacetimeType, Debug, Clone)]
#[sats(crate = spacetimedb_lib)]
pub struct QueryUpdate<F: WebsocketFormat> {
    /// When in a [`TransactionUpdate`], the matching rows of this table deleted by the transaction.
    ///
    /// Rows are encoded as BSATN or JSON according to the table's schema
    /// and the client's requested protocol.
    ///
    /// Always empty when in an [`InitialSubscription`].
    pub deletes: F::List,
    /// When in a [`TransactionUpdate`], the matching rows of this table inserted by the transaction.
    /// When in an [`InitialSubscription`], the matching rows of this table in the entire committed state.
    ///
    /// Rows are encoded as BSATN or JSON according to the table's schema
    /// and the client's requested protocol.
    pub inserts: F::List,
}

/// A response to a [`OneOffQuery`].
/// Will contain either one error or some number of response rows.
/// At most one of these messages will be sent in reply to any query.
///
/// The messageId will be identical to the one sent in the original query.
#[derive(SpacetimeType, Debug)]
#[sats(crate = spacetimedb_lib)]
pub struct OneOffQueryResponse<F: WebsocketFormat> {
    pub message_id: Box<[u8]>,
    /// If query compilation or evaluation errored, an error message.
    pub error: Option<Box<str>>,

    /// If query compilation and evaluation succeeded, a set of resulting rows, grouped by table.
    pub tables: Box<[OneOffTable<F>]>,

    /// The total duration of query compilation and evaluation on the server, in microseconds.
    pub total_host_execution_duration_micros: u64,
}

/// A table included as part of a [`OneOffQueryResponse`].
#[derive(SpacetimeType, Debug)]
#[sats(crate = spacetimedb_lib)]
pub struct OneOffTable<F: WebsocketFormat> {
    /// The name of the table.
    pub table_name: Box<str>,
    /// The set of rows which matched the query, encoded as BSATN or JSON according to the table's schema
    /// and the client's requested protocol.
    ///
    /// TODO(centril, 1.0): Evalutate whether we want to conditionally compress these.
    pub rows: F::List,
}

/// Used whenever different formats need to coexist.
#[derive(Debug, Clone)]
pub enum FormatSwitch<B, J> {
    Bsatn(B),
    Json(J),
}

impl<B1, J1> FormatSwitch<B1, J1> {
    /// Zips together two switches.
    pub fn zip_mut<B2, J2>(&mut self, other: FormatSwitch<B2, J2>) -> FormatSwitch<(&mut B1, B2), (&mut J1, J2)> {
        match (self, other) {
            (FormatSwitch::Bsatn(a), FormatSwitch::Bsatn(b)) => FormatSwitch::Bsatn((a, b)),
            (FormatSwitch::Json(a), FormatSwitch::Json(b)) => FormatSwitch::Json((a, b)),
            _ => panic!("format should be the same for both sides of the zip"),
        }
    }
}

#[derive(Clone, Copy, Default, Debug, SpacetimeType)]
#[sats(crate = spacetimedb_lib)]
pub struct JsonFormat;

impl WebsocketFormat for JsonFormat {
    type Single = ByteString;

    type List = Vec<ByteString>;

    fn encode_list<R: ToBsatn + Serialize>(elems: impl Iterator<Item = R>) -> (Self::List, u64) {
        let mut count = 0;
        let list = elems
            .map(|elem| serde_json::to_string(&SerializeWrapper::new(elem)).unwrap().into())
            .inspect(|_| count += 1)
            .collect();
        (list, count)
    }

    type QueryUpdate = QueryUpdate<Self>;

    fn into_query_update(qu: QueryUpdate<Self>, _: Compression) -> Self::QueryUpdate {
        qu
    }
}

#[derive(Clone, Copy, Default, Debug, SpacetimeType)]
#[sats(crate = spacetimedb_lib)]
pub struct BsatnFormat;

impl WebsocketFormat for BsatnFormat {
    type Single = Box<[u8]>;

    type List = BsatnRowList;

    fn encode_list<R: ToBsatn + Serialize>(mut elems: impl Iterator<Item = R>) -> (Self::List, u64) {
        // For an empty list, the size of a row is unknown, so use `RowOffsets`.
        let Some(first) = elems.next() else {
            return (BsatnRowList::row_offsets(), 0);
        };
        // We have at least one row. Determine the static size from that, if available.
        let (mut list, mut scratch) = match first.static_bsatn_size() {
            Some(size) => (BsatnRowListBuilder::fixed(size), Vec::with_capacity(size as usize)),
            None => (BsatnRowListBuilder::row_offsets(), Vec::new()),
        };
        // Add the first element and then the rest.
        // We assume that the schema of rows yielded by `elems` stays the same,
        // so once the size is fixed, it will stay that way.
        let mut count = 0;
        let mut push = |elem: R| {
            elem.to_bsatn_extend(&mut scratch).unwrap();
            list.push(&scratch);
            scratch.clear();
            count += 1;
        };
        push(first);
        for elem in elems {
            push(elem);
        }
        (list.finish(), count)
    }

    type QueryUpdate = CompressableQueryUpdate<Self>;

    fn into_query_update(qu: QueryUpdate<Self>, compression: Compression) -> Self::QueryUpdate {
        let qu_len_would_have_been = bsatn::to_len(&qu).unwrap();

        match decide_compression(qu_len_would_have_been, compression) {
            Compression::None => CompressableQueryUpdate::Uncompressed(qu),
            Compression::Brotli => {
                let bytes = bsatn::to_vec(&qu).unwrap();
                let mut out = Vec::new();
                brotli_compress(&bytes, &mut out);
                CompressableQueryUpdate::Brotli(out.into())
            }
            Compression::Gzip => {
                let bytes = bsatn::to_vec(&qu).unwrap();
                let mut out = Vec::new();
                gzip_compress(&bytes, &mut out);
                CompressableQueryUpdate::Gzip(out.into())
            }
        }
    }
}

/// A specification of either a desired or decided compression algorithm.
#[derive(serde::Deserialize, Default, PartialEq, Eq, Clone, Copy, Hash, Debug)]
pub enum Compression {
    /// No compression ever.
    None,
    /// Compress using brotli if a certain size threshold was met.
    #[default]
    Brotli,
    /// Compress using gzip if a certain size threshold was met.
    Gzip,
}

pub fn decide_compression(len: usize, compression: Compression) -> Compression {
    /// The threshold beyond which we start to compress messages.
    /// 1KiB was chosen without measurement.
    /// TODO(perf): measure!
    const COMPRESS_THRESHOLD: usize = 1024;

    if len > COMPRESS_THRESHOLD {
        compression
    } else {
        Compression::None
    }
}

pub fn brotli_compress(bytes: &[u8], out: &mut Vec<u8>) {
    let reader = &mut &bytes[..];

    // TODO(perf): Compression should depend on message size and type.
    //
    // SubscriptionUpdate messages will typically be quite large,
    // while TransactionUpdate messages will typically be quite small.
    //
    // If we are optimizing for SubscriptionUpdates,
    // we want a large buffer.
    // But if we are optimizing for TransactionUpdates,
    // we probably want to skip compression altogether.
    //
    // For now we choose a reasonable middle ground,
    // which is to compress everything using a 32KB buffer.
    const BUFFER_SIZE: usize = 32 * 1024;
    // Again we are optimizing for compression speed,
    // so we choose the lowest (fastest) level of compression.
    // Experiments on internal workloads have shown compression ratios between 7:1 and 10:1
    // for large `SubscriptionUpdate` messages at this level.
    const COMPRESSION_LEVEL: u32 = 1;
    // The default value for an internal compression parameter.
    // See `BrotliEncoderParams` for more details.
    const LG_WIN: u32 = 22;

    let mut encoder = brotli::CompressorReader::new(reader, BUFFER_SIZE, COMPRESSION_LEVEL, LG_WIN);

    encoder
        .read_to_end(out)
        .expect("Failed to Brotli compress `SubscriptionUpdateMessage`");
}

pub fn brotli_decompress(bytes: &[u8]) -> Result<Vec<u8>, io::Error> {
    let mut decompressed = Vec::new();
    brotli::BrotliDecompress(&mut &bytes[..], &mut decompressed)?;
    Ok(decompressed)
}

pub fn gzip_compress(bytes: &[u8], out: &mut Vec<u8>) {
    let mut encoder = flate2::write::GzEncoder::new(out, flate2::Compression::fast());
    encoder.write_all(bytes).unwrap();
    encoder.finish().expect("Failed to gzip compress `bytes`");
}

pub fn gzip_decompress(bytes: &[u8]) -> Result<Vec<u8>, io::Error> {
    let mut decompressed = Vec::new();
    let _ = flate2::read::GzDecoder::new(bytes).read(&mut decompressed)?;
    Ok(decompressed)
}

type RowSize = u16;
type RowOffset = u64;

/// A packed list of BSATN-encoded rows.
#[derive(SpacetimeType, Debug, Clone)]
#[sats(crate = spacetimedb_lib)]
pub struct BsatnRowList<B = Bytes, I = Arc<[RowOffset]>> {
    /// A size hint about `rows_data`
    /// intended to facilitate parallel decode purposes on large initial updates.
    size_hint: RowSizeHint<I>,
    /// The flattened byte array for a list of rows.
    rows_data: B,
}

impl Default for BsatnRowList {
    fn default() -> Self {
        Self::row_offsets()
    }
}

/// NOTE(centril, 1.0): We might want to add a `None` variant to this
/// where the client has to decode in a loop until `rows_data` has been exhausted.
/// The use-case for this is clients who are bandwidth limited and where every byte counts.
#[derive(SpacetimeType, Debug, Clone)]
#[sats(crate = spacetimedb_lib)]
pub enum RowSizeHint<I> {
    /// Each row in `rows_data` is of the same fixed size as specified here.
    FixedSize(RowSize),
    /// The offsets into `rows_data` defining the boundaries of each row.
    /// Only stores the offset to the start of each row.
    /// The ends of each row is inferred from the start of the next row, or `rows_data.len()`.
    /// The behavior of this is identical to that of `PackedStr`.
    RowOffsets(I),
}

impl<I: AsRef<[RowOffset]>> RowSizeHint<I> {
    fn index_to_range(&self, index: usize, data_end: usize) -> Option<Range<usize>> {
        match self {
            Self::FixedSize(size) => {
                let size = *size as usize;
                let start = index * size;
                if start >= data_end {
                    // We've reached beyond `data_end`,
                    // so this is a row that doesn't exist, so we are beyond the count.
                    return None;
                }
                let end = (index + 1) * size;
                Some(start..end)
            }
            Self::RowOffsets(offsets) => {
                let offsets = offsets.as_ref();
                let start = *offsets.get(index)? as usize;
                // The end is either the start of the next element or the end.
                let end = offsets.get(index + 1).map(|e| *e as usize).unwrap_or(data_end);
                Some(start..end)
            }
        }
    }
}

impl<B: Default, I> BsatnRowList<B, I> {
    pub fn fixed(row_size: RowSize) -> Self {
        Self {
            size_hint: RowSizeHint::FixedSize(row_size),
            rows_data: <_>::default(),
        }
    }

    /// Returns a new empty list using indices
    pub fn row_offsets() -> Self
    where
        I: From<[RowOffset; 0]>,
    {
        Self {
            size_hint: RowSizeHint::RowOffsets([].into()),
            rows_data: <_>::default(),
        }
    }
}

impl<B: AsRef<[u8]>, I: AsRef<[RowOffset]>> RowListLen for BsatnRowList<B, I> {
    /// Returns the length of the row list.
    fn len(&self) -> usize {
        match &self.size_hint {
            RowSizeHint::FixedSize(size) => self.rows_data.as_ref().len() / *size as usize,
            RowSizeHint::RowOffsets(offsets) => offsets.as_ref().len(),
        }
    }
}

impl BsatnRowList {
    /// Returns the element at `index` in the list.
    pub fn get(&self, index: usize) -> Option<Bytes> {
        let data_end = self.rows_data.len();
        let data_range = self.size_hint.index_to_range(index, data_end)?;
        Some(self.rows_data.slice(data_range))
    }
}

/// An iterator over all the elements in a [`BsatnRowList`].
pub struct BsatnRowListIter<'a> {
    list: &'a BsatnRowList,
    index: usize,
}

impl<'a> IntoIterator for &'a BsatnRowList {
    type IntoIter = BsatnRowListIter<'a>;
    type Item = Bytes;
    fn into_iter(self) -> Self::IntoIter {
        BsatnRowListIter { list: self, index: 0 }
    }
}

impl Iterator for BsatnRowListIter<'_> {
    type Item = Bytes;
    fn next(&mut self) -> Option<Self::Item> {
        let index = self.index;
        self.index += 1;
        self.list.get(index)
    }
}

/// A [`BsatnRowList`] that can be added to.
pub type BsatnRowListBuilder = BsatnRowList<Vec<u8>, Vec<RowOffset>>;

impl BsatnRowListBuilder {
    /// Adds `row`, BSATN-encoded to this list.
    #[inline]
    pub fn push(&mut self, row: &[u8]) {
        if let RowSizeHint::RowOffsets(offsets) = &mut self.size_hint {
            offsets.push(self.rows_data.len() as u64);
        }
        self.rows_data.extend_from_slice(row);
    }

    /// Finish the in flight list, throwing away the capability to mutate.
    pub fn finish(self) -> BsatnRowList {
        let Self { size_hint, rows_data } = self;
        let rows_data = rows_data.into();
        let size_hint = match size_hint {
            RowSizeHint::FixedSize(fs) => RowSizeHint::FixedSize(fs),
            RowSizeHint::RowOffsets(ro) => RowSizeHint::RowOffsets(ro.into()),
        };
        BsatnRowList { size_hint, rows_data }
    }
}
