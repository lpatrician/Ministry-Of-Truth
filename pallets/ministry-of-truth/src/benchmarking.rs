//! Benchmarking setup for ministry-of-truth

use super::*;

#[allow(unused)]
use crate::Pallet as Template;
use frame_benchmarking::{benchmarks, impl_benchmark_test_suite, whitelisted_caller};
use frame_system::RawOrigin;

#[cfg(feature = "runtime-benchmarks")]

// TODO: Implement
// mod benchmarking {
// 	use crate::{*, Module as PalletModule};
// 	use frame_benchmarking::{benchmarks, account, impl_benchmark_test_suite};
// 	use frame_system::RawOrigin;
  
// 	benchmarks! {
// 		store_article {
// 			let s in 0 .. 100;
// 			let caller: T::AccountId = whitelisted_caller();
// 			let x: Vec<u8> = [s, s].to_vec();
// 			let n: Vec<u8>  = [s, s].to_vec();
// 		}: _(RawOrigin::Signed(caller), x, n)
// 		store_claim_for_article {
// 			let s in 0 .. 100;
// 			let caller: T::AccountId = whitelisted_caller();
// 			let x: Vec<u8>  = [s, s].to_vec();
// 		}: _(RawOrigin::Signed(caller), x, s, false)
// 		verify {
// 			assert_eq!(Something::<T>::get(), Some(s));
// 		}
// 	}
// }

impl_benchmark_test_suite!(MinistryOfTruth, crate::mock::new_test_ext(), crate::mock::Test);
