
//! Autogenerated weights for `pallet_reward`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-12-17, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `bgallois-ms7d43`, CPU: `12th Gen Intel(R) Core(TM) i3-12100F`
//! EXECUTION: ``, WASM-EXECUTION: `Compiled`, CHAIN: `Some("dev")`, DB CACHE: 1024

// Executed Command:
// target/release/science-vault
// benchmark
// pallet
// --chain
// dev
// --wasm-execution=compiled
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

/// Weight functions for `pallet_reward`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_reward::WeightInfo for WeightInfo<T> {
	/// Storage: `Reward::Reputations` (r:2 w:1)
	/// Proof: `Reward::Reputations` (`max_values`: None, `max_size`: Some(88), added: 2563, mode: `MaxEncodedLen`)
	/// Storage: `Reward::MaxRawReputation` (r:1 w:1)
	/// Proof: `Reward::MaxRawReputation` (`max_values`: Some(1), `max_size`: Some(16), added: 511, mode: `MaxEncodedLen`)
	fn reward() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `290`
		//  Estimated: `6116`
		// Minimum execution time: 19_407_000 picoseconds.
		Weight::from_parts(31_351_000, 0)
			.saturating_add(Weight::from_parts(0, 6116))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: `Reward::Reputations` (r:2 w:1)
	/// Proof: `Reward::Reputations` (`max_values`: None, `max_size`: Some(88), added: 2563, mode: `MaxEncodedLen`)
	/// Storage: `Reward::MaxRawReputation` (r:1 w:0)
	/// Proof: `Reward::MaxRawReputation` (`max_values`: Some(1), `max_size`: Some(16), added: 511, mode: `MaxEncodedLen`)
	fn punish() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `290`
		//  Estimated: `6116`
		// Minimum execution time: 17_414_000 picoseconds.
		Weight::from_parts(18_314_000, 0)
			.saturating_add(Weight::from_parts(0, 6116))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Reward::Reputations` (r:1 w:1)
	/// Proof: `Reward::Reputations` (`max_values`: None, `max_size`: Some(88), added: 2563, mode: `MaxEncodedLen`)
	/// Storage: `Reward::MaxRawReputation` (r:1 w:1)
	/// Proof: `Reward::MaxRawReputation` (`max_values`: Some(1), `max_size`: Some(16), added: 511, mode: `MaxEncodedLen`)
	fn slash() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `191`
		//  Estimated: `3553`
		// Minimum execution time: 9_662_000 picoseconds.
		Weight::from_parts(10_815_000, 0)
			.saturating_add(Weight::from_parts(0, 3553))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: `Reward::AccountQueue` (r:1 w:0)
	/// Proof: `Reward::AccountQueue` (`max_values`: Some(1), `max_size`: Some(3200004), added: 3200499, mode: `MaxEncodedLen`)
	/// Storage: `Reward::EvaluationQueue` (r:0 w:1)
	/// Proof: `Reward::EvaluationQueue` (`max_values`: Some(1), `max_size`: Some(3200004), added: 3200499, mode: `MaxEncodedLen`)
	/// Storage: `Reward::MaxRawReputation` (r:0 w:1)
	/// Proof: `Reward::MaxRawReputation` (`max_values`: Some(1), `max_size`: Some(16), added: 511, mode: `MaxEncodedLen`)
	fn do_process_evaluation_queue() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `43`
		//  Estimated: `3201489`
		// Minimum execution time: 4_455_000 picoseconds.
		Weight::from_parts(4_733_000, 0)
			.saturating_add(Weight::from_parts(0, 3201489))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: `Reward::AccountQueue` (r:1 w:0)
	/// Proof: `Reward::AccountQueue` (`max_values`: Some(1), `max_size`: Some(3200004), added: 3200499, mode: `MaxEncodedLen`)
	/// Storage: `Reward::EvaluationQueue` (r:0 w:1)
	/// Proof: `Reward::EvaluationQueue` (`max_values`: Some(1), `max_size`: Some(3200004), added: 3200499, mode: `MaxEncodedLen`)
	/// Storage: `Reward::MaxRawReputation` (r:0 w:1)
	/// Proof: `Reward::MaxRawReputation` (`max_values`: Some(1), `max_size`: Some(16), added: 511, mode: `MaxEncodedLen`)
	/// The range of component `i` is `[1, 1024]`.
	fn process_evaluation_queue(i: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `76`
		//  Estimated: `3201489`
		// Minimum execution time: 4_424_000 picoseconds.
		Weight::from_parts(5_812_655, 0)
			.saturating_add(Weight::from_parts(0, 3201489))
			// Standard Error: 100
			.saturating_add(Weight::from_parts(623, 0).saturating_mul(i.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: `Reward::AccountQueue` (r:1 w:0)
	/// Proof: `Reward::AccountQueue` (`max_values`: Some(1), `max_size`: Some(3200004), added: 3200499, mode: `MaxEncodedLen`)
	/// Storage: `Reward::EvaluationQueue` (r:0 w:1)
	/// Proof: `Reward::EvaluationQueue` (`max_values`: Some(1), `max_size`: Some(3200004), added: 3200499, mode: `MaxEncodedLen`)
	/// Storage: `Reward::MaxRawReputation` (r:0 w:1)
	/// Proof: `Reward::MaxRawReputation` (`max_values`: Some(1), `max_size`: Some(16), added: 511, mode: `MaxEncodedLen`)
	fn on_idle_noop() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `43`
		//  Estimated: `3201489`
		// Minimum execution time: 4_566_000 picoseconds.
		Weight::from_parts(4_768_000, 0)
			.saturating_add(Weight::from_parts(0, 3201489))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(2))
	}
}
