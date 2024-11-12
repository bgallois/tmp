
//! Autogenerated weights for `pallet_treasury`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 43.0.0
//! DATE: 2024-11-14, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `bgallois-ms7d43`, CPU: `12th Gen Intel(R) Core(TM) i3-12100F`
//! WASM-EXECUTION: `Compiled`, CHAIN: `None`, DB CACHE: 1024

// Executed Command:
// target/release/science-vault
// benchmark
// pallet
// --genesis-builder=spec-genesis
// --pallet
// *
// --extrinsic
// *
// --steps
// 50
// --repeat
// 20
// --output
// runtime/src/weights/

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::Weight};
use core::marker::PhantomData;

/// Weight functions for `pallet_treasury`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_treasury::WeightInfo for WeightInfo<T> {
	fn spend_local() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 0_000 picoseconds.
		Weight::from_parts(0, 0)
			.saturating_add(Weight::from_parts(0, 0))
	}
	/// Storage: `Treasury::Approvals` (r:1 w:0)
	/// Proof: `Treasury::Approvals` (`max_values`: Some(1), `max_size`: Some(402), added: 897, mode: `MaxEncodedLen`)
	fn remove_approval() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `6`
		//  Estimated: `1887`
		// Minimum execution time: 3_443_000 picoseconds.
		Weight::from_parts(3_993_000, 0)
			.saturating_add(Weight::from_parts(0, 1887))
			.saturating_add(T::DbWeight::get().reads(1))
	}
	/// Storage: `System::Account` (r:1 w:0)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `Treasury::Deactivated` (r:1 w:1)
	/// Proof: `Treasury::Deactivated` (`max_values`: Some(1), `max_size`: Some(16), added: 511, mode: `MaxEncodedLen`)
	/// Storage: `Treasury::Approvals` (r:1 w:1)
	/// Proof: `Treasury::Approvals` (`max_values`: Some(1), `max_size`: Some(402), added: 897, mode: `MaxEncodedLen`)
	/// The range of component `p` is `[0, 99]`.
	fn on_initialize_proposals(p: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `122 + p * (1 ±0)`
		//  Estimated: `3593`
		// Minimum execution time: 17_276_000 picoseconds.
		Weight::from_parts(19_095_703, 0)
			.saturating_add(Weight::from_parts(0, 3593))
			// Standard Error: 919
			.saturating_add(Weight::from_parts(26_198, 0).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	fn spend() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 0_000 picoseconds.
		Weight::from_parts(0, 0)
			.saturating_add(Weight::from_parts(0, 0))
	}
	/// Storage: `Treasury::Spends` (r:1 w:0)
	/// Proof: `Treasury::Spends` (`max_values`: None, `max_size`: Some(69), added: 2544, mode: `MaxEncodedLen`)
	fn payout() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `6`
		//  Estimated: `3534`
		// Minimum execution time: 4_382_000 picoseconds.
		Weight::from_parts(4_699_000, 0)
			.saturating_add(Weight::from_parts(0, 3534))
			.saturating_add(T::DbWeight::get().reads(1))
	}
	/// Storage: `Treasury::Spends` (r:1 w:0)
	/// Proof: `Treasury::Spends` (`max_values`: None, `max_size`: Some(69), added: 2544, mode: `MaxEncodedLen`)
	fn check_status() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `6`
		//  Estimated: `3534`
		// Minimum execution time: 4_327_000 picoseconds.
		Weight::from_parts(4_707_000, 0)
			.saturating_add(Weight::from_parts(0, 3534))
			.saturating_add(T::DbWeight::get().reads(1))
	}
	/// Storage: `Treasury::Spends` (r:1 w:0)
	/// Proof: `Treasury::Spends` (`max_values`: None, `max_size`: Some(69), added: 2544, mode: `MaxEncodedLen`)
	fn void_spend() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `6`
		//  Estimated: `3534`
		// Minimum execution time: 3_907_000 picoseconds.
		Weight::from_parts(4_147_000, 0)
			.saturating_add(Weight::from_parts(0, 3534))
			.saturating_add(T::DbWeight::get().reads(1))
	}
}
