#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;
#[cfg(test)]
mod mock;
mod pallet;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarks;

pub use pallet::*;
use alloc::vec::Vec;

use polkadot_sdk::{
    frame_support,
    polkadot_sdk_frame::{
        self as frame,
        runtime::{apis, prelude::*},
    },
    *,

};

type TxExtension = (frame_system::AuthorizeCall<Runtime>);
type Header = HeaderFor<Runtime>;
type Block = frame::runtime::types_common::BlockOf<Runtime, TxExtension>;
type AccountId = frame::runtime::types_common::AccountId;
type Balance = <Runtime as pallet_balances::Config>::Balance;
type MinimumBalance = <Runtime as pallet_balances::Config>::ExistentialDeposit;

type RuntimeExecutive =
    Executive<Runtime, Block, frame_system::ChainContext<Runtime>, Runtime, AllPalletsWithSystem>;

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

/// Provides getters for genesis configuration presets.
pub mod genesis_config_presets {
    use super::*;
    use crate::{
        BalancesConfig, RuntimeGenesisConfig,
        sp_keyring::Sr25519Keyring,
    };

    use alloc::{vec, vec::Vec};
    use serde_json::Value;

    /// Returns a development genesis config preset.
    pub fn development_config_genesis() -> Value {
        let endowment = <MinimumBalance as Get<Balance>>::get().max(1) * 1000;
        frame_support::build_struct_json_patch!(RuntimeGenesisConfig {
            balances: BalancesConfig {
                balances: Sr25519Keyring::iter()
                    .map(|a| (a.to_account_id(), endowment))
                    .collect::<Vec<_>>(),
            },
        })
    }

    /// Get the set of the available genesis config presets.
    pub fn get_preset(id: &PresetId) -> Option<Vec<u8>> {
        let patch = match id.as_ref() {
            sp_genesis_builder::DEV_RUNTIME_PRESET => development_config_genesis(),
            _ => return None,
        };
        Some(
            serde_json::to_string(&patch)
                .expect("serialization to json is expected to work. qed.")
                .into_bytes(),
        )
    }

    /// List of supported presets.
    pub fn preset_names() -> Vec<PresetId> {
        vec![PresetId::from(sp_genesis_builder::DEV_RUNTIME_PRESET)]
    }
}

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
    impl apis::Metadata<Block> for Runtime {
        fn metadata() -> OpaqueMetadata {
            OpaqueMetadata::new(Runtime::metadata().into())
        }

        fn metadata_at_version(version: u32) -> Option<OpaqueMetadata> {
            Runtime::metadata_at_version(version)
        }

        fn metadata_versions() -> Vec<u32> {
            Runtime::metadata_versions()
        }
    }

    impl apis::BlockBuilder<Block> for Runtime {
        fn apply_extrinsic(extrinsic: ExtrinsicFor<Runtime>) -> ApplyExtrinsicResult {
            RuntimeExecutive::apply_extrinsic(extrinsic)
        }

        fn finalize_block() -> HeaderFor<Runtime> {
            RuntimeExecutive::finalize_block()
        }

        fn inherent_extrinsics(data: InherentData) -> Vec<ExtrinsicFor<Runtime>> {
            data.create_extrinsics()
        }

        fn check_inherents(
            block: Block,
            data: InherentData,
        ) -> CheckInherentsResult {
            data.check_extrinsics(&block)
        }
    }

    impl apis::GenesisBuilder<Block> for Runtime {
        fn build_state(config: Vec<u8>) -> sp_genesis_builder::Result {
            build_state::<RuntimeGenesisConfig>(config)
        }

        fn get_preset(id: &Option<PresetId>) -> Option<Vec<u8>> {
            get_preset::<RuntimeGenesisConfig>(id, self::genesis_config_presets::get_preset)
        }

        fn preset_names() -> Vec<PresetId> {
            self::genesis_config_presets::preset_names()
        }
    }

    #[cfg(feature = "runtime-benchmarks")]
	impl frame_benchmarking::Benchmark<Block> for Runtime {
		fn benchmark_metadata(extra: bool) -> (
			Vec<frame_benchmarking::BenchmarkList>,
			Vec<frame_support::traits::StorageInfo>,
		) {
			use frame_benchmarking::{baseline, BenchmarkList};
			use frame_support::traits::StorageInfoTrait;
			use frame_system_benchmarking::Pallet as SystemBench;
			use frame_system_benchmarking::extensions::Pallet as SystemExtensionsBench;
			use baseline::Pallet as BaselineBench;

			let mut list = Vec::<BenchmarkList>::new();
			list_benchmarks!(list, extra);

			let storage_info = AllPalletsWithSystem::storage_info();

			(list, storage_info)
		}

		#[allow(non_local_definitions)]
		fn dispatch_benchmark(
			config: frame_benchmarking::BenchmarkConfig
		) -> Result<Vec<frame_benchmarking::BenchmarkBatch>, alloc::string::String> {
			use frame_benchmarking::{baseline, BenchmarkBatch};
			use sp_storage::TrackedStorageKey;
			use frame_system_benchmarking::Pallet as SystemBench;
			use frame_system_benchmarking::extensions::Pallet as SystemExtensionsBench;
			use baseline::Pallet as BaselineBench;

			impl frame_system_benchmarking::Config for Runtime {}
			impl baseline::Config for Runtime {}

			use frame_support::traits::WhitelistedStorageKeys;
			let whitelist: Vec<TrackedStorageKey> = AllPalletsWithSystem::whitelisted_storage_keys();

			let mut batches = Vec::<BenchmarkBatch>::new();
			let params = (&config, &whitelist);
			add_benchmarks!(params, batches);

			Ok(batches)
		}
	}
}

