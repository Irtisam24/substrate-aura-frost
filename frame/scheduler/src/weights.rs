// This file is part of Substrate.

// Copyright (C) 2022 Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Autogenerated weights for pallet_scheduler
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-05-23, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// ./target/production/substrate
// benchmark
// pallet
// --chain=dev
// --steps=50
// --repeat=20
// --pallet=pallet_scheduler
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --template=./.maintain/frame-weight-template.hbs
// --output=./frame/scheduler/src/weights.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for pallet_scheduler.
pub trait WeightInfo {
	fn service_agendas() -> Weight;
	fn service_agenda(_i: u32, ) -> Weight;
	fn service_task_base() -> Weight;
	fn service_task_periodic() -> Weight;
	fn service_task_named() -> Weight;
	fn service_task_fetched(_s: u32, ) -> Weight;
	fn execute_dispatch_signed() -> Weight;
	fn execute_dispatch_unsigned() -> Weight;
	fn schedule(_s: u32, ) -> Weight;
	fn cancel(_s: u32, ) -> Weight;
	fn schedule_named(_s: u32, ) -> Weight;
	fn cancel_named(_s: u32, ) -> Weight;
}

pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	fn service_agendas() -> Weight { Weight::zero() }
	fn service_agenda(_i: u32, ) -> Weight { Weight::zero() }
	fn service_task_base() -> Weight { Weight::zero() }
	fn service_task_periodic() -> Weight { Weight::zero() }
	fn service_task_named() -> Weight { Weight::zero() }
	fn service_task_fetched(_s: u32, ) -> Weight { Weight::zero() }
	fn execute_dispatch_signed() -> Weight { Weight::zero() }
	fn execute_dispatch_unsigned() -> Weight { Weight::zero() }
	fn schedule(_s: u32, ) -> Weight { Weight::zero() }
	fn cancel(_s: u32, ) -> Weight { Weight::zero() }
	fn schedule_named(_s: u32, ) -> Weight { Weight::zero() }
	fn cancel_named(_s: u32, ) -> Weight { Weight::zero() }
}

impl WeightInfo for () {
	fn service_agendas() -> Weight { Weight::zero() }
	fn service_agenda(_i: u32, ) -> Weight { Weight::zero() }
	fn service_task_base() -> Weight { Weight::zero() }
	fn service_task_periodic() -> Weight { Weight::zero() }
	fn service_task_named() -> Weight { Weight::zero() }
	fn service_task_fetched(_s: u32, ) -> Weight { Weight::zero() }
	fn execute_dispatch_signed() -> Weight { Weight::zero() }
	fn execute_dispatch_unsigned() -> Weight { Weight::zero() }
	fn schedule(_s: u32, ) -> Weight { Weight::zero() }
	fn cancel(_s: u32, ) -> Weight { Weight::zero() }
	fn schedule_named(_s: u32, ) -> Weight { Weight::zero() }
	fn cancel_named(_s: u32, ) -> Weight { Weight::zero() }
}
