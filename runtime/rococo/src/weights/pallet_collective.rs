// Copyright 2017-2022 Parity Technologies (UK) Ltd.
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
//! Autogenerated weights for `pallet_collective`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-08-19, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `bm6`, CPU: `Intel(R) Core(TM) i7-7700K CPU @ 4.20GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("rococo-dev"), DB CACHE: 1024

// Executed Command:
// ./target/production/polkadot
// benchmark
// pallet
// --chain=rococo-dev
// --steps=50
// --repeat=20
// --pallet=pallet_collective
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --header=./file_header.txt
// --output=./runtime/rococo/src/weights/pallet_collective.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{RefTimeWeight, Weight}};
use sp_std::marker::PhantomData;

/// Weight functions for `pallet_collective`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_collective::WeightInfo for WeightInfo<T> {
	// Storage: Collective Members (r:1 w:1)
	// Storage: Collective Proposals (r:1 w:0)
	// Storage: Collective Voting (r:100 w:100)
	// Storage: Collective Prime (r:0 w:1)
	/// The range of component `m` is `[1, 100]`.
	/// The range of component `n` is `[1, 100]`.
	/// The range of component `p` is `[1, 100]`.
	fn set_members(m: u32, n: u32, p: u32, ) -> Weight {
		Weight::from_ref_time(0 as RefTimeWeight)
			// Standard Error: 11_000
			.saturating_add(Weight::from_ref_time(9_184_000 as RefTimeWeight).saturating_mul(m as RefTimeWeight))
			// Standard Error: 11_000
			.saturating_add(Weight::from_ref_time(10_000 as RefTimeWeight).saturating_mul(n as RefTimeWeight))
			// Standard Error: 11_000
			.saturating_add(Weight::from_ref_time(11_927_000 as RefTimeWeight).saturating_mul(p as RefTimeWeight))
			.saturating_add(T::DbWeight::get().reads(2 as RefTimeWeight))
			.saturating_add(T::DbWeight::get().reads((1 as RefTimeWeight).saturating_mul(p as RefTimeWeight)))
			.saturating_add(T::DbWeight::get().writes(2 as RefTimeWeight))
			.saturating_add(T::DbWeight::get().writes((1 as RefTimeWeight).saturating_mul(p as RefTimeWeight)))
	}
	// Storage: Collective Members (r:1 w:0)
	/// The range of component `b` is `[1, 1024]`.
	/// The range of component `m` is `[1, 100]`.
	fn execute(b: u32, m: u32, ) -> Weight {
		Weight::from_ref_time(16_514_000 as RefTimeWeight)
			// Standard Error: 0
			.saturating_add(Weight::from_ref_time(2_000 as RefTimeWeight).saturating_mul(b as RefTimeWeight))
			// Standard Error: 0
			.saturating_add(Weight::from_ref_time(15_000 as RefTimeWeight).saturating_mul(m as RefTimeWeight))
			.saturating_add(T::DbWeight::get().reads(1 as RefTimeWeight))
	}
	// Storage: Collective Members (r:1 w:0)
	// Storage: Collective ProposalOf (r:1 w:0)
	/// The range of component `b` is `[1, 1024]`.
	/// The range of component `m` is `[1, 100]`.
	fn propose_execute(b: u32, m: u32, ) -> Weight {
		Weight::from_ref_time(19_149_000 as RefTimeWeight)
			// Standard Error: 0
			.saturating_add(Weight::from_ref_time(2_000 as RefTimeWeight).saturating_mul(b as RefTimeWeight))
			// Standard Error: 0
			.saturating_add(Weight::from_ref_time(20_000 as RefTimeWeight).saturating_mul(m as RefTimeWeight))
			.saturating_add(T::DbWeight::get().reads(2 as RefTimeWeight))
	}
	// Storage: Collective Members (r:1 w:0)
	// Storage: Collective ProposalOf (r:1 w:1)
	// Storage: Collective Proposals (r:1 w:1)
	// Storage: Collective ProposalCount (r:1 w:1)
	// Storage: Collective Voting (r:0 w:1)
	/// The range of component `b` is `[1, 1024]`.
	/// The range of component `m` is `[2, 100]`.
	/// The range of component `p` is `[1, 100]`.
	fn propose_proposed(b: u32, m: u32, p: u32, ) -> Weight {
		Weight::from_ref_time(25_040_000 as RefTimeWeight)
			// Standard Error: 0
			.saturating_add(Weight::from_ref_time(3_000 as RefTimeWeight).saturating_mul(b as RefTimeWeight))
			// Standard Error: 0
			.saturating_add(Weight::from_ref_time(20_000 as RefTimeWeight).saturating_mul(m as RefTimeWeight))
			// Standard Error: 0
			.saturating_add(Weight::from_ref_time(125_000 as RefTimeWeight).saturating_mul(p as RefTimeWeight))
			.saturating_add(T::DbWeight::get().reads(4 as RefTimeWeight))
			.saturating_add(T::DbWeight::get().writes(4 as RefTimeWeight))
	}
	// Storage: Collective Members (r:1 w:0)
	// Storage: Collective Voting (r:1 w:1)
	/// The range of component `m` is `[5, 100]`.
	fn vote(m: u32, ) -> Weight {
		Weight::from_ref_time(23_127_000 as RefTimeWeight)
			// Standard Error: 0
			.saturating_add(Weight::from_ref_time(53_000 as RefTimeWeight).saturating_mul(m as RefTimeWeight))
			.saturating_add(T::DbWeight::get().reads(2 as RefTimeWeight))
			.saturating_add(T::DbWeight::get().writes(1 as RefTimeWeight))
	}
	// Storage: Collective Voting (r:1 w:1)
	// Storage: Collective Members (r:1 w:0)
	// Storage: Collective Proposals (r:1 w:1)
	// Storage: Collective ProposalOf (r:0 w:1)
	/// The range of component `m` is `[4, 100]`.
	/// The range of component `p` is `[1, 100]`.
	fn close_early_disapproved(m: u32, p: u32, ) -> Weight {
		Weight::from_ref_time(27_759_000 as RefTimeWeight)
			// Standard Error: 0
			.saturating_add(Weight::from_ref_time(25_000 as RefTimeWeight).saturating_mul(m as RefTimeWeight))
			// Standard Error: 0
			.saturating_add(Weight::from_ref_time(95_000 as RefTimeWeight).saturating_mul(p as RefTimeWeight))
			.saturating_add(T::DbWeight::get().reads(3 as RefTimeWeight))
			.saturating_add(T::DbWeight::get().writes(3 as RefTimeWeight))
	}
	// Storage: Collective Voting (r:1 w:1)
	// Storage: Collective Members (r:1 w:0)
	// Storage: Collective ProposalOf (r:1 w:1)
	// Storage: Collective Proposals (r:1 w:1)
	/// The range of component `b` is `[1, 1024]`.
	/// The range of component `m` is `[4, 100]`.
	/// The range of component `p` is `[1, 100]`.
	fn close_early_approved(b: u32, m: u32, p: u32, ) -> Weight {
		Weight::from_ref_time(37_384_000 as RefTimeWeight)
			// Standard Error: 0
			.saturating_add(Weight::from_ref_time(1_000 as RefTimeWeight).saturating_mul(b as RefTimeWeight))
			// Standard Error: 0
			.saturating_add(Weight::from_ref_time(29_000 as RefTimeWeight).saturating_mul(m as RefTimeWeight))
			// Standard Error: 0
			.saturating_add(Weight::from_ref_time(105_000 as RefTimeWeight).saturating_mul(p as RefTimeWeight))
			.saturating_add(T::DbWeight::get().reads(4 as RefTimeWeight))
			.saturating_add(T::DbWeight::get().writes(3 as RefTimeWeight))
	}
	// Storage: Collective Voting (r:1 w:1)
	// Storage: Collective Members (r:1 w:0)
	// Storage: Collective Prime (r:1 w:0)
	// Storage: Collective Proposals (r:1 w:1)
	// Storage: Collective ProposalOf (r:0 w:1)
	/// The range of component `m` is `[4, 100]`.
	/// The range of component `p` is `[1, 100]`.
	fn close_disapproved(m: u32, p: u32, ) -> Weight {
		Weight::from_ref_time(29_142_000 as RefTimeWeight)
			// Standard Error: 0
			.saturating_add(Weight::from_ref_time(32_000 as RefTimeWeight).saturating_mul(m as RefTimeWeight))
			// Standard Error: 0
			.saturating_add(Weight::from_ref_time(105_000 as RefTimeWeight).saturating_mul(p as RefTimeWeight))
			.saturating_add(T::DbWeight::get().reads(4 as RefTimeWeight))
			.saturating_add(T::DbWeight::get().writes(3 as RefTimeWeight))
	}
	// Storage: Collective Voting (r:1 w:1)
	// Storage: Collective Members (r:1 w:0)
	// Storage: Collective Prime (r:1 w:0)
	// Storage: Collective ProposalOf (r:1 w:1)
	// Storage: Collective Proposals (r:1 w:1)
	/// The range of component `b` is `[1, 1024]`.
	/// The range of component `m` is `[4, 100]`.
	/// The range of component `p` is `[1, 100]`.
	fn close_approved(b: u32, m: u32, p: u32, ) -> Weight {
		Weight::from_ref_time(40_384_000 as RefTimeWeight)
			// Standard Error: 0
			.saturating_add(Weight::from_ref_time(1_000 as RefTimeWeight).saturating_mul(b as RefTimeWeight))
			// Standard Error: 0
			.saturating_add(Weight::from_ref_time(26_000 as RefTimeWeight).saturating_mul(m as RefTimeWeight))
			// Standard Error: 0
			.saturating_add(Weight::from_ref_time(103_000 as RefTimeWeight).saturating_mul(p as RefTimeWeight))
			.saturating_add(T::DbWeight::get().reads(5 as RefTimeWeight))
			.saturating_add(T::DbWeight::get().writes(3 as RefTimeWeight))
	}
	// Storage: Collective Proposals (r:1 w:1)
	// Storage: Collective Voting (r:0 w:1)
	// Storage: Collective ProposalOf (r:0 w:1)
	/// The range of component `p` is `[1, 100]`.
	fn disapprove_proposal(p: u32, ) -> Weight {
		Weight::from_ref_time(17_661_000 as RefTimeWeight)
			// Standard Error: 0
			.saturating_add(Weight::from_ref_time(106_000 as RefTimeWeight).saturating_mul(p as RefTimeWeight))
			.saturating_add(T::DbWeight::get().reads(1 as RefTimeWeight))
			.saturating_add(T::DbWeight::get().writes(3 as RefTimeWeight))
	}
}
