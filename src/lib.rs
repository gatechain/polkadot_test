#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;
use alloc::vec::Vec;
use polkadot_sdk::frame_support::{
    BoundedVec, dynamic_params::*, require_transactional, storage_alias,
    traits::EnsureOriginWithArg, transactional,
};
use polkadot_sdk::{
    frame_system,
    pallet_parameters::*,
    polkadot_sdk_frame::{
        self as frame,
        runtime::{apis, prelude::*},
    },
    *,
};

type RuntimeExecutive =
    Executive<Runtime, Block, frame_system::ChainContext<Runtime>, Runtime, AllPalletsWithSystem>;

/// The transaction extensions that are added to the runtime.
type TxExtension = (frame_system::AuthorizeCall<Runtime>,);
type Block = frame::runtime::types_common::BlockOf<Runtime, TxExtension>;
type Header = HeaderFor<Runtime>;

parameter_types! {
    pub const Version: RuntimeVersion = VERSION;
}

/// The runtime version.
#[runtime_version]
pub const VERSION: RuntimeVersion = RuntimeVersion {
    spec_name: alloc::borrow::Cow::Borrowed("minimal-template-runtime"),
    impl_name: alloc::borrow::Cow::Borrowed("minimal-template-runtime"),
    authoring_version: 1,
    spec_version: 0,
    impl_version: 1,
    apis: RUNTIME_API_VERSIONS,
    transaction_version: 1,
    system_version: 1,
};

impl_runtime_apis! {
    impl apis::Core<Block> for Runtime {
        fn version() -> RuntimeVersion {
            VERSION
        }

        fn execute_block(block: Block) {
            RuntimeExecutive::execute_block(block)
        }

        fn initialize_block(header: &Header) -> ExtrinsicInclusionMode {
            RuntimeExecutive::initialize_block(header)
        }
    }
}

pub struct MyBaseConfig;
pub struct MyBlockHashCount<C: Get<u32>>(core::marker::PhantomData<C>);

#[frame_support::register_default_impl(MyBaseConfig)]
impl frame_system::DefaultConfig for MyBaseConfig {
    #[inject_runtime_type]
    type RuntimeEvent = ();
    #[inject_runtime_type]
    type RuntimeOrigin = ();
    #[inject_runtime_type]
    type RuntimeCall = ();
    #[inject_runtime_type]
    type PalletInfo = ();
    #[inject_runtime_type]
    type RuntimeTask = ();

    type Nonce = u32;
    type Hash = sp_core::hash::H256;
    type Hashing = sp_runtime::traits::BlakeTwo256;
    type AccountId = u64;
    type Lookup = sp_runtime::traits::IdentityLookup<Self::AccountId>;
    type MaxConsumers = frame_support::traits::ConstU32<16>;
    type AccountData = ();
    type OnNewAccount = ();
    type OnKilledAccount = ();
    type SystemWeightInfo = ();
    type ExtensionsWeightInfo = ();
    type SS58Prefix = ();
    type Version = ();
    type BlockWeights = ();
    type BlockLength = ();
    type DbWeight = ();

    type BaseCallFilter = frame_support::traits::Everything;
    type BlockHashCount = MyBlockHashCount<frame_support::traits::ConstU32<10>>;
    type OnSetCode = ();
    type SingleBlockMigrations = ();
    type MultiBlockMigrator = ();
    type PreInherents = ();
    type PostInherents = ();

    type PostTransactions = ();
}

/*
parameter_types! {
    pub const TestFieldValue: u32 = 10;
}
*/

pub struct SizeLimit();
impl Get<u32> for SizeLimit {
    fn get() -> u32 {
        100
    }
}


#[frame_support::pallet]
pub mod pallet {
    use super::*;
    #[pallet::pallet]
    pub struct Pallet<T>(_);

    #[pallet::genesis_config]
    pub struct GenesisConfig {
        pub max_number: u32,
    }

    #[pallet::genesis_build]
    impl BuildGenesisConfig for GenesisConfig {
        fn build(&self) {}
    }

    #[pallet::config(with_default)]
    pub trait Config: polkadot_sdk::frame_system::Config {
        type TestField: Get<u32>;
        type TestField2: Get<u32>;

