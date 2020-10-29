use bitcoin::Error as BitcoinError;
use jsonrpc_http_server::jsonrpc_core::Error as JsonRpcError;
use parity_scale_codec::Error as CodecError;
use runtime::{substrate_subxt::Error as XtError, Error as RuntimeError};
use std::net::AddrParseError;
use thiserror::Error;
#[derive(Error, Debug)]
pub enum Error {
    #[error("BitcoinError: {0}")]
    BitcoinError(#[from] BitcoinError),
    #[error("RuntimeError: {0}")]
    RuntimeError(#[from] RuntimeError),
    #[error("SubXtError: {0}")]
    SubXtError(#[from] XtError),
    #[error("JsonRpcError: {0}")]
    JsonRpcError(#[from] JsonRpcError),
    #[error("CodecError: {0}")]
    CodecError(#[from] CodecError),
    #[error("AddrParseError: {0}")]
    AddrParseError(#[from] AddrParseError),
}
