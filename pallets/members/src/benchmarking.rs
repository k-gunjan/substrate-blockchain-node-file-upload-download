// SPDX-License-Identifier: Apache-2.0


//! Benchmarking for pallet-club-members.

#![cfg(feature = "runtime-benchmarks")]
mod benchmarking;

use crate::*;
use frame_benchmarking::{benchmarks, whitelisted_caller};
use frame_system::RawOrigin;
use sp_core::Pair;

// To actually run this benchmark on pallet-example-basic, we need to put this pallet into the
//   runtime and compile it with `runtime-benchmarks` feature. The detail procedures are
//   documented at:
//   https://docs.substrate.io/reference/how-to-guides/weights/add-benchmarks/
//
// The auto-generated weight estimate of this pallet is copied over to the `weights.rs` file.
// The exact command of how the estimate generated is printed at the top of the file.

// Details on using the benchmarks macro can be seen at:
//   https://paritytech.github.io/substrate/master/frame_benchmarking/trait.Benchmarking.html#tymethod.benchmarks
benchmarks! {
	// This will measure the execution time of `set_dummy` for b in [1..1000] range.
	add_member_benchmark {
		// This is the benchmark setup phase
        // let (pair, _) = sp_core::sr25519::Pair::generate();
        // let a: T::AccountId = pair.public();
        let a: T::AccountId = whitelisted_caller();

	}: add_member(RawOrigin::Root, a) // The execution phase is just running `add_member` extrinsic call
	// verify {
	// 	// This is the optional benchmark verification phase, asserting certain states.
	// 	assert_eq!(Pallet::<T>::dummy(), Some(b.into()))
	// }

	// This will measure the execution time of `accumulate_dummy` for b in [1..1000] range.
	// The benchmark execution phase is shorthanded. When the name of the benchmark case is the same
	// as the extrinsic call. `_(...)` is used to represent the extrinsic name.
	// The benchmark verification phase is omitted.
	// accumulate_dummy {
	// 	let b in 1 .. 1000;
	// 	// The caller account is whitelisted for DB reads/write by the benchmarking macro.
	// 	let caller: T::AccountId = whitelisted_caller();
	// }: _(RawOrigin::Signed(caller), b.into())

	// // This will measure the execution time of sorting a vector.
	// sort_vector {
	// 	let x in 0 .. 10000;
	// 	let mut m = Vec::<u32>::new();
	// 	for i in (0..x).rev() {
	// 		m.push(i);
	// 	}
	// }: {
	// 	// The benchmark execution phase could also be a closure with custom code
	// 	m.sort_unstable();
	// }

	// // This line generates test cases for benchmarking, and could be run by:
	// //   `cargo test -p pallet-example-basic --all-features`, you will see one line per case:
	// //   `test benchmarking::bench_sort_vector ... ok`
	// //   `test benchmarking::bench_accumulate_dummy ... ok`
	// //   `test benchmarking::bench_set_dummy_benchmark ... ok` in the result.
	// //
	// // The line generates three steps per benchmark, with repeat=1 and the three steps are
	// //   [low, mid, high] of the range.
	// impl_benchmark_test_suite!(Pallet, crate::tests::new_test_ext(), crate::tests::Test)
}