        #[pallet::include_metadata]
        type MinerStatus: TypeInfo;
    }

    #[pallet::storage]
    #[pallet::whitelist_storage]
    #[pallet::unbounded]
    #[pallet::disable_try_decode_storage]
    pub type TestCounter<T: Config> = StorageValue<_, u32>;




    #[pallet::composite_enum]
    pub enum FreezeReason {
        AAAA
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::call_index(1)]
        #[pallet::weight(100)]
        pub fn test_transactional(
            origin: OriginFor<T>,
            #[pallet::compact] val: u64,
        ) -> DispatchResult {
            Self::deposit_event(Event::TestInfo);
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

pub trait Config {
    type SpecialType;
    type RegularType;
}

pub struct ExtendMyConfig;
// Composes the runtime by adding all the used pallets and deriving necessary types.
#[frame_construct_runtime]
mod runtime {
    /// The main runtime type.
    #[runtime::runtime]
    #[runtime::derive(
        RuntimeCall,
        RuntimeEvent,
        RuntimeError,
        RuntimeOrigin,
        RuntimeFreezeReason,
        RuntimeHoldReason,
        RuntimeSlashReason,
        RuntimeLockId,
        RuntimeTask,
        RuntimeViewFunction
    )]

    pub struct Runtime;

    /// Mandatory system pallet that should always be included in a FRAME runtime.
    #[runtime::pallet_index(0)]
    pub type System = frame_system::Pallet<Runtime>;

    /// Provides the ability to keep track of balances.
    #[runtime::pallet_index(2)]
    pub type Balances = pallet_balances::Pallet<Runtime>;

    #[runtime::pallet_index(3)]
    pub type Parameters = pallet_parameters::Pallet<Runtime>;

       #[runtime::pallet_index(4)]
    pub type Customs = pallet::Pallet<Runtime>;
}

#[derive_impl(frame_system::config_preludes::SolochainDefaultConfig)]
impl frame_system::Config for Runtime {
    type Block = Block;
    type Version = Version;
    // Use the account data from the balances pallet
    type AccountData = pallet_balances::AccountData<<Runtime as pallet_balances::Config>::Balance>;
}

#[derive_impl(pallet_balances::config_preludes::TestDefaultConfig)]
impl pallet_balances::Config for Runtime {
    type AccountStore = System;
    #[inject_runtime_type]
    type RuntimeHoldReason = ();
}

impl pallet_parameters::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type RuntimeParameters = RuntimeParameters;
    type AdminOrigin = DynamicParameterOrigin;
    type WeightInfo = ();
}

/// Defines what origin can modify which dynamic parameters.
pub struct DynamicParameterOrigin;
impl EnsureOriginWithArg<RuntimeOrigin, RuntimeParametersKey> for DynamicParameterOrigin {
    type Success = ();

    fn try_origin(
        origin: RuntimeOrigin,
        key: &RuntimeParametersKey,
    ) -> Result<Self::Success, RuntimeOrigin> {
        use crate::RuntimeParametersKey::*;

        match key {
            InflationAV(_) => frame_system::ensure_root(origin.clone()),
            Inflation2FG(_) => frame_system::ensure_root(origin.clone()),
        }
        .map_err(|_| origin)
    }
}

/// Dynamic params that can be adjusted at runtime.
#[dynamic_params(RuntimeParameters, pallet_parameters::Parameters::<Runtime>)]
pub mod dynamic_params {
    use super::*;
    /// Parameters used to calculate era payouts, see
    /// [`polkadot_runtime_common::impls::EraPayoutParams`].
    #[dynamic_pallet_params]
    #[codec(index = 0)]
    pub mod inflationAV {
        /// Minimum inflation rate used to calculate era payouts.
        #[codec(index = 0)]
        pub static MinInflation: u32 = 0;
    }

    #[dynamic_pallet_params]
    #[codec(index = 1)]
    pub mod inflation2FG {
        /// Minimum inflation rate used to calculate era payouts.
        #[codec(index = 0)]
        pub static MinInflation2: u32 = 0;
    }
}
