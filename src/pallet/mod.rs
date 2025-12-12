#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

use alloc::vec::Vec;
pub use pallet::*;

use polkadot_sdk::{
    frame_support::{
        BoundedVec, dynamic_params::*, require_transactional, storage_alias,
        traits::EnsureOriginWithArg, transactional,
    },
    polkadot_sdk_frame::{
        self as frame,
        runtime::{apis, prelude::*},
    },
    *,
};

#[frame_support::pallet]
pub mod pallet {
    use polkadot_sdk::sp_tracing::event;

    use super::*;
    #[pallet::pallet]
    pub struct Pallet<T>(_);

    #[derive(Default)]
    #[pallet::genesis_config]
    pub struct GenesisConfig {
        pub max_number: u32,
    }

    #[pallet::genesis_build]
    impl BuildGenesisConfig for GenesisConfig {
        fn build(&self) {}
    }

    #[pallet::config(with_default)]
    pub trait Config: polkadot_sdk::frame_system::Config {}

    #[pallet::storage]
    #[pallet::whitelist_storage]
    #[pallet::unbounded]
    #[pallet::disable_try_decode_storage]
    pub type TestCounter<T: Config> = StorageValue<_, u32>;

    impl<T: Config> Pallet<T> {
        pub fn test_block() -> DispatchResult {
            Ok(())
        }
    }
    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::call_index(1)]
        #[pallet::weight(100)]
        pub fn test_transactional(
            origin: OriginFor<T>,
            #[pallet::compact] val: u64,
        ) -> DispatchResult {
            //   Self::deposit_event(Event::TestInfo);
            Ok(())
        }
    }

    impl<T: Config> ValidateUnsigned for Pallet<T> {
        type Call = Call<T>;
        fn validate_unsigned(
            _source: TransactionSource,
            _call: &Self::Call,
        ) -> TransactionValidity {
            Err(TransactionValidityError::Invalid(InvalidTransaction::Call))
        }
    }
}
