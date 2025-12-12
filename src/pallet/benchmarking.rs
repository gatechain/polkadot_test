use super::*;

#[allow(unused)]
use crate::Pallet as Customs;
use polkadot_sdk::frame_benchmarking::v2::*;
use polkadot_sdk::frame_system::RawOrigin;

#[benchmarks]
mod benchmarks {
    use super::*;

    #[benchmark]
    fn test_transactional() {
        let value = 100u64;
        let caller: T::AccountId = whitelisted_caller();
        #[extrinsic_call]
        test_transactional(RawOrigin::Signed(caller), value);
    }

    #[benchmark]
    fn test_block() {
        #[block]
        {
            Customs::<T>::test_block();
        }
    }

    impl_benchmark_test_suite!(Customs, crate::mock::new_test_ext(), crate::mock::Test);
}
