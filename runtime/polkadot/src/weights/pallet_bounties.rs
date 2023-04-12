// Copyright (C) Parity Technologies (UK) Ltd.
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
//! Autogenerated weights for `pallet_bounties`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-03-15, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `bm4`, CPU: `Intel(R) Core(TM) i7-7700K CPU @ 4.20GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("polkadot-dev"), DB CACHE: 1024

// Executed Command:
// ./target/production/polkadot
// benchmark
// pallet
// --chain=polkadot-dev
// --steps=50
// --repeat=20
// --pallet=pallet_bounties
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --header=./file_header.txt
// --output=./runtime/polkadot/src/weights/

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `pallet_bounties`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_bounties::WeightInfo for WeightInfo<T> {
	/// Storage: Bounties BountyCount (r:1 w:1)
	/// Proof: Bounties BountyCount (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: Bounties BountyDescriptions (r:0 w:1)
	/// Proof: Bounties BountyDescriptions (max_values: None, max_size: Some(16400), added: 18875, mode: MaxEncodedLen)
	/// Storage: Bounties Bounties (r:0 w:1)
	/// Proof: Bounties Bounties (max_values: None, max_size: Some(177), added: 2652, mode: MaxEncodedLen)
	/// The range of component `d` is `[0, 16384]`.
	fn propose_bounty(d: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `143`
		//  Estimated: `5082`
		// Minimum execution time: 25_262_000 picoseconds.
		Weight::from_parts(26_509_529, 0)
			.saturating_add(Weight::from_parts(0, 5082))
			// Standard Error: 9
			.saturating_add(Weight::from_parts(742, 0).saturating_mul(d.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	/// Storage: Bounties Bounties (r:1 w:1)
	/// Proof: Bounties Bounties (max_values: None, max_size: Some(177), added: 2652, mode: MaxEncodedLen)
	/// Storage: Bounties BountyApprovals (r:1 w:1)
	/// Proof: Bounties BountyApprovals (max_values: Some(1), max_size: Some(402), added: 897, mode: MaxEncodedLen)
	fn approve_bounty() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `235`
		//  Estimated: `5529`
		// Minimum execution time: 12_338_000 picoseconds.
		Weight::from_parts(12_658_000, 0)
			.saturating_add(Weight::from_parts(0, 5529))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: Bounties Bounties (r:1 w:1)
	/// Proof: Bounties Bounties (max_values: None, max_size: Some(177), added: 2652, mode: MaxEncodedLen)
	fn propose_curator() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `255`
		//  Estimated: `3642`
		// Minimum execution time: 10_929_000 picoseconds.
		Weight::from_parts(11_084_000, 0)
			.saturating_add(Weight::from_parts(0, 3642))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Bounties Bounties (r:1 w:1)
	/// Proof: Bounties Bounties (max_values: None, max_size: Some(177), added: 2652, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	fn unassign_curator() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `431`
		//  Estimated: `7235`
		// Minimum execution time: 34_597_000 picoseconds.
		Weight::from_parts(35_211_000, 0)
			.saturating_add(Weight::from_parts(0, 7235))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: Bounties Bounties (r:1 w:1)
	/// Proof: Bounties Bounties (max_values: None, max_size: Some(177), added: 2652, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	fn accept_curator() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `427`
		//  Estimated: `7235`
		// Minimum execution time: 23_681_000 picoseconds.
		Weight::from_parts(23_957_000, 0)
			.saturating_add(Weight::from_parts(0, 7235))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: Bounties Bounties (r:1 w:1)
	/// Proof: Bounties Bounties (max_values: None, max_size: Some(177), added: 2652, mode: MaxEncodedLen)
	/// Storage: ChildBounties ParentChildBounties (r:1 w:0)
	/// Proof: ChildBounties ParentChildBounties (max_values: None, max_size: Some(16), added: 2491, mode: MaxEncodedLen)
	fn award_bounty() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `405`
		//  Estimated: `7123`
		// Minimum execution time: 20_088_000 picoseconds.
		Weight::from_parts(20_361_000, 0)
			.saturating_add(Weight::from_parts(0, 7123))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Bounties Bounties (r:1 w:1)
	/// Proof: Bounties Bounties (max_values: None, max_size: Some(177), added: 2652, mode: MaxEncodedLen)
	/// Storage: System Account (r:3 w:3)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: ChildBounties ChildrenCuratorFees (r:1 w:1)
	/// Proof: ChildBounties ChildrenCuratorFees (max_values: None, max_size: Some(28), added: 2503, mode: MaxEncodedLen)
	/// Storage: Bounties BountyDescriptions (r:0 w:1)
	/// Proof: Bounties BountyDescriptions (max_values: None, max_size: Some(16400), added: 18875, mode: MaxEncodedLen)
	fn claim_bounty() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `769`
		//  Estimated: `15934`
		// Minimum execution time: 73_246_000 picoseconds.
		Weight::from_parts(73_967_000, 0)
			.saturating_add(Weight::from_parts(0, 15934))
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(6))
	}
	/// Storage: Bounties Bounties (r:1 w:1)
	/// Proof: Bounties Bounties (max_values: None, max_size: Some(177), added: 2652, mode: MaxEncodedLen)
	/// Storage: ChildBounties ParentChildBounties (r:1 w:0)
	/// Proof: ChildBounties ParentChildBounties (max_values: None, max_size: Some(16), added: 2491, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: Bounties BountyDescriptions (r:0 w:1)
	/// Proof: Bounties BountyDescriptions (max_values: None, max_size: Some(16400), added: 18875, mode: MaxEncodedLen)
	fn close_bounty_proposed() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `449`
		//  Estimated: `10716`
		// Minimum execution time: 40_039_000 picoseconds.
		Weight::from_parts(40_570_000, 0)
			.saturating_add(Weight::from_parts(0, 10716))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: Bounties Bounties (r:1 w:1)
	/// Proof: Bounties Bounties (max_values: None, max_size: Some(177), added: 2652, mode: MaxEncodedLen)
	/// Storage: ChildBounties ParentChildBounties (r:1 w:0)
	/// Proof: ChildBounties ParentChildBounties (max_values: None, max_size: Some(16), added: 2491, mode: MaxEncodedLen)
	/// Storage: System Account (r:2 w:2)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: Bounties BountyDescriptions (r:0 w:1)
	/// Proof: Bounties BountyDescriptions (max_values: None, max_size: Some(16400), added: 18875, mode: MaxEncodedLen)
	fn close_bounty_active() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `685`
		//  Estimated: `13319`
		// Minimum execution time: 50_308_000 picoseconds.
		Weight::from_parts(50_840_000, 0)
			.saturating_add(Weight::from_parts(0, 13319))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	/// Storage: Bounties Bounties (r:1 w:1)
	/// Proof: Bounties Bounties (max_values: None, max_size: Some(177), added: 2652, mode: MaxEncodedLen)
	fn extend_bounty_expiry() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `291`
		//  Estimated: `3642`
		// Minimum execution time: 16_032_000 picoseconds.
		Weight::from_parts(16_338_000, 0)
			.saturating_add(Weight::from_parts(0, 3642))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Bounties BountyApprovals (r:1 w:1)
	/// Proof: Bounties BountyApprovals (max_values: Some(1), max_size: Some(402), added: 897, mode: MaxEncodedLen)
	/// Storage: Bounties Bounties (r:100 w:100)
	/// Proof: Bounties Bounties (max_values: None, max_size: Some(177), added: 2652, mode: MaxEncodedLen)
	/// Storage: System Account (r:200 w:200)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// The range of component `b` is `[0, 100]`.
	fn spend_funds(b: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0 + b * (297 ±0)`
		//  Estimated: `3867 + b * (7858 ±0)`
		// Minimum execution time: 5_108_000 picoseconds.
		Weight::from_parts(3_020_045, 0)
			.saturating_add(Weight::from_parts(0, 3867))
			// Standard Error: 25_988
			.saturating_add(Weight::from_parts(32_112_772, 0).saturating_mul(b.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().reads((3_u64).saturating_mul(b.into())))
			.saturating_add(T::DbWeight::get().writes(1))
			.saturating_add(T::DbWeight::get().writes((3_u64).saturating_mul(b.into())))
			.saturating_add(Weight::from_parts(0, 7858).saturating_mul(b.into()))
	}
}