pub struct MyBaseConfig;
pub struct MyBlockHashCount<C: Get<u32>>(core::marker::PhantomData<C>);
impl<I: From<u32>, C: Get<u32>> Get<I> for MyBlockHashCount<C> {
        fn get() -> I {
            C::get().into()
        }
    }

#[frame_support::register_default_impl(MyBaseConfig)]
impl frame_system::DefaultConfig for MyBaseConfig {
			type Nonce = u32;
			type Hash = sp_core::hash::H256;
			type Hashing = sp_runtime::traits::BlakeTwo256;
			type AccountId = AccountId;
			type Lookup = sp_runtime::traits::AccountIdLookup<Self::AccountId, ()>;
			type MaxConsumers = frame_support::traits::ConstU32<128>;
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
			#[inject_runtime_type]
			type RuntimeEvent = ();
			#[inject_runtime_type]
			type RuntimeOrigin = ();
			#[inject_runtime_type]
			type RuntimeCall = ();
			#[inject_runtime_type]
			type RuntimeTask = ();

			#[inject_runtime_type]
			type PalletInfo = ();
			type BaseCallFilter = frame_support::traits::Everything;
			type BlockHashCount = MyBlockHashCount<frame_support::traits::ConstU32<256>>;
			type OnSetCode = ();
			type SingleBlockMigrations = ();
			type MultiBlockMigrator = ();
			type PreInherents = ();
			type PostInherents = ();
			type PostTransactions = ();
}

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
    #[runtime::pallet_index(1)]
    pub type Balances = pallet_balances::Pallet<Runtime>;

    #[runtime::pallet_index(2)]
    pub type Customs = pallet::Pallet<Runtime>;
}

#[derive_impl(MyBaseConfig)]
impl frame_system::Config for Runtime {
    type Block = Block;
    type Version = Version;
    type AccountData = pallet_balances::AccountData<<Runtime as pallet_balances::Config>::Balance>;
}

#[derive_impl(pallet_balances::config_preludes::TestDefaultConfig)]
impl pallet_balances::Config for Runtime {
    type AccountStore = System;
    #[inject_runtime_type]
    type RuntimeHoldReason = ();
}

impl pallet::Config for Runtime {}
