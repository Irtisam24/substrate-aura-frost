// This file is part of Substrate.

// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Autogenerated weights for pallet_dex
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-03-24, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `bm3`, CPU: `Intel(R) Core(TM) i7-7700K CPU @ 4.20GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// target/production/substrate
// benchmark
// pallet
// --steps=50
// --repeat=20
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --json-file=/var/lib/gitlab-runner/builds/zyw4fam_/0/parity/mirrors/substrate/.git/.artifacts/bench.json
// --pallet=pallet_dex
// --chain=dev
// --header=./HEADER-APACHE2
// --output=./frame/dex/src/weights.rs
// --template=./.maintain/frame-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for pallet_dex.
pub trait WeightInfo {
	fn create_pool() -> Weight;
	fn add_liquidity() -> Weight;
	fn remove_liquidity() -> Weight;
	fn swap_exact_tokens_for_tokens() -> Weight;
	fn swap_tokens_for_exact_tokens() -> Weight;
}

/// Weights for pallet_dex using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	/// Storage: Dex Pools (r:1 w:1)
	/// Proof: Dex Pools (max_values: None, max_size: Some(30), added: 2505, mode: MaxEncodedLen)
	/// Storage: System Account (r:2 w:2)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: Dex NextPoolAssetId (r:1 w:1)
	/// Proof: Dex NextPoolAssetId (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: PoolAssets Asset (r:1 w:1)
	/// Proof: PoolAssets Asset (max_values: None, max_size: Some(210), added: 2685, mode: MaxEncodedLen)
	fn create_pool() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `323`
		//  Estimated: `14855`
		// Minimum execution time: 72_228_000 picoseconds.
		Weight::from_parts(72_932_000, 14855)
			.saturating_add(T::DbWeight::get().reads(5_u64))
			.saturating_add(T::DbWeight::get().writes(5_u64))
	}
	/// Storage: Dex Pools (r:1 w:0)
	/// Proof: Dex Pools (max_values: None, max_size: Some(30), added: 2505, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: Assets Asset (r:1 w:1)
	/// Proof: Assets Asset (max_values: None, max_size: Some(210), added: 2685, mode: MaxEncodedLen)
	/// Storage: Assets Account (r:2 w:2)
	/// Proof: Assets Account (max_values: None, max_size: Some(102), added: 2577, mode: MaxEncodedLen)
	/// Storage: PoolAssets Asset (r:1 w:1)
	/// Proof: PoolAssets Asset (max_values: None, max_size: Some(210), added: 2685, mode: MaxEncodedLen)
	/// Storage: PoolAssets Account (r:2 w:2)
	/// Proof: PoolAssets Account (max_values: None, max_size: Some(102), added: 2577, mode: MaxEncodedLen)
	fn add_liquidity() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1232`
		//  Estimated: `26726`
		// Minimum execution time: 137_656_000 picoseconds.
		Weight::from_parts(138_526_000, 26726)
			.saturating_add(T::DbWeight::get().reads(8_u64))
			.saturating_add(T::DbWeight::get().writes(7_u64))
	}
	/// Storage: Dex Pools (r:1 w:0)
	/// Proof: Dex Pools (max_values: None, max_size: Some(30), added: 2505, mode: MaxEncodedLen)
	/// Storage: PoolAssets Asset (r:1 w:1)
	/// Proof: PoolAssets Asset (max_values: None, max_size: Some(210), added: 2685, mode: MaxEncodedLen)
	/// Storage: PoolAssets Account (r:2 w:2)
	/// Proof: PoolAssets Account (max_values: None, max_size: Some(102), added: 2577, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: Assets Asset (r:1 w:1)
	/// Proof: Assets Asset (max_values: None, max_size: Some(210), added: 2685, mode: MaxEncodedLen)
	/// Storage: Assets Account (r:2 w:2)
	/// Proof: Assets Account (max_values: None, max_size: Some(102), added: 2577, mode: MaxEncodedLen)
	fn remove_liquidity() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1496`
		//  Estimated: `26726`
		// Minimum execution time: 159_778_000 picoseconds.
		Weight::from_parts(160_730_000, 26726)
			.saturating_add(T::DbWeight::get().reads(8_u64))
			.saturating_add(T::DbWeight::get().writes(7_u64))
	}
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: Assets Asset (r:2 w:2)
	/// Proof: Assets Asset (max_values: None, max_size: Some(210), added: 2685, mode: MaxEncodedLen)
	/// Storage: Assets Account (r:4 w:4)
	/// Proof: Assets Account (max_values: None, max_size: Some(102), added: 2577, mode: MaxEncodedLen)
	fn swap_exact_tokens_for_tokens() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1134`
		//  Estimated: `21251`
		// Minimum execution time: 136_852_000 picoseconds.
		Weight::from_parts(137_764_000, 21251)
			.saturating_add(T::DbWeight::get().reads(7_u64))
			.saturating_add(T::DbWeight::get().writes(7_u64))
	}
	/// Storage: Assets Asset (r:2 w:2)
	/// Proof: Assets Asset (max_values: None, max_size: Some(210), added: 2685, mode: MaxEncodedLen)
	/// Storage: Assets Account (r:4 w:4)
	/// Proof: Assets Account (max_values: None, max_size: Some(102), added: 2577, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	fn swap_tokens_for_exact_tokens() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1134`
		//  Estimated: `21251`
		// Minimum execution time: 136_660_000 picoseconds.
		Weight::from_parts(137_522_000, 21251)
			.saturating_add(T::DbWeight::get().reads(7_u64))
			.saturating_add(T::DbWeight::get().writes(7_u64))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	/// Storage: Dex Pools (r:1 w:1)
	/// Proof: Dex Pools (max_values: None, max_size: Some(30), added: 2505, mode: MaxEncodedLen)
	/// Storage: System Account (r:2 w:2)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: Dex NextPoolAssetId (r:1 w:1)
	/// Proof: Dex NextPoolAssetId (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: PoolAssets Asset (r:1 w:1)
	/// Proof: PoolAssets Asset (max_values: None, max_size: Some(210), added: 2685, mode: MaxEncodedLen)
	fn create_pool() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `323`
		//  Estimated: `14855`
		// Minimum execution time: 72_228_000 picoseconds.
		Weight::from_parts(72_932_000, 14855)
			.saturating_add(RocksDbWeight::get().reads(5_u64))
			.saturating_add(RocksDbWeight::get().writes(5_u64))
	}
	/// Storage: Dex Pools (r:1 w:0)
	/// Proof: Dex Pools (max_values: None, max_size: Some(30), added: 2505, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: Assets Asset (r:1 w:1)
	/// Proof: Assets Asset (max_values: None, max_size: Some(210), added: 2685, mode: MaxEncodedLen)
	/// Storage: Assets Account (r:2 w:2)
	/// Proof: Assets Account (max_values: None, max_size: Some(102), added: 2577, mode: MaxEncodedLen)
	/// Storage: PoolAssets Asset (r:1 w:1)
	/// Proof: PoolAssets Asset (max_values: None, max_size: Some(210), added: 2685, mode: MaxEncodedLen)
	/// Storage: PoolAssets Account (r:2 w:2)
	/// Proof: PoolAssets Account (max_values: None, max_size: Some(102), added: 2577, mode: MaxEncodedLen)
	fn add_liquidity() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1232`
		//  Estimated: `26726`
		// Minimum execution time: 137_656_000 picoseconds.
		Weight::from_parts(138_526_000, 26726)
			.saturating_add(RocksDbWeight::get().reads(8_u64))
			.saturating_add(RocksDbWeight::get().writes(7_u64))
	}
	/// Storage: Dex Pools (r:1 w:0)
	/// Proof: Dex Pools (max_values: None, max_size: Some(30), added: 2505, mode: MaxEncodedLen)
	/// Storage: PoolAssets Asset (r:1 w:1)
	/// Proof: PoolAssets Asset (max_values: None, max_size: Some(210), added: 2685, mode: MaxEncodedLen)
	/// Storage: PoolAssets Account (r:2 w:2)
	/// Proof: PoolAssets Account (max_values: None, max_size: Some(102), added: 2577, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: Assets Asset (r:1 w:1)
	/// Proof: Assets Asset (max_values: None, max_size: Some(210), added: 2685, mode: MaxEncodedLen)
	/// Storage: Assets Account (r:2 w:2)
	/// Proof: Assets Account (max_values: None, max_size: Some(102), added: 2577, mode: MaxEncodedLen)
	fn remove_liquidity() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1496`
		//  Estimated: `26726`
		// Minimum execution time: 159_778_000 picoseconds.
		Weight::from_parts(160_730_000, 26726)
			.saturating_add(RocksDbWeight::get().reads(8_u64))
			.saturating_add(RocksDbWeight::get().writes(7_u64))
	}
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: Assets Asset (r:2 w:2)
	/// Proof: Assets Asset (max_values: None, max_size: Some(210), added: 2685, mode: MaxEncodedLen)
	/// Storage: Assets Account (r:4 w:4)
	/// Proof: Assets Account (max_values: None, max_size: Some(102), added: 2577, mode: MaxEncodedLen)
	fn swap_exact_tokens_for_tokens() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1134`
		//  Estimated: `21251`
		// Minimum execution time: 136_852_000 picoseconds.
		Weight::from_parts(137_764_000, 21251)
			.saturating_add(RocksDbWeight::get().reads(7_u64))
			.saturating_add(RocksDbWeight::get().writes(7_u64))
	}
	/// Storage: Assets Asset (r:2 w:2)
	/// Proof: Assets Asset (max_values: None, max_size: Some(210), added: 2685, mode: MaxEncodedLen)
	/// Storage: Assets Account (r:4 w:4)
	/// Proof: Assets Account (max_values: None, max_size: Some(102), added: 2577, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	fn swap_tokens_for_exact_tokens() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1134`
		//  Estimated: `21251`
		// Minimum execution time: 136_660_000 picoseconds.
		Weight::from_parts(137_522_000, 21251)
			.saturating_add(RocksDbWeight::get().reads(7_u64))
			.saturating_add(RocksDbWeight::get().writes(7_u64))
	}
}
