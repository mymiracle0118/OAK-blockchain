
//! Autogenerated weights for `pallet_automation_time`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-07-15, STEPS: `100`, REPEAT: 64, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// ./target/release/oak-collator
// benchmark
// pallet
// --chain
// dev
// --execution
// wasm
// --wasm-execution
// compiled
// --pallet
// pallet_automation_time
// --extrinsic
// "*"
// --repeat
// 20
// --steps
// 50
// --output
// ./pallets/automation-time/src/weights.rs

// Summary:
//:append_to_missed_tasks 1_943_000
//:cancel_scheduled_task_full 859_475_000
//:force_cancel_scheduled_task 27_236_000
//:force_cancel_scheduled_task_full 826_320_000
//:run_auto_compound_delegated_stake_task 101_751_000
//:run_dynamic_dispatch_action 20_000_000
//:run_dynamic_dispatch_action_fail_decode 13_000_000
//:run_missed_tasks_many_found 169_000
//:run_missed_tasks_many_missing 186_000
//:run_native_transfer_task 33_877_000
//:run_notify_task 8_589_000
//:run_tasks_many_found 203_000
//:run_tasks_many_missing 182_000
//:run_xcmp_task 92_000_000
//:schedule_auto_compound_delegated_stake_task_full 112_631_000
//:schedule_native_transfer_task_empty 57_630_000
//:schedule_native_transfer_task_full 80_207_000
//:schedule_notify_task_empty 57_667_000
//:schedule_notify_task_full 70_014_000
//:schedule_xcmp_task_full 107_673_000
//:shift_missed_tasks 29_323_000
//:update_scheduled_task_queue 48_984_000
//:update_task_queue_overhead 1_938_000

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for pallet_automation_time.
pub trait WeightInfo {
	fn schedule_notify_task_empty() -> Weight;
	fn schedule_notify_task_full(v: u32, ) -> Weight;
	fn schedule_native_transfer_task_empty() -> Weight;
	fn schedule_native_transfer_task_full(v: u32, ) -> Weight;
	fn schedule_xcmp_task_full(v: u32, ) -> Weight;
	fn schedule_auto_compound_delegated_stake_task_full() -> Weight;
	fn cancel_scheduled_task_full() -> Weight;
	fn force_cancel_scheduled_task() -> Weight;
	fn force_cancel_scheduled_task_full() -> Weight;
	fn run_notify_task() -> Weight;
	fn run_native_transfer_task() -> Weight;
	fn run_xcmp_task() -> Weight;
	fn run_auto_compound_delegated_stake_task() -> Weight;
	fn run_dynamic_dispatch_action() -> Weight;
	fn run_dynamic_dispatch_action_fail_decode() -> Weight;
	fn run_missed_tasks_many_found(v: u32, ) -> Weight;
	fn run_missed_tasks_many_missing(v: u32, ) -> Weight;
	fn run_tasks_many_found(v: u32, ) -> Weight;
	fn run_tasks_many_missing(v: u32, ) -> Weight;
	fn update_task_queue_overhead() -> Weight;
	fn append_to_missed_tasks(v: u32, ) -> Weight;
	fn update_scheduled_task_queue() -> Weight;
	fn shift_missed_tasks() -> Weight;
	fn migration_v4_2_slots() -> Weight;
	fn migration_v4_1_slots() -> Weight;
	fn migration_v4_0_slots() -> Weight;
}

