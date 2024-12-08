// Copyright 2021-2022 Parity Technologies (UK) Ltd.
// This file is part of Polkadot.

// Polkadot is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Polkadot is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Polkadot.  If not, see <http://www.gnu.org/licenses/>.

//! Types that we don't fetch from a particular runtime and just assume that they are constant all
//! of the place.
//!
//! It is actually easy to convert the rest as well, but it'll be a lot of noise in our codebase,
//! needing to sprinkle `any_runtime` in a few extra places.

// re-exports.
pub use polkadot_sdk::{
	pallet_election_provider_multi_phase::{Miner, MinerConfig},
	sp_runtime::traits::{Block as BlockT, Header as HeaderT},
};

use subxt::utils::{MultiAddress, MultiSignature, H256};
use subxt::config::substrate::{BlakeTwo256, SubstrateHeader};
use subxt::config::SubstrateExtrinsicParams;

pub type Signature = MultiSignature;

/// The account id type.
pub type AccountId = subxt::ext::subxt_core::utils::AccountId20;
/// The header type. We re-export it here, but we can easily get it from block as well.
pub type Header = SubstrateHeader<u32, BlakeTwo256>;
/// The header type. We re-export it here, but we can easily get it from block as well.
pub type Hash = H256;
/// Balance type
pub type Balance = u128;

/// Default URI to connect to.
///
/// This will never work on a remote node, so we might as well try a local node.
pub const DEFAULT_URI: &str = "ws://127.0.0.1:9944";
/// Default port to start the prometheus server on.
pub const DEFAULT_PROMETHEUS_PORT: u16 = 9999;
/// The logging target.
pub const LOG_TARGET: &str = "polkadot-staking-miner";
/// The key pair type being used. We "strongly" assume sr25519 for simplicity.
pub type Pair = polkadot_sdk::sp_core::ecdsa::Pair;
/// The accuracy that we use for election computations.
pub type Accuracy = polkadot_sdk::sp_runtime::Perbill;
/// RPC client.
pub type RpcClient = subxt::backend::legacy::LegacyRpcMethods<ZenchainConfig>;
/// Subxt client used by the staking miner on all chains.
pub type ChainClient = subxt::OnlineClient<ZenchainConfig>;
/// Config used by the staking-miner


#[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum ZenchainConfig {}

impl subxt::Config for ZenchainConfig {
	type Hash = H256;
	type AccountId = AccountId;
	type Address = MultiAddress<AccountId, u32>;
	type Signature = Signature;
	type Hasher = BlakeTwo256;
	type Header = SubstrateHeader<u32, BlakeTwo256>;
	type ExtrinsicParams = SubstrateExtrinsicParams<Self>;
	type AssetId = u32;
}

pub type Config = ZenchainConfig;
/// Submission type used by the staking miner.
pub type SignedSubmission<S> =
	polkadot_sdk::pallet_election_provider_multi_phase::SignedSubmission<AccountId, Balance, S>;

#[subxt::subxt(
	runtime_metadata_path = "artifacts/metadata.scale",
	derive_for_all_types = "Clone, Debug, Eq, PartialEq",
	derive_for_type(
		path = "pallet_election_provider_multi_phase::RoundSnapshot",
		derive = "Default"
	),
	substitute_type(
		path = "sp_core::crypto::AccountId32",
		with = "::subxt::ext::subxt_core::utils::AccountId20"
	),
	substitute_type(
		path = "sp_npos_elections::ElectionScore",
		with = "::subxt::utils::Static<polkadot_sdk::sp_npos_elections::ElectionScore>"
	),
	substitute_type(
		path = "pallet_election_provider_multi_phase::Phase<Bn>",
		with = "::subxt::utils::Static<polkadot_sdk::pallet_election_provider_multi_phase::Phase<Bn>>"
	)
)]
pub mod runtime {}

pub static SHARED_CLIENT: once_cell::sync::OnceCell<crate::client::Client> =
	once_cell::sync::OnceCell::new();
