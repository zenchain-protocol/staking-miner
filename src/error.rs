use crate::prelude::sp_core;

#[derive(thiserror::Error, Debug)]
pub enum Error {
	#[error("I/O error: `{0}`")]
	Io(#[from] std::io::Error),
	#[error("RPC error: `{0}`")]
	RpcError(#[from] jsonrpsee::core::Error),
	#[error("subxt error: `{0}`")]
	Subxt(#[from] subxt::Error),
	#[error("Crypto error: `{0:?}`")]
	Crypto(sp_core::crypto::SecretStringError),
	#[error("Incorrect phase")]
	IncorrectPhase,
	#[error("Submission is already submitted")]
	AlreadySubmitted,
	#[error("The account does not exist")]
	AccountDoesNotExists,
	#[error("Submission with better score already exist")]
	BetterScoreExist,
	#[error("Invalid chain: `{0}`, staking-miner supports only polkadot, kusama and westend")]
	InvalidChain(String),
	#[error("Other error: `{0}`")]
	Other(String),
}