/// Weights for pallet_vesting using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	// Storage: Timestamp Now (r:1 w:0)
	// Storage: System Account (r:2 w:2)
	// Storage: AutomationTime AccountTasks (r:1 w:1)
	// Storage: AutomationTime ScheduledTasksV2 (r:1 w:1)
	fn schedule_notify_task_empty() -> Weight {
		(57_667_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(5 as Weight))
			.saturating_add(T::DbWeight::get().writes(4 as Weight))
	}
	// Storage: Timestamp Now (r:1 w:0)
	// Storage: System Account (r:2 w:2)
	// Storage: AutomationTime AccountTasks (r:1 w:1)
	// Storage: AutomationTime ScheduledTasksV2 (r:1 w:1)
	/// The range of component `v` is `[1, 24]`.
	fn schedule_notify_task_full(v: u32, ) -> Weight {
		(70_014_000 as Weight)
			// Standard Error: 179_000
			.saturating_add((36_043_000 as Weight).saturating_mul(v as Weight))
			.saturating_add(T::DbWeight::get().reads(4 as Weight))
			.saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(v as Weight)))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
			.saturating_add(T::DbWeight::get().writes((1 as Weight).saturating_mul(v as Weight)))
	}
	// Storage: Timestamp Now (r:1 w:0)
	// Storage: XcmpHandler XcmChainCurrencyData (r:1 w:0)
	// Storage: System Account (r:3 w:3)
	// Storage: AutomationTime AccountTasks (r:1 w:1)
	// Storage: AutomationTime ScheduledTasksV2 (r:1 w:1)
	// Storage: ParachainInfo ParachainId (r:1 w:0)
	/// The range of component `v` is `[1, 24]`.
	fn schedule_xcmp_task_full(v: u32, ) -> Weight {
		(107_673_000 as Weight)
			// Standard Error: 156_000
			.saturating_add((35_604_000 as Weight).saturating_mul(v as Weight))
			.saturating_add(T::DbWeight::get().reads(7 as Weight))
			.saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(v as Weight)))
			.saturating_add(T::DbWeight::get().writes(4 as Weight))
			.saturating_add(T::DbWeight::get().writes((1 as Weight).saturating_mul(v as Weight)))
	}
	// Storage: Timestamp Now (r:1 w:0)
	// Storage: System Account (r:2 w:2)
	// Storage: AutomationTime AccountTasks (r:1 w:1)
	// Storage: AutomationTime ScheduledTasksV2 (r:1 w:1)
	fn schedule_native_transfer_task_empty() -> Weight {
		(57_630_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(5 as Weight))
			.saturating_add(T::DbWeight::get().writes(4 as Weight))
	}
	// Storage: Timestamp Now (r:1 w:0)
	// Storage: System Account (r:2 w:2)
	// Storage: AutomationTime AccountTasks (r:1 w:1)
	// Storage: AutomationTime ScheduledTasksV2 (r:1 w:1)
	/// The range of component `v` is `[1, 24]`.
	fn schedule_native_transfer_task_full(v: u32, ) -> Weight {
		(80_207_000 as Weight)
			// Standard Error: 76_000
			.saturating_add((34_837_000 as Weight).saturating_mul(v as Weight))
			.saturating_add(T::DbWeight::get().reads(4 as Weight))
			.saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(v as Weight)))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
			.saturating_add(T::DbWeight::get().writes((1 as Weight).saturating_mul(v as Weight)))
	}
	// Storage: Timestamp Now (r:1 w:0)
	// Storage: System Account (r:2 w:2)
	// Storage: AutomationTime AccountTasks (r:1 w:1)
	// Storage: AutomationTime ScheduledTasksV2 (r:1 w:1)
	fn schedule_auto_compound_delegated_stake_task_full() -> Weight {
		(112_631_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(5 as Weight))
			.saturating_add(T::DbWeight::get().writes(4 as Weight))
	}
	// Storage: AutomationTime AccountTasks (r:1 w:1)
	// Storage: Timestamp Now (r:1 w:0)
	// Storage: AutomationTime LastTimeSlot (r:1 w:0)
	// Storage: AutomationTime ScheduledTasksV2 (r:24 w:24)
	fn cancel_scheduled_task_full() -> Weight {
		(859_475_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(27 as Weight))
			.saturating_add(T::DbWeight::get().writes(25 as Weight))
	}
	// Storage: AutomationTime AccountTasks (r:1 w:1)
	// Storage: Timestamp Now (r:1 w:0)
	// Storage: AutomationTime LastTimeSlot (r:1 w:0)
	// Storage: AutomationTime ScheduledTasksV2 (r:1 w:1)
	fn force_cancel_scheduled_task() -> Weight {
		(27_236_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(4 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: AutomationTime AccountTasks (r:1 w:1)
	// Storage: Timestamp Now (r:1 w:0)
	// Storage: AutomationTime LastTimeSlot (r:1 w:0)
	// Storage: AutomationTime ScheduledTasksV2 (r:24 w:24)
	fn force_cancel_scheduled_task_full() -> Weight {
		(826_320_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(27 as Weight))
			.saturating_add(T::DbWeight::get().writes(25 as Weight))
	}
	fn run_notify_task() -> Weight {
		(8_589_000 as Weight)
	}
	// Storage: System Account (r:2 w:2)
	fn run_native_transfer_task() -> Weight {
		(33_877_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: XcmpHandler XcmChainCurrencyData (r:1 w:0)
	// Storage: ParachainInfo ParachainId (r:1 w:0)
	// Storage: UnknownTokens ConcreteFungibleBalances (r:1 w:0)
	// Storage: AssetRegistry LocationToAssetId (r:1 w:0)
	// Storage: System Account (r:2 w:2)
	// Storage: ParachainSystem HostConfiguration (r:1 w:0)
	// Storage: ParachainSystem PendingUpwardMessages (r:1 w:1)
	fn run_xcmp_task() -> Weight {
		(102_000_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(8 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	// Storage: System Account (r:2 w:2)
	// Storage: ParachainStaking DelegatorState (r:1 w:1)
	// Storage: ParachainStaking DelegationScheduledRequests (r:1 w:0)
	// Storage: ParachainStaking DelegatorReserveToLockMigrations (r:1 w:0)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: ParachainStaking CandidateInfo (r:1 w:1)
	// Storage: ParachainStaking TopDelegations (r:1 w:1)
	// Storage: ParachainStaking CandidatePool (r:1 w:1)
	// Storage: ParachainStaking Total (r:1 w:1)
	// Storage: AutomationTime ScheduledTasksV2 (r:1 w:1)
	fn run_auto_compound_delegated_stake_task() -> Weight {
		(101_751_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(11 as Weight))
			.saturating_add(T::DbWeight::get().writes(9 as Weight))
	}
	fn run_dynamic_dispatch_action() -> Weight {
		(20_000_000 as Weight)
	}
	fn run_dynamic_dispatch_action_fail_decode() -> Weight {
		(13_000_000 as Weight)
	}
	// Storage: AutomationTime AccountTasks (r:1 w:1)
	/// The range of component `v` is `[0, 1]`.
	fn run_missed_tasks_many_found(v: u32, ) -> Weight {
		(169_000 as Weight)
			// Standard Error: 33_000
			.saturating_add((14_391_000 as Weight).saturating_mul(v as Weight))
			.saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(v as Weight)))
			.saturating_add(T::DbWeight::get().writes((1 as Weight).saturating_mul(v as Weight)))
	}
	// Storage: AutomationTime AccountTasks (r:1 w:0)
	/// The range of component `v` is `[0, 1]`.
	fn run_missed_tasks_many_missing(v: u32, ) -> Weight {
		(186_000 as Weight)
			// Standard Error: 25_000
			.saturating_add((11_071_000 as Weight).saturating_mul(v as Weight))
			.saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(v as Weight)))
	}
	// Storage: AutomationTime AccountTasks (r:1 w:1)
	// Storage: System Account (r:2 w:2)
	/// The range of component `v` is `[0, 1]`.
	fn run_tasks_many_found(v: u32, ) -> Weight {
		(203_000 as Weight)
			// Standard Error: 57_000
			.saturating_add((39_164_000 as Weight).saturating_mul(v as Weight))
			.saturating_add(T::DbWeight::get().reads((3 as Weight).saturating_mul(v as Weight)))
			.saturating_add(T::DbWeight::get().writes((3 as Weight).saturating_mul(v as Weight)))
	}
	// Storage: AutomationTime AccountTasks (r:1 w:0)
	/// The range of component `v` is `[0, 1]`.
	fn run_tasks_many_missing(v: u32, ) -> Weight {
		(182_000 as Weight)
			// Standard Error: 24_000
			.saturating_add((10_994_000 as Weight).saturating_mul(v as Weight))
			.saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(v as Weight)))
	}
	// Storage: Timestamp Now (r:1 w:0)
	fn update_task_queue_overhead() -> Weight {
		(1_938_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
	}
	// Storage: AutomationTime MissedQueueV2 (r:1 w:1)
	// Storage: AutomationTime ScheduledTasksV2 (r:1 w:1)
	/// The range of component `v` is `[0, 2]`.
	fn append_to_missed_tasks(v: u32, ) -> Weight {
		(1_943_000 as Weight)
			// Standard Error: 225_000
			.saturating_add((2_517_000 as Weight).saturating_mul(v as Weight))
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(v as Weight)))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
			.saturating_add(T::DbWeight::get().writes((1 as Weight).saturating_mul(v as Weight)))
	}
	// Storage: AutomationTime TaskQueueV2 (r:1 w:1)
	// Storage: AutomationTime MissedQueueV2 (r:1 w:1)
	// Storage: AutomationTime ScheduledTasksV2 (r:1 w:1)
	fn update_scheduled_task_queue() -> Weight {
		(48_984_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	// Storage: AutomationTime ScheduledTasksV2 (r:1 w:1)
	fn shift_missed_tasks() -> Weight {
		(29_323_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: unknown [0x85227ce1b17dca8228bad629dedea8b3cdc1d89ad121752636d864cb8fbf2580] (r:3 w:2)
	// Storage: AutomationTime AccountTasks (r:2 w:0)
	// Storage: AutomationTime ScheduledTasksV3 (r:0 w:2)
	fn migration_v4_2_slots() -> Weight {
		(27_000_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(5 as Weight))
			.saturating_add(T::DbWeight::get().writes(4 as Weight))
	}
	// Storage: unknown [0x85227ce1b17dca8228bad629dedea8b3cdc1d89ad121752636d864cb8fbf2580] (r:2 w:1)
	// Storage: AutomationTime AccountTasks (r:1 w:0)
	// Storage: AutomationTime ScheduledTasksV3 (r:0 w:1)
	fn migration_v4_1_slots() -> Weight {
		(14_000_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: unknown [0x85227ce1b17dca8228bad629dedea8b3cdc1d89ad121752636d864cb8fbf2580] (r:1 w:0)
	fn migration_v4_0_slots() -> Weight {
		(5_000_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
	}
}

// For backwards compatibility and tests
pub struct AutomationWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for AutomationWeight<T> {
	// Storage: Timestamp Now (r:1 w:0)
	// Storage: System Account (r:2 w:2)
	// Storage: AutomationTime Tasks (r:1 w:1)
	// Storage: AutomationTime ScheduledTasks (r:1 w:1)
	fn schedule_notify_task_empty() -> Weight {
		(60_000_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(5 as Weight))
			.saturating_add(RocksDbWeight::get().writes(4 as Weight))
	}
	// Storage: Timestamp Now (r:1 w:0)
	// Storage: System Account (r:2 w:2)
	// Storage: AutomationTime Tasks (r:1 w:1)
	// Storage: AutomationTime ScheduledTasks (r:1 w:1)
	/// The range of component `v` is `[1, 24]`.
	fn schedule_notify_task_full(v: u32, ) -> Weight {
		(72_025_000 as Weight)
			// Standard Error: 98_000
			.saturating_add((52_701_000 as Weight).saturating_mul(v as Weight))
			.saturating_add(RocksDbWeight::get().reads(4 as Weight))
			.saturating_add(RocksDbWeight::get().reads((1 as Weight).saturating_mul(v as Weight)))
			.saturating_add(RocksDbWeight::get().writes(3 as Weight))
			.saturating_add(RocksDbWeight::get().writes((1 as Weight).saturating_mul(v as Weight)))
	}
	// Storage: Timestamp Now (r:1 w:0)
	// Storage: XcmpHandler XcmChainCurrencyData (r:1 w:0)
	// Storage: System Account (r:3 w:3)
	// Storage: AutomationTime Tasks (r:1 w:1)
	// Storage: AutomationTime ScheduledTasks (r:1 w:1)
	// Storage: ParachainInfo ParachainId (r:1 w:0)
	/// The range of component `v` is `[1, 24]`.
	fn schedule_xcmp_task_full(v: u32, ) -> Weight {
		(122_408_000 as Weight)
			// Standard Error: 140_000
			.saturating_add((51_350_000 as Weight).saturating_mul(v as Weight))
			.saturating_add(RocksDbWeight::get().reads(7 as Weight))
			.saturating_add(RocksDbWeight::get().reads((1 as Weight).saturating_mul(v as Weight)))
			.saturating_add(RocksDbWeight::get().writes(4 as Weight))
			.saturating_add(RocksDbWeight::get().writes((1 as Weight).saturating_mul(v as Weight)))
	}
	// Storage: Timestamp Now (r:1 w:0)
	// Storage: System Account (r:2 w:2)
	// Storage: AutomationTime Tasks (r:1 w:1)
	// Storage: AutomationTime ScheduledTasks (r:1 w:1)
	fn schedule_native_transfer_task_empty() -> Weight {
		(65_000_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(5 as Weight))
			.saturating_add(RocksDbWeight::get().writes(4 as Weight))
	}
	// Storage: Timestamp Now (r:1 w:0)
	// Storage: System Account (r:2 w:2)
	// Storage: AutomationTime Tasks (r:1 w:1)
	// Storage: AutomationTime ScheduledTasks (r:1 w:1)
	/// The range of component `v` is `[1, 24]`.
	fn schedule_native_transfer_task_full(v: u32, ) -> Weight {
		(68_175_000 as Weight)
			// Standard Error: 110_000
			.saturating_add((52_169_000 as Weight).saturating_mul(v as Weight))
			.saturating_add(RocksDbWeight::get().reads(4 as Weight))
			.saturating_add(RocksDbWeight::get().reads((1 as Weight).saturating_mul(v as Weight)))
			.saturating_add(RocksDbWeight::get().writes(3 as Weight))
			.saturating_add(RocksDbWeight::get().writes((1 as Weight).saturating_mul(v as Weight)))
	}
	// Storage: Timestamp Now (r:1 w:0)
	// Storage: System Account (r:2 w:2)
	// Storage: AutomationTime Tasks (r:1 w:1)
	// Storage: AutomationTime ScheduledTasks (r:1 w:1)
	fn schedule_auto_compound_delegated_stake_task_full() -> Weight {
		(100_000_000 as Weight)
		.saturating_add(RocksDbWeight::get().reads(5 as Weight))
		.saturating_add(RocksDbWeight::get().writes(4 as Weight))
	}
	// Storage: AutomationTime Tasks (r:1 w:1)
	// Storage: Timestamp Now (r:1 w:0)
	// Storage: AutomationTime LastTimeSlot (r:1 w:0)
	// Storage: AutomationTime ScheduledTasks (r:24 w:24)
	fn cancel_scheduled_task_full() -> Weight {
		(1_284_000_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(27 as Weight))
			.saturating_add(RocksDbWeight::get().writes(25 as Weight))
	}
	// Storage: AutomationTime Tasks (r:1 w:1)
	// Storage: Timestamp Now (r:1 w:0)
	// Storage: AutomationTime LastTimeSlot (r:1 w:0)
	// Storage: AutomationTime ScheduledTasks (r:1 w:1)
	fn force_cancel_scheduled_task() -> Weight {
		(29_000_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(4 as Weight))
			.saturating_add(RocksDbWeight::get().writes(2 as Weight))
	}
	// Storage: AutomationTime Tasks (r:1 w:1)
	// Storage: Timestamp Now (r:1 w:0)
	// Storage: AutomationTime LastTimeSlot (r:1 w:0)
	// Storage: AutomationTime ScheduledTasks (r:24 w:24)
	fn force_cancel_scheduled_task_full() -> Weight {
		(1_274_000_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(27 as Weight))
			.saturating_add(RocksDbWeight::get().writes(25 as Weight))
	}
	fn run_notify_task() -> Weight {
		(10_000_000 as Weight)
	}
	// Storage: System Account (r:2 w:2)
	fn run_native_transfer_task() -> Weight {
		(37_000_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(2 as Weight))
			.saturating_add(RocksDbWeight::get().writes(2 as Weight))
	}
	// Storage: XcmpHandler XcmChainCurrencyData (r:1 w:0)
	// Storage: ParachainInfo ParachainId (r:1 w:0)
	// Storage: UnknownTokens ConcreteFungibleBalances (r:1 w:0)
	// Storage: AssetRegistry LocationToAssetId (r:1 w:0)
	// Storage: System Account (r:2 w:2)
	// Storage: ParachainSystem HostConfiguration (r:1 w:0)
	// Storage: ParachainSystem PendingUpwardMessages (r:1 w:1)
	fn run_xcmp_task() -> Weight {
		(102_000_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(8 as Weight))
			.saturating_add(RocksDbWeight::get().writes(3 as Weight))
	}
	// Storage: System Account (r:2 w:2)
	// Storage: ParachainStaking DelegatorState (r:1 w:1)
	// Storage: ParachainStaking DelegationScheduledRequests (r:1 w:0)
	// Storage: ParachainStaking DelegatorReserveToLockMigrations (r:1 w:0)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: ParachainStaking CandidateInfo (r:1 w:1)
	// Storage: ParachainStaking TopDelegations (r:1 w:1)
	// Storage: ParachainStaking CandidatePool (r:1 w:1)
	// Storage: ParachainStaking Total (r:1 w:1)
	// Storage: AutomationTime ScheduledTasks (r:1 w:1)
	fn run_auto_compound_delegated_stake_task() -> Weight {
		(109_000_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(11 as Weight))
			.saturating_add(RocksDbWeight::get().writes(9 as Weight))
	}
	fn run_dynamic_dispatch_action() -> Weight {
		(20_000_000 as Weight)
	}
	fn run_dynamic_dispatch_action_fail_decode() -> Weight {
		(13_000_000 as Weight)
	}
	// Storage: AutomationTime Tasks (r:1 w:1)
	fn run_missed_tasks_many_found(v: u32, ) -> Weight {
		(0 as Weight)
			// Standard Error: 0
			.saturating_add((12_000_000 as Weight).saturating_mul(v as Weight))
			.saturating_add(RocksDbWeight::get().reads((1 as Weight).saturating_mul(v as Weight)))
			.saturating_add(RocksDbWeight::get().writes((1 as Weight).saturating_mul(v as Weight)))
	}
	// Storage: AutomationTime Tasks (r:1 w:0)
	fn run_missed_tasks_many_missing(v: u32, ) -> Weight {
		(0 as Weight)
			// Standard Error: 88_000
			.saturating_add((9_594_000 as Weight).saturating_mul(v as Weight))
			.saturating_add(RocksDbWeight::get().reads((1 as Weight).saturating_mul(v as Weight)))
	}
	// Storage: AutomationTime Tasks (r:1 w:1)
	// Storage: System Account (r:2 w:2)
	fn run_tasks_many_found(v: u32, ) -> Weight {
		(0 as Weight)
			// Standard Error: 88_000
			.saturating_add((32_406_000 as Weight).saturating_mul(v as Weight))
			.saturating_add(RocksDbWeight::get().reads((3 as Weight).saturating_mul(v as Weight)))
			.saturating_add(RocksDbWeight::get().writes((3 as Weight).saturating_mul(v as Weight)))
	}
	// Storage: AutomationTime Tasks (r:1 w:0)
	/// The range of component `v` is `[0, 1]`.
	fn run_tasks_many_missing(v: u32, ) -> Weight {
		(63_000 as Weight)
			// Standard Error: 135_000
			.saturating_add((12_250_000 as Weight).saturating_mul(v as Weight))
			.saturating_add(RocksDbWeight::get().reads((1 as Weight).saturating_mul(v as Weight)))
	}
	// Storage: Timestamp Now (r:1 w:0)
	fn update_task_queue_overhead() -> Weight {
		(2_000_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(1 as Weight))
	}
	// Storage: AutomationTime MissedQueue (r:1 w:1)
	// Storage: AutomationTime ScheduledTasks (r:1 w:1)
	/// The range of component `v` is `[0, 2]`.
	fn append_to_missed_tasks(v: u32, ) -> Weight {
		(2_240_000 as Weight)
			// Standard Error: 185_000
			.saturating_add((2_156_000 as Weight).saturating_mul(v as Weight))
			.saturating_add(RocksDbWeight::get().reads(1 as Weight))
			.saturating_add(RocksDbWeight::get().reads((1 as Weight).saturating_mul(v as Weight)))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
			.saturating_add(RocksDbWeight::get().writes((1 as Weight).saturating_mul(v as Weight)))
	}
	// Storage: AutomationTime TaskQueue (r:1 w:1)
	// Storage: AutomationTime MissedQueue (r:1 w:1)
	// Storage: AutomationTime ScheduledTasks (r:1 w:1)
	fn update_scheduled_task_queue() -> Weight {
		(51_000_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(3 as Weight))
			.saturating_add(RocksDbWeight::get().writes(3 as Weight))
	}
	// Storage: AutomationTime ScheduledTasks (r:1 w:1)
	fn shift_missed_tasks() -> Weight {
		(19_000_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(1 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	// Storage: unknown [0x85227ce1b17dca8228bad629dedea8b3cdc1d89ad121752636d864cb8fbf2580] (r:3 w:2)
	// Storage: AutomationTime AccountTasks (r:2 w:0)
	// Storage: AutomationTime ScheduledTasksV3 (r:0 w:2)
	fn migration_v4_2_slots() -> Weight {
		(27_000_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(5 as Weight))
			.saturating_add(RocksDbWeight::get().writes(4 as Weight))
	}
	// Storage: unknown [0x85227ce1b17dca8228bad629dedea8b3cdc1d89ad121752636d864cb8fbf2580] (r:2 w:1)
	// Storage: AutomationTime AccountTasks (r:1 w:0)
	// Storage: AutomationTime ScheduledTasksV3 (r:0 w:1)
	fn migration_v4_1_slots() -> Weight {
		(14_000_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(3 as Weight))
			.saturating_add(RocksDbWeight::get().writes(2 as Weight))
	}
	// Storage: unknown [0x85227ce1b17dca8228bad629dedea8b3cdc1d89ad121752636d864cb8fbf2580] (r:1 w:0)
	fn migration_v4_0_slots() -> Weight {
		(5_000_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(1 as Weight))
	}
}
