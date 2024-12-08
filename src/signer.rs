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

//! Wrappers around creating a signer account.

use crate::{
	error::Error,
	prelude::{Pair},
};
use polkadot_sdk::{
	sp_core::Pair as PairT,
};
use subxt::Config;
use crate::prelude::{ZenchainConfig};

/// A [`Signer`] implementation that can be constructed from an [`Pair`].
#[derive(Clone)]
pub struct PairSigner {
	account_id: <ZenchainConfig as  Config>::AccountId,
	signer: Pair,
}

impl PairSigner {
	/// Creates a new [`Signer`] from an [`Pair`].
	pub fn new(signer: Pair) -> Self {
		let account_id = fp_account::AccountId20::from(signer.public());
		let subxt_account_id = subxt::ext::subxt_core::utils::AccountId20(account_id.0);
		Self { account_id: subxt_account_id, signer }
	}

	/// Return the account ID.
	pub fn account_id(&self) -> &<ZenchainConfig as  Config>::AccountId {
		&self.account_id
	}
}

impl subxt::tx::Signer<ZenchainConfig> for PairSigner {
	fn account_id(&self) -> <ZenchainConfig as  Config>::AccountId {
		self.account_id.clone()
	}

	fn address(&self) -> <ZenchainConfig as  Config>::Address {
		self.account_id.clone().into()
	}

	fn sign(&self, signer_payload: &[u8]) -> <ZenchainConfig as  Config>::Signature {
		let signature = self.signer.sign(signer_payload);
		subxt::config::substrate::MultiSignature::Ecdsa(signature.0)
	}
}

// Signer wrapper.
#[derive(Clone)]
pub struct Signer(PairSigner);

impl std::fmt::Display for Signer {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", self.0.account_id())
	}
}

impl Signer {
	pub fn new(mut seed_or_path: &str) -> Result<Self, Error> {
		seed_or_path = seed_or_path.trim();

		let seed = match std::fs::read(seed_or_path) {
			Ok(s) => String::from_utf8(s).map_err(|e| Error::Other(e.to_string()))?,
			Err(_) => seed_or_path.to_string(),
		};

		let seed = seed.trim();
		let pair = Pair::from_string(seed, None).map_err(Error::Crypto)?;
		let signer = PairSigner::new(pair);

		Ok(Self(signer))
	}
}

impl std::ops::Deref for Signer {
	type Target = PairSigner;

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

impl std::ops::DerefMut for Signer {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.0
	}
}
