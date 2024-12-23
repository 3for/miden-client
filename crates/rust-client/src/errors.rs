use alloc::{
    string::{String, ToString},
    vec::Vec,
};
use core::fmt;

use miden_objects::{
    accounts::AccountId, crypto::merkle::MerkleError, notes::NoteId, AccountError, AssetError,
    NoteError, TransactionScriptError,
};
use miden_tx::{
    utils::{DeserializationError, HexParseError},
    TransactionExecutorError, TransactionProverError,
};

use crate::{
    notes::NoteScreenerError,
    rpc::RpcError,
    store::{NoteRecordError, StoreError},
    transactions::{TransactionRequestError, TransactionScriptBuilderError},
};

// CLIENT ERROR
// ================================================================================================

/// Errors generated by the client.
#[derive(Debug)]
pub enum ClientError {
    AccountError(AccountError),
    AssetError(AssetError),
    DataDeserializationError(DeserializationError),
    NoteNotFoundOnChain(NoteId),
    HexParseError(HexParseError),
    ImportNewAccountWithoutSeed,
    MerkleError(MerkleError),
    MissingOutputNotes(Vec<NoteId>),
    NoteError(NoteError),
    NoteImportError(String),
    NoteRecordError(NoteRecordError),
    NoConsumableNoteForAccount(AccountId),
    RpcError(RpcError),
    NoteScreenerError(NoteScreenerError),
    StoreError(StoreError),
    TransactionExecutorError(TransactionExecutorError),
    TransactionProvingError(TransactionProverError),
    TransactionRequestError(TransactionRequestError),
    TransactionScriptBuilderError(TransactionScriptBuilderError),
    TransactionScriptError(TransactionScriptError),
}

impl fmt::Display for ClientError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ClientError::AccountError(err) => write!(f, "Account error: {err}"),
            ClientError::AssetError(err) => write!(f, "Asset error: {err}"),
            ClientError::DataDeserializationError(err) => {
                write!(f, "Data deserialization error: {err}")
            },
            ClientError::NoteNotFoundOnChain(note_id) => {
                write!(f, "The note with ID {note_id} doesn't exist in the chain")
            },
            ClientError::HexParseError(err) => write!(f, "Error turning array to Digest: {err}"),
            ClientError::ImportNewAccountWithoutSeed => write!(
                f,
                "Import account error: can't import a new account without its initial seed"
            ),
            ClientError::MerkleError(merkle_error) => {
                write!(f, "Error with merkle path: {merkle_error}")
            },
            ClientError::MissingOutputNotes(note_ids) => {
                write!(
                    f,
                    "Transaction error: The transaction did not produce the expected notes corresponding to Note IDs: {}",
                    note_ids.iter().map(|&id| id.to_hex()).collect::<Vec<_>>().join(", ")
                )
            },
            ClientError::NoConsumableNoteForAccount(account_id) => {
                write!(f, "No consumable note for account ID {}", account_id)
            },
            ClientError::NoteError(err) => write!(f, "Note error: {err}"),
            ClientError::NoteImportError(err) => write!(f, "Error importing note: {err}"),
            ClientError::NoteRecordError(err) => write!(f, "Note record error: {err}"),
            ClientError::RpcError(err) => write!(f, "RPC api error: {err}"),
            ClientError::NoteScreenerError(err) => write!(f, "Note screener error: {err}"),
            ClientError::StoreError(err) => write!(f, "Store error: {err}"),
            ClientError::TransactionExecutorError(err) => {
                write!(f, "Transaction executor error: {err}")
            },
            ClientError::TransactionProvingError(err) => {
                write!(f, "Transaction prover error: {err}")
            },
            ClientError::TransactionRequestError(err) => {
                write!(f, "Transaction request error: {err}")
            },
            ClientError::TransactionScriptBuilderError(err) => {
                write!(f, "Transaction script builder error: {err}")
            },
            ClientError::TransactionScriptError(err) => {
                write!(f, "Transaction script error: {err}")
            },
        }
    }
}

// CONVERSIONS
// ================================================================================================

impl From<AccountError> for ClientError {
    fn from(err: AccountError) -> Self {
        Self::AccountError(err)
    }
}

impl From<DeserializationError> for ClientError {
    fn from(err: DeserializationError) -> Self {
        Self::DataDeserializationError(err)
    }
}

impl From<HexParseError> for ClientError {
    fn from(err: HexParseError) -> Self {
        Self::HexParseError(err)
    }
}

impl From<NoteError> for ClientError {
    fn from(err: NoteError) -> Self {
        Self::NoteError(err)
    }
}

impl From<NoteRecordError> for ClientError {
    fn from(err: NoteRecordError) -> Self {
        Self::NoteRecordError(err)
    }
}

impl From<MerkleError> for ClientError {
    fn from(err: MerkleError) -> Self {
        Self::MerkleError(err)
    }
}

impl From<RpcError> for ClientError {
    fn from(err: RpcError) -> Self {
        Self::RpcError(err)
    }
}

impl From<StoreError> for ClientError {
    fn from(err: StoreError) -> Self {
        Self::StoreError(err)
    }
}

impl From<TransactionExecutorError> for ClientError {
    fn from(err: TransactionExecutorError) -> Self {
        Self::TransactionExecutorError(err)
    }
}

impl From<TransactionProverError> for ClientError {
    fn from(err: TransactionProverError) -> Self {
        Self::TransactionProvingError(err)
    }
}

impl From<NoteScreenerError> for ClientError {
    fn from(err: NoteScreenerError) -> Self {
        Self::NoteScreenerError(err)
    }
}

impl From<TransactionRequestError> for ClientError {
    fn from(err: TransactionRequestError) -> Self {
        Self::TransactionRequestError(err)
    }
}

impl From<ClientError> for String {
    fn from(err: ClientError) -> String {
        err.to_string()
    }
}

impl From<TransactionScriptBuilderError> for ClientError {
    fn from(err: TransactionScriptBuilderError) -> Self {
        Self::TransactionScriptBuilderError(err)
    }
}

// ID PREFIX FETCH ERROR
// ================================================================================================

/// Error when Looking for a specific ID from a partial ID
#[derive(Debug, Eq, PartialEq)]
pub enum IdPrefixFetchError {
    /// No matches were found for the ID prefix
    NoMatch(String),
    /// Multiple entities matched with the ID prefix
    MultipleMatches(String),
}

impl fmt::Display for IdPrefixFetchError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            IdPrefixFetchError::NoMatch(id) => {
                write!(f, "No matches were found with the {id}.")
            },
            IdPrefixFetchError::MultipleMatches(id) => {
                write!(
                    f,
                    "Found more than one element for the provided {id} and only one match is expected."
                )
            },
        }
    }
}
