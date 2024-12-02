
//! Autogenerated weights for `aga_access_segregator`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-04-26, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// ./target/release/standalone-node-template
// benchmark
// pallet
// --chain
// dev
// --execution=wasm
// --wasm-execution=compiled
// --pallet
// aga_access_segregator
// --extrinsic
// *
// --steps
// 50
// --repeat
// 20
// --output
// access_weight.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use core::marker::PhantomData;

pub trait WeightInfo {
	fn pause_bridge() -> Weight;
	fn unpause_bridge() -> Weight;
	fn register_domain() -> Weight;
	fn unregister_domain() -> Weight;
	fn deposit() -> Weight;
	fn execute_proposals(n: u32) -> Weight;
	fn set_fee() -> Weight;
	fn pause_all_bridges() -> Weight;
	fn unpause_all_bridges() -> Weight;
}

/// Weight functions for `aga_access_segregator`.
/// Weights for pallet_template using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	/// Storage: AgaBridge DestDomainIds (r:0 w:1)
	/// Proof Skipped: AgaBridge DestDomainIds (max_values: None, max_size: None, mode: Measured)
	/// Storage: AgaBridge DestChainIds (r:0 w:1)
	/// Proof Skipped: AgaBridge DestChainIds (max_values: None, max_size: None, mode: Measured)
	fn pause_bridge() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 10_000_000 picoseconds.
		Weight::from_parts(10_000_000, 0)
			.saturating_add(Weight::from_parts(0, 0))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: AgaBridge DestDomainIds (r:0 w:1)
	/// Proof Skipped: AgaBridge DestDomainIds (max_values: None, max_size: None, mode: Measured)
	/// Storage: AgaBridge DestChainIds (r:0 w:1)
	/// Proof Skipped: AgaBridge DestChainIds (max_values: None, max_size: None, mode: Measured)
	fn unpause_bridge() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 10_000_000 picoseconds.
		Weight::from_parts(10_000_000, 0)
			.saturating_add(Weight::from_parts(0, 0))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: AgaBridge DestDomainIds (r:0 w:1)
	/// Proof Skipped: AgaBridge DestDomainIds (max_values: None, max_size: None, mode: Measured)
	/// Storage: AgaBridge DestChainIds (r:0 w:1)
	/// Proof Skipped: AgaBridge DestChainIds (max_values: None, max_size: None, mode: Measured)
	fn register_domain() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 10_000_000 picoseconds.
		Weight::from_parts(10_000_000, 0)
			.saturating_add(Weight::from_parts(0, 0))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: AgaBridge DestDomainIds (r:0 w:1)
	/// Proof Skipped: AgaBridge DestDomainIds (max_values: None, max_size: None, mode: Measured)
	/// Storage: AgaBridge DestChainIds (r:0 w:1)
	/// Proof Skipped: AgaBridge DestChainIds (max_values: None, max_size: None, mode: Measured)
	fn unregister_domain() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 10_000_000 picoseconds.
		Weight::from_parts(10_000_000, 0)
			.saturating_add(Weight::from_parts(0, 0))
			.saturating_add(T::DbWeight::get().writes(2))
	}

	/// Storage: AgaBridge DestDomainIds (r:0 w:1)
	/// Proof Skipped: AgaBridge DestDomainIds (max_values: None, max_size: None, mode: Measured)
	/// Storage: AgaBridge DestChainIds (r:0 w:1)
	/// Proof Skipped: AgaBridge DestChainIds (max_values: None, max_size: None, mode: Measured)
	fn deposit() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 10_000_000 picoseconds.
		Weight::from_parts(10_000_000, 0)
			.saturating_add(Weight::from_parts(0, 0))
			.saturating_add(T::DbWeight::get().writes(2))
	}

	/// Storage: AgaBridge MpcAddr (r:1 w:0)
	/// Proof Skipped: AgaBridge MpcAddr (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: AgaBridge IsPaused (r:1 w:0)
	/// Proof Skipped: AgaBridge IsPaused (max_values: None, max_size: None, mode: Measured)
	/// Storage: AgaBridge DestDomainIds (r:1 w:0)
	/// Proof Skipped: AgaBridge DestDomainIds (max_values: None, max_size: None, mode: Measured)
	/// Storage: AgaBridge UsedNonces (r:1 w:1)
	/// Proof Skipped: AgaBridge UsedNonces (max_values: None, max_size: None, mode: Measured)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// The range of component `n` is `[1, 1000]`.
	fn execute_proposals(n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `280`
		//  Estimated: `16593`
		// Minimum execution time: 123_000_000 picoseconds.
		Weight::from_parts(151_050_908, 0)
			.saturating_add(Weight::from_parts(0, 16593))
			// Standard Error: 18_882
			.saturating_add(Weight::from_parts(10_748_102, 0).saturating_mul(n.into()))
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(2))
	}


	/// Storage: AgaBridge DestDomainIds (r:0 w:1)
	/// Proof Skipped: AgaBridge DestDomainIds (max_values: None, max_size: None, mode: Measured)
	/// Storage: AgaBridge DestChainIds (r:0 w:1)
	/// Proof Skipped: AgaBridge DestChainIds (max_values: None, max_size: None, mode: Measured)
	fn set_fee() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 10_000_000 picoseconds.
		Weight::from_parts(10_000_000, 0)
			.saturating_add(Weight::from_parts(0, 0))
			.saturating_add(T::DbWeight::get().writes(2))
	}

	/// Storage: AgaBridge DestDomainIds (r:0 w:1)
	/// Proof Skipped: AgaBridge DestDomainIds (max_values: None, max_size: None, mode: Measured)
	/// Storage: AgaBridge DestChainIds (r:0 w:1)
	/// Proof Skipped: AgaBridge DestChainIds (max_values: None, max_size: None, mode: Measured)
	fn pause_all_bridges() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 10_000_000 picoseconds.
		Weight::from_parts(10_000_000, 0)
			.saturating_add(Weight::from_parts(0, 0))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	
	/// Storage: AgaBridge DestDomainIds (r:0 w:1)
	/// Proof Skipped: AgaBridge DestDomainIds (max_values: None, max_size: None, mode: Measured)
	/// Storage: AgaBridge DestChainIds (r:0 w:1)
	/// Proof Skipped: AgaBridge DestChainIds (max_values: None, max_size: None, mode: Measured)
	fn unpause_all_bridges() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 10_000_000 picoseconds.
		Weight::from_parts(10_000_000, 0)
			.saturating_add(Weight::from_parts(0, 0))
			.saturating_add(T::DbWeight::get().writes(2))
	}
}
