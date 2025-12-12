#![feature(prelude_import)]
#![no_std]
#[macro_use]
extern crate core;
#[prelude_import]
use core::prelude::rust_2024::*;
extern crate alloc;
use polkadot_sdk::frame_support::{
    BoundedVec, dynamic_params::*, require_transactional, storage_alias,
    traits::EnsureOriginWithArg, transactional,
};
use polkadot_sdk::{
    pallet_parameters::*,
    polkadot_sdk_frame::{self as frame, runtime::{apis, prelude::*}},
    *,
};
type RuntimeExecutive = Executive<
    Runtime,
    Block,
    frame_system::ChainContext<Runtime>,
    Runtime,
    AllPalletsWithSystem,
>;
/// The transaction extensions that are added to the runtime.
type TxExtension = (frame_system::AuthorizeCall<Runtime>,);
type Block = frame::runtime::types_common::BlockOf<Runtime, TxExtension>;
type Header = HeaderFor<Runtime>;
pub struct Version;
impl Version {
    /// Returns the value of this parameter type.
    pub const fn get() -> RuntimeVersion {
        VERSION
    }
}
impl<_I: From<RuntimeVersion>> ::frame_support::traits::Get<_I> for Version {
    fn get() -> _I {
        _I::from(Self::get())
    }
}
impl ::frame_support::traits::TypedGet for Version {
    type Type = RuntimeVersion;
    fn get() -> RuntimeVersion {
        Self::get()
    }
}
/// The runtime version.
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
const _: () = {
    #[link_section = "runtime_version"]
    static SECTION_CONTENTS: [u8; 68usize] = [
        96u8, 109u8, 105u8, 110u8, 105u8, 109u8, 97u8, 108u8, 45u8, 116u8, 101u8, 109u8,
        112u8, 108u8, 97u8, 116u8, 101u8, 45u8, 114u8, 117u8, 110u8, 116u8, 105u8, 109u8,
        101u8, 96u8, 109u8, 105u8, 110u8, 105u8, 109u8, 97u8, 108u8, 45u8, 116u8, 101u8,
        109u8, 112u8, 108u8, 97u8, 116u8, 101u8, 45u8, 114u8, 117u8, 110u8, 116u8, 105u8,
        109u8, 101u8, 1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 1u8, 0u8, 0u8, 0u8, 0u8,
        1u8, 0u8, 0u8, 0u8, 1u8,
    ];
};
const _: () = {};
pub struct RuntimeApi {}
impl apis::runtime_decl_for_core::Core<Block> for Runtime {
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
pub const RUNTIME_API_VERSIONS: polkadot_sdk::sp_api::__private::ApisVec = ::sp_version::Cow::Borrowed(
    &[(apis::runtime_decl_for_core::ID, apis::runtime_decl_for_core::VERSION)],
);
const _: () = {
    #[link_section = "runtime_apis"]
    static SECTION_CONTENTS: [u8; 12] = polkadot_sdk::sp_api::__private::serialize_runtime_api_info(
        apis::runtime_decl_for_core::ID,
        apis::runtime_decl_for_core::VERSION,
    );
};
#[doc(hidden)]
impl polkadot_sdk::sp_api::__private::metadata_ir::InternalImplRuntimeApis for Runtime {
    fn runtime_metadata(
        &self,
    ) -> polkadot_sdk::sp_api::__private::vec::Vec<
        polkadot_sdk::sp_api::__private::metadata_ir::RuntimeApiMetadataIR,
    > {
        <[_]>::into_vec(
            ::alloc::boxed::box_new([
                apis::runtime_decl_for_core::runtime_metadata::<
                    Block,
                >(apis::runtime_decl_for_core::VERSION),
            ]),
        )
    }
}
pub mod api {
    use super::*;
    #[no_mangle]
    pub unsafe extern fn Core_version(input_data: *mut u8, input_len: usize) -> u64 {
        let mut input = if input_len == 0 {
            &[0u8; 0]
        } else {
            unsafe { ::core::slice::from_raw_parts(input_data, input_len) }
        };
        polkadot_sdk::sp_api::__private::init_runtime_logger();
        let output = (move || {
            if !input.is_empty() {
                {
                    ::core::panicking::panic_fmt(
                        format_args!(
                            "Bad input data provided to {0}: expected no parameters, but input buffer is not empty. Nothing bad happened: someone sent an invalid transaction to the node.",
                            "version",
                        ),
                    );
                };
            }
            #[allow(deprecated)]
            <Runtime as apis::runtime_decl_for_core::Core<Block>>::version()
        })();
        polkadot_sdk::sp_api::__private::to_substrate_wasm_fn_return_value(&output)
    }
    #[no_mangle]
    pub unsafe extern fn Core_execute_block(
        input_data: *mut u8,
        input_len: usize,
    ) -> u64 {
        let mut input = if input_len == 0 {
            &[0u8; 0]
        } else {
            unsafe { ::core::slice::from_raw_parts(input_data, input_len) }
        };
        polkadot_sdk::sp_api::__private::init_runtime_logger();
        let output = (move || {
            let block: Block = match polkadot_sdk::sp_api::__private::Decode::decode(
                &mut input,
            ) {
                Ok(res) => res,
                Err(e) => {
                    ::core::panicking::panic_fmt(
                        format_args!(
                            "Bad input data provided to {0}: {1}. Nothing bad happened: someone sent an invalid transaction to the node.",
                            "execute_block",
                            e,
                        ),
                    );
                }
            };
            #[allow(deprecated)]
            <Runtime as apis::runtime_decl_for_core::Core<Block>>::execute_block(block)
        })();
        polkadot_sdk::sp_api::__private::to_substrate_wasm_fn_return_value(&output)
    }
    #[no_mangle]
    pub unsafe extern fn Core_initialize_block(
        input_data: *mut u8,
        input_len: usize,
    ) -> u64 {
        let mut input = if input_len == 0 {
            &[0u8; 0]
        } else {
            unsafe { ::core::slice::from_raw_parts(input_data, input_len) }
        };
        polkadot_sdk::sp_api::__private::init_runtime_logger();
        let output = (move || {
            let header: Header = match polkadot_sdk::sp_api::__private::Decode::decode(
                &mut input,
            ) {
                Ok(res) => res,
                Err(e) => {
                    ::core::panicking::panic_fmt(
                        format_args!(
                            "Bad input data provided to {0}: {1}. Nothing bad happened: someone sent an invalid transaction to the node.",
                            "initialize_block",
                            e,
                        ),
                    );
                }
            };
            #[allow(deprecated)]
            <Runtime as apis::runtime_decl_for_core::Core<
                Block,
            >>::initialize_block(&header)
        })();
        polkadot_sdk::sp_api::__private::to_substrate_wasm_fn_return_value(&output)
    }
}
pub struct MyBaseConfig;
pub struct MyBlockHashCount<C: Get<u32>>(core::marker::PhantomData<C>);
pub use __export_tokens_tt_859260323_my_base_config_0 as MyBaseConfig;
#[allow(unused)]
impl frame_system::DefaultConfig for MyBaseConfig {
    type RuntimeEvent = ();
    type RuntimeOrigin = ();
    type RuntimeCall = ();
    type PalletInfo = ();
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
pub struct SizeLimit();
impl Get<u32> for SizeLimit {
    fn get() -> u32 {
        100
    }
}
/**The `pallet` module in each FRAME pallet hosts the most important items needed
to construct this pallet.

The main components of this pallet are:
- [`Pallet`], which implements all of the dispatchable extrinsics of the pallet, among
other public functions.
	- The subset of the functions that are dispatchable can be identified either in the
	[`dispatchables`] module or in the [`Call`] enum.
- [`storage_types`], which contains the list of all types that are representing a
storage item. Otherwise, all storage items are listed among [*Type Definitions*](#types).
- [`Config`], which contains the configuration trait of this pallet.
- [`Event`] and [`Error`], which are listed among the [*Enums*](#enums).
		*/
pub mod pallet {
    use super::*;
    /**
				The `Pallet` struct, the main type that implements traits and standalone
				functions within the pallet.
			*/
    pub struct Pallet<T>(core::marker::PhantomData<(T)>);
    const _: () = {
        #[automatically_derived]
        #[allow(deprecated)]
        impl<T> ::core::clone::Clone for Pallet<T> {
            fn clone(&self) -> Self {
                Self(::core::clone::Clone::clone(&self.0))
            }
        }
    };
    #[allow(deprecated)]
    const _: () = {
        impl<T> ::core::cmp::Eq for Pallet<T> {}
    };
    const _: () = {
        #[automatically_derived]
        #[allow(deprecated)]
        impl<T> ::core::cmp::PartialEq for Pallet<T> {
            fn eq(&self, other: &Self) -> bool {
                true && self.0 == other.0
            }
        }
    };
    const _: () = {
        #[automatically_derived]
        #[allow(deprecated)]
        impl<T> ::core::fmt::Debug for Pallet<T> {
            fn fmt(&self, fmt: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                fmt.debug_tuple("Pallet").field(&self.0).finish()
            }
        }
    };
    /**
					Can be used to configure the
					[genesis state](https://docs.substrate.io/build/genesis-configuration/)
					of this pallet.
					*/
    #[serde(rename_all = "camelCase")]
    #[serde(deny_unknown_fields)]
    #[serde(bound(serialize = ""))]
    #[serde(bound(deserialize = ""))]
    #[serde(crate = "polkadot_sdk :: frame_support::__private::serde")]
    pub struct GenesisConfig {
        pub max_number: u32,
    }
    #[doc(hidden)]
    #[allow(
        non_upper_case_globals,
        unused_attributes,
        unused_qualifications,
        clippy::absolute_paths,
    )]
    const _: () = {
        use polkadot_sdk::frame_support::__private::serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for GenesisConfig {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private228::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                let mut __serde_state = _serde::Serializer::serialize_struct(
                    __serializer,
                    "GenesisConfig",
                    false as usize + 1,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "maxNumber",
                    &self.max_number,
                )?;
                _serde::ser::SerializeStruct::end(__serde_state)
            }
        }
    };
    #[doc(hidden)]
    #[allow(
        non_upper_case_globals,
        unused_attributes,
        unused_qualifications,
        clippy::absolute_paths,
    )]
    const _: () = {
        use polkadot_sdk::frame_support::__private::serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for GenesisConfig {
            fn deserialize<__D>(
                __deserializer: __D,
            ) -> _serde::__private228::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                #[doc(hidden)]
                enum __Field {
                    __field0,
                }
                #[doc(hidden)]
                struct __FieldVisitor;
                #[automatically_derived]
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private228::Formatter,
                    ) -> _serde::__private228::fmt::Result {
                        _serde::__private228::Formatter::write_str(
                            __formatter,
                            "field identifier",
                        )
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::__private228::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::__private228::Ok(__Field::__field0),
                            _ => {
                                _serde::__private228::Err(
                                    _serde::de::Error::invalid_value(
                                        _serde::de::Unexpected::Unsigned(__value),
                                        &"field index 0 <= i < 1",
                                    ),
                                )
                            }
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::__private228::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "maxNumber" => _serde::__private228::Ok(__Field::__field0),
                            _ => {
                                _serde::__private228::Err(
                                    _serde::de::Error::unknown_field(__value, FIELDS),
                                )
                            }
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::__private228::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"maxNumber" => _serde::__private228::Ok(__Field::__field0),
                            _ => {
                                let __value = &_serde::__private228::from_utf8_lossy(
                                    __value,
                                );
                                _serde::__private228::Err(
                                    _serde::de::Error::unknown_field(__value, FIELDS),
                                )
                            }
                        }
                    }
                }
                #[automatically_derived]
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private228::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(
                            __deserializer,
                            __FieldVisitor,
                        )
                    }
                }
                #[doc(hidden)]
                struct __Visitor<'de> {
                    marker: _serde::__private228::PhantomData<GenesisConfig>,
                    lifetime: _serde::__private228::PhantomData<&'de ()>,
                }
                #[automatically_derived]
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = GenesisConfig;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private228::Formatter,
                    ) -> _serde::__private228::fmt::Result {
                        _serde::__private228::Formatter::write_str(
                            __formatter,
                            "struct GenesisConfig",
                        )
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::__private228::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 = match _serde::de::SeqAccess::next_element::<
                            u32,
                        >(&mut __seq)? {
                            _serde::__private228::Some(__value) => __value,
                            _serde::__private228::None => {
                                return _serde::__private228::Err(
                                    _serde::de::Error::invalid_length(
                                        0usize,
                                        &"struct GenesisConfig with 1 element",
                                    ),
                                );
                            }
                        };
                        _serde::__private228::Ok(GenesisConfig {
                            max_number: __field0,
                        })
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::__private228::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        let mut __field0: _serde::__private228::Option<u32> = _serde::__private228::None;
                        while let _serde::__private228::Some(__key) = _serde::de::MapAccess::next_key::<
                            __Field,
                        >(&mut __map)? {
                            match __key {
                                __Field::__field0 => {
                                    if _serde::__private228::Option::is_some(&__field0) {
                                        return _serde::__private228::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "maxNumber",
                                            ),
                                        );
                                    }
                                    __field0 = _serde::__private228::Some(
                                        _serde::de::MapAccess::next_value::<u32>(&mut __map)?,
                                    );
                                }
                            }
                        }
                        let __field0 = match __field0 {
                            _serde::__private228::Some(__field0) => __field0,
                            _serde::__private228::None => {
                                _serde::__private228::de::missing_field("maxNumber")?
                            }
                        };
                        _serde::__private228::Ok(GenesisConfig {
                            max_number: __field0,
                        })
                    }
                }
                #[doc(hidden)]
                const FIELDS: &'static [&'static str] = &["maxNumber"];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "GenesisConfig",
                    FIELDS,
                    __Visitor {
                        marker: _serde::__private228::PhantomData::<GenesisConfig>,
                        lifetime: _serde::__private228::PhantomData,
                    },
                )
            }
        }
    };
    impl BuildGenesisConfig for GenesisConfig {
        fn build(&self) {}
    }
    /**
Configuration trait of this pallet.

The main purpose of this trait is to act as an interface between this pallet and the runtime in
which it is embedded in. A type, function, or constant in this trait is essentially left to be
configured by the runtime that includes this pallet.

Consequently, a runtime that wants to include this pallet must implement this trait.*/
    pub trait Config: polkadot_sdk::frame_system::Config {
        type TestField: Get<u32>;
        type TestField2: Get<u32>;
        type MinerStatus: TypeInfo;
    }
    #[allow(type_alias_bounds)]
    ///
    ///Storage type is [`StorageValue`] with value type `u32`.
    pub type TestCounter<T: Config> = StorageValue<
        _GeneratedPrefixForStorageTestCounter<T>,
        u32,
    >;
    impl<T: Config> Pallet<T> {
        pub fn test_transactional(origin: OriginFor<T>, val: u64) -> DispatchResult {
            polkadot_sdk::frame_support::storage::with_storage_layer::<
                (),
                polkadot_sdk::frame_support::pallet_prelude::DispatchError,
                _,
            >(|| {
                TestCounter::<T>::get();
                Ok(())
            })
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
    impl<T: Config> Pallet<T> {
        #[doc(hidden)]
        pub fn pallet_documentation_metadata() -> polkadot_sdk::frame_support::__private::Vec<
            &'static str,
        > {
            ::alloc::vec::Vec::new()
        }
    }
    impl<T: Config> Pallet<T> {
        #[doc(hidden)]
        pub fn pallet_constants_metadata() -> polkadot_sdk::frame_support::__private::Vec<
            polkadot_sdk::frame_support::__private::metadata_ir::PalletConstantMetadataIR,
        > {
            ::alloc::vec::Vec::new()
        }
    }
    impl<T: Config> Pallet<T> {
        #[doc(hidden)]
        pub fn error_metadata() -> Option<
            polkadot_sdk::frame_support::__private::metadata_ir::PalletErrorMetadataIR,
        > {
            None
        }
    }
    /// Type alias to `Pallet`, to be used by `construct_runtime`.
    ///
    /// Generated by `pallet` attribute macro.
    #[deprecated(note = "use `Pallet` instead")]
    #[allow(dead_code)]
    pub type Module<T> = Pallet<T>;
    impl<T: Config> polkadot_sdk::frame_support::traits::GetStorageVersion
    for Pallet<T> {
        type InCodeStorageVersion = polkadot_sdk::frame_support::traits::NoStorageVersionSet;
        fn in_code_storage_version() -> Self::InCodeStorageVersion {
            core::default::Default::default()
        }
        fn on_chain_storage_version() -> polkadot_sdk::frame_support::traits::StorageVersion {
            polkadot_sdk::frame_support::traits::StorageVersion::get::<Self>()
        }
    }
    impl<T: Config> polkadot_sdk::frame_support::traits::OnGenesis for Pallet<T> {
        fn on_genesis() {
            let storage_version: polkadot_sdk::frame_support::traits::StorageVersion = core::default::Default::default();
            storage_version.put::<Self>();
        }
    }
    impl<T: Config> polkadot_sdk::frame_support::traits::PalletInfoAccess for Pallet<T> {
        fn index() -> usize {
            <<T as polkadot_sdk::frame_system::Config>::PalletInfo as polkadot_sdk::frame_support::traits::PalletInfo>::index::<
                Self,
            >()
                .expect(
                    "Pallet is part of the runtime because pallet `Config` trait is \
						implemented by the runtime",
                )
        }
        fn name() -> &'static str {
            <<T as polkadot_sdk::frame_system::Config>::PalletInfo as polkadot_sdk::frame_support::traits::PalletInfo>::name::<
                Self,
            >()
                .expect(
                    "Pallet is part of the runtime because pallet `Config` trait is \
						implemented by the runtime",
                )
        }
        fn name_hash() -> [u8; 16] {
            <<T as polkadot_sdk::frame_system::Config>::PalletInfo as polkadot_sdk::frame_support::traits::PalletInfo>::name_hash::<
                Self,
            >()
                .expect(
                    "Pallet is part of the runtime because pallet `Config` trait is \
						implemented by the runtime",
                )
        }
        fn module_name() -> &'static str {
            <<T as polkadot_sdk::frame_system::Config>::PalletInfo as polkadot_sdk::frame_support::traits::PalletInfo>::module_name::<
                Self,
            >()
                .expect(
                    "Pallet is part of the runtime because pallet `Config` trait is \
						implemented by the runtime",
                )
        }
        fn crate_version() -> polkadot_sdk::frame_support::traits::CrateVersion {
            polkadot_sdk::frame_support::traits::CrateVersion {
                major: 0u16,
                minor: 1u8,
                patch: 0u8,
            }
        }
    }
    impl<T: Config> polkadot_sdk::frame_support::traits::PalletsInfoAccess
    for Pallet<T> {
        fn count() -> usize {
            1
        }
        fn infos() -> polkadot_sdk::frame_support::__private::Vec<
            polkadot_sdk::frame_support::traits::PalletInfoData,
        > {
            use polkadot_sdk::frame_support::traits::PalletInfoAccess;
            let item = polkadot_sdk::frame_support::traits::PalletInfoData {
                index: Self::index(),
                name: Self::name(),
                module_name: Self::module_name(),
                crate_version: Self::crate_version(),
            };
            <[_]>::into_vec(::alloc::boxed::box_new([item]))
        }
    }
    impl<T: Config> polkadot_sdk::frame_support::traits::StorageInfoTrait for Pallet<T> {
        fn storage_info() -> polkadot_sdk::frame_support::__private::Vec<
            polkadot_sdk::frame_support::traits::StorageInfo,
        > {
            #[allow(unused_mut)]
            let mut res = ::alloc::vec::Vec::new();
            {
                let mut storage_info = <TestCounter<
                    T,
                > as polkadot_sdk::frame_support::traits::PartialStorageInfoTrait>::partial_storage_info();
                res.append(&mut storage_info);
            }
            res
        }
    }
    use polkadot_sdk::frame_support::traits::{
        StorageInfoTrait, TrackedStorageKey, WhitelistedStorageKeys,
    };
    impl<T: Config> WhitelistedStorageKeys for Pallet<T> {
        fn whitelisted_storage_keys() -> polkadot_sdk::frame_support::__private::Vec<
            TrackedStorageKey,
        > {
            use polkadot_sdk::frame_support::__private::vec;
            <[_]>::into_vec(
                ::alloc::boxed::box_new([
                    TrackedStorageKey::new(TestCounter::<T>::hashed_key().to_vec()),
                ]),
            )
        }
    }
    impl<T> Pallet<T> {
        #[allow(dead_code)]
        #[doc(hidden)]
        pub fn deprecation_info() -> polkadot_sdk::frame_support::__private::metadata_ir::ItemDeprecationInfoIR {
            polkadot_sdk::frame_support::__private::metadata_ir::ItemDeprecationInfoIR::NotDeprecated
        }
    }
    /// Based on [`Config`]. Auto-generated by
    /// [`#[pallet::config(with_default)]`](`frame_support::pallet_macros::config`).
    /// Can be used in tandem with
    /// [`#[register_default_config]`](`frame_support::register_default_config`) and
    /// [`#[derive_impl]`](`frame_support::derive_impl`) to derive test config traits
    /// based on existing pallet config traits in a safe and developer-friendly way.
    ///
    /// See [here](`frame_support::pallet_macros::config`) for more information and caveats about
    /// the auto-generated `DefaultConfig` trait and how it is generated.
    pub trait DefaultConfig: polkadot_sdk::frame_system::DefaultConfig {
        type TestField: Get<u32>;
        type TestField2: Get<u32>;
        type MinerStatus: TypeInfo;
    }
    impl<T: Config> Pallet<T> {
        #[doc(hidden)]
        pub fn pallet_associated_types_metadata() -> polkadot_sdk::frame_support::__private::vec::Vec<
            polkadot_sdk::frame_support::__private::metadata_ir::PalletAssociatedTypeMetadataIR,
        > {
            <[_]>::into_vec(
                ::alloc::boxed::box_new([
                    {
                        polkadot_sdk::frame_support::__private::metadata_ir::PalletAssociatedTypeMetadataIR {
                            name: "MinerStatus",
                            ty: polkadot_sdk::frame_support::__private::scale_info::meta_type::<
                                <T as Config>::MinerStatus,
                            >(),
                            docs: ::alloc::vec::Vec::new(),
                        }
                    },
                ]),
            )
        }
    }
    #[doc(hidden)]
    mod warnings {
        /// This function should not be called and only exists to emit a compiler warning.
        ///
        /// It is a No-OP in any case.
        #[allow(dead_code)]
        #[allow(non_camel_case_types)]
        #[allow(non_snake_case)]
        fn ConstantWeight_0() {
            #[deprecated(
                note = "\n\t\tIt is deprecated to use hard-coded constant as call weight.\n\t\tPlease instead benchmark all calls or put the pallet into `dev` mode.\n\n\t\tFor more info see:\n\t\t\t<https://github.com/paritytech/substrate/pull/13798>"
            )]
            #[allow(non_upper_case_globals)]
            const _w: () = ();
            let _ = _w;
        }
    }
    #[allow(unused_imports)]
    #[doc(hidden)]
    pub mod __substrate_call_check {
        #[doc(hidden)]
        pub use __is_call_part_defined_0 as is_call_part_defined;
    }
    ///Contains a variant per dispatchable extrinsic that this pallet has.
    #[codec(encode_bound())]
    #[codec(decode_bound())]
    #[scale_info(skip_type_params(T), capture_docs = "always")]
    #[allow(non_camel_case_types)]
    pub enum Call<T: Config> {
        #[doc(hidden)]
        #[codec(skip)]
        __Ignore(::core::marker::PhantomData<(T,)>, polkadot_sdk::frame_support::Never),
        #[codec(index = 1u8)]
        test_transactional { #[allow(missing_docs)] #[codec(compact)] val: u64 },
    }
    const _: () = {
        #[automatically_derived]
        #[allow(deprecated)]
        impl<T: Config> ::core::fmt::Debug for Call<T> {
            fn fmt(&self, fmt: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match *self {
                    Self::__Ignore(ref _0, ref _1) => {
                        fmt.debug_tuple("Call::__Ignore").field(&_0).field(&_1).finish()
                    }
                    Self::test_transactional { ref val } => {
                        fmt.debug_struct("Call::test_transactional")
                            .field("val", &val)
                            .finish()
                    }
                }
            }
        }
    };
    const _: () = {
        #[automatically_derived]
        #[allow(deprecated)]
        impl<T: Config> ::core::clone::Clone for Call<T> {
            fn clone(&self) -> Self {
                match self {
                    Self::__Ignore(ref _0, ref _1) => {
                        Self::__Ignore(
                            ::core::clone::Clone::clone(_0),
                            ::core::clone::Clone::clone(_1),
                        )
                    }
                    Self::test_transactional { ref val } => {
                        Self::test_transactional {
                            val: ::core::clone::Clone::clone(val),
                        }
                    }
                }
            }
        }
    };
    #[allow(deprecated)]
    const _: () = {
        impl<T: Config> ::core::cmp::Eq for Call<T> {}
    };
    const _: () = {
        #[automatically_derived]
        #[allow(deprecated)]
        impl<T: Config> ::core::cmp::PartialEq for Call<T> {
            fn eq(&self, other: &Self) -> bool {
                match (self, other) {
                    (Self::__Ignore(_0, _1), Self::__Ignore(_0_other, _1_other)) => {
                        true && _0 == _0_other && _1 == _1_other
                    }
                    (
                        Self::test_transactional { val },
                        Self::test_transactional { val: _0 },
                    ) => true && val == _0,
                    (Self::__Ignore { .. }, Self::test_transactional { .. }) => false,
                    (Self::test_transactional { .. }, Self::__Ignore { .. }) => false,
                }
            }
        }
    };
    #[allow(deprecated)]
    const _: () = {
        #[allow(non_camel_case_types)]
        #[automatically_derived]
        impl<T: Config> ::codec::Encode for Call<T> {
            fn size_hint(&self) -> usize {
                1_usize
                    + match *self {
                        Call::test_transactional { ref val } => {
                            0_usize
                                .saturating_add(
                                    ::codec::Encode::size_hint(
                                        &<<u64 as ::codec::HasCompact>::Type as ::codec::EncodeAsRef<
                                            '_,
                                            u64,
                                        >>::RefType::from(val),
                                    ),
                                )
                        }
                        _ => 0_usize,
                    }
            }
            fn encode_to<__CodecOutputEdqy: ::codec::Output + ?::core::marker::Sized>(
                &self,
                __codec_dest_edqy: &mut __CodecOutputEdqy,
            ) {
                #[automatically_derived]
                const _: () = {
                    #[allow(clippy::unnecessary_cast)]
                    #[allow(clippy::cast_possible_truncation)]
                    const indices: [(usize, &'static str); 1usize] = [
                        ((1usize) as ::core::primitive::usize, "test_transactional"),
                    ];
                    const fn search_for_invalid_index(
                        array: &[(usize, &'static str); 1usize],
                    ) -> (bool, usize) {
                        let mut i = 0;
                        while i < 1usize {
                            if array[i].0 > 255 {
                                return (true, i);
                            }
                            i += 1;
                        }
                        (false, 0)
                    }
                    const INVALID_INDEX: (bool, usize) = search_for_invalid_index(
                        &indices,
                    );
                    if INVALID_INDEX.0 {
                        let msg = ::const_format::pmr::__AssertStr {
                            x: {
                                use ::const_format::__cf_osRcTFl4A;
                                ({
                                    #[doc(hidden)]
                                    #[allow(unused_mut, non_snake_case)]
                                    const CONCATP_NHPMWYD3NJA: &[__cf_osRcTFl4A::pmr::PArgument] = {
                                        let fmt = __cf_osRcTFl4A::pmr::FormattingFlags::NEW;
                                        &[
                                            __cf_osRcTFl4A::pmr::PConvWrapper("Found variant `")
                                                .to_pargument_display(fmt),
                                            __cf_osRcTFl4A::pmr::PConvWrapper(
                                                    indices[INVALID_INDEX.1].1,
                                                )
                                                .to_pargument_display(fmt),
                                            __cf_osRcTFl4A::pmr::PConvWrapper("` with invalid index: `")
                                                .to_pargument_display(fmt),
                                            __cf_osRcTFl4A::pmr::PConvWrapper(
                                                    indices[INVALID_INDEX.1].0,
                                                )
                                                .to_pargument_display(fmt),
                                            __cf_osRcTFl4A::pmr::PConvWrapper(
                                                    "`. Max supported index is 255.",
                                                )
                                                .to_pargument_display(fmt),
                                        ]
                                    };
                                    {
                                        #[doc(hidden)]
                                        const ARR_LEN: usize = ::const_format::pmr::PArgument::calc_len(
                                            CONCATP_NHPMWYD3NJA,
                                        );
                                        #[doc(hidden)]
                                        const CONCAT_ARR: &::const_format::pmr::LenAndArray<
                                            [u8; ARR_LEN],
                                        > = &::const_format::pmr::__priv_concatenate(
                                            CONCATP_NHPMWYD3NJA,
                                        );
                                        #[doc(hidden)]
                                        #[allow(clippy::transmute_ptr_to_ptr)]
                                        const CONCAT_STR: &str = unsafe {
                                            let slice = ::const_format::pmr::transmute::<
                                                &[u8; ARR_LEN],
                                                &[u8; CONCAT_ARR.len],
                                            >(&CONCAT_ARR.array);
                                            {
                                                let bytes: &'static [::const_format::pmr::u8] = slice;
                                                let string: &'static ::const_format::pmr::str = {
                                                    ::const_format::__hidden_utils::PtrToRef {
                                                        ptr: bytes as *const [::const_format::pmr::u8] as *const str,
                                                    }
                                                        .reff
                                                };
                                                string
                                            }
                                        };
                                        CONCAT_STR
                                    }
                                })
                            },
                        }
                            .x;
                        {
                            ::core::panicking::panic_display(&msg);
                        };
                    }
                    const fn duplicate_info(
                        array: &[(usize, &'static str); 1usize],
                    ) -> (bool, usize, usize) {
                        let len = 1usize;
                        let mut i = 0usize;
                        while i < len {
                            let mut j = i + 1;
                            while j < len {
                                if array[i].0 == array[j].0 {
                                    return (true, i, j);
                                }
                                j += 1;
                            }
                            i += 1;
                        }
                        (false, 0, 0)
                    }
                    const DUP_INFO: (bool, usize, usize) = duplicate_info(&indices);
                    if DUP_INFO.0 {
                        let msg = ::const_format::pmr::__AssertStr {
                            x: {
                                use ::const_format::__cf_osRcTFl4A;
                                ({
                                    #[doc(hidden)]
                                    #[allow(unused_mut, non_snake_case)]
                                    const CONCATP_NHPMWYD3NJA: &[__cf_osRcTFl4A::pmr::PArgument] = {
                                        let fmt = __cf_osRcTFl4A::pmr::FormattingFlags::NEW;
                                        &[
                                            __cf_osRcTFl4A::pmr::PConvWrapper(
                                                    "Found variants that have duplicate indexes. Both `",
                                                )
                                                .to_pargument_display(fmt),
                                            __cf_osRcTFl4A::pmr::PConvWrapper(indices[DUP_INFO.1].1)
                                                .to_pargument_display(fmt),
                                            __cf_osRcTFl4A::pmr::PConvWrapper("` and `")
                                                .to_pargument_display(fmt),
                                            __cf_osRcTFl4A::pmr::PConvWrapper(indices[DUP_INFO.2].1)
                                                .to_pargument_display(fmt),
                                            __cf_osRcTFl4A::pmr::PConvWrapper("` have the index `")
                                                .to_pargument_display(fmt),
                                            __cf_osRcTFl4A::pmr::PConvWrapper(indices[DUP_INFO.1].0)
                                                .to_pargument_display(fmt),
                                            __cf_osRcTFl4A::pmr::PConvWrapper(
                                                    "`. Use different indexes for each variant.",
                                                )
                                                .to_pargument_display(fmt),
                                        ]
                                    };
                                    {
                                        #[doc(hidden)]
                                        const ARR_LEN: usize = ::const_format::pmr::PArgument::calc_len(
                                            CONCATP_NHPMWYD3NJA,
                                        );
                                        #[doc(hidden)]
                                        const CONCAT_ARR: &::const_format::pmr::LenAndArray<
                                            [u8; ARR_LEN],
                                        > = &::const_format::pmr::__priv_concatenate(
                                            CONCATP_NHPMWYD3NJA,
                                        );
                                        #[doc(hidden)]
                                        #[allow(clippy::transmute_ptr_to_ptr)]
                                        const CONCAT_STR: &str = unsafe {
                                            let slice = ::const_format::pmr::transmute::<
                                                &[u8; ARR_LEN],
                                                &[u8; CONCAT_ARR.len],
                                            >(&CONCAT_ARR.array);
                                            {
                                                let bytes: &'static [::const_format::pmr::u8] = slice;
                                                let string: &'static ::const_format::pmr::str = {
                                                    ::const_format::__hidden_utils::PtrToRef {
                                                        ptr: bytes as *const [::const_format::pmr::u8] as *const str,
                                                    }
                                                        .reff
                                                };
                                                string
                                            }
                                        };
                                        CONCAT_STR
                                    }
                                })
                            },
                        }
                            .x;
                        {
                            ::core::panicking::panic_display(&msg);
                        };
                    }
                };
                match *self {
                    Call::test_transactional { ref val } => {
                        #[allow(clippy::unnecessary_cast)]
                        __codec_dest_edqy.push_byte((1usize) as ::core::primitive::u8);
                        {
                            ::codec::Encode::encode_to(
                                &<<u64 as ::codec::HasCompact>::Type as ::codec::EncodeAsRef<
                                    '_,
                                    u64,
                                >>::RefType::from(val),
                                __codec_dest_edqy,
                            );
                        }
                    }
                    _ => {}
                }
            }
        }
        #[automatically_derived]
        impl<T: Config> ::codec::EncodeLike for Call<T> {}
    };
    #[allow(deprecated)]
    const _: () = {
        #[allow(non_camel_case_types)]
        #[automatically_derived]
        impl<T: Config> ::codec::Decode for Call<T> {
            fn decode<__CodecInputEdqy: ::codec::Input>(
                __codec_input_edqy: &mut __CodecInputEdqy,
            ) -> ::core::result::Result<Self, ::codec::Error> {
                #[automatically_derived]
                const _: () = {
                    #[allow(clippy::unnecessary_cast)]
                    #[allow(clippy::cast_possible_truncation)]
                    const indices: [(usize, &'static str); 1usize] = [
                        ((1usize) as ::core::primitive::usize, "test_transactional"),
                    ];
                    const fn search_for_invalid_index(
                        array: &[(usize, &'static str); 1usize],
                    ) -> (bool, usize) {
                        let mut i = 0;
                        while i < 1usize {
                            if array[i].0 > 255 {
                                return (true, i);
                            }
                            i += 1;
                        }
                        (false, 0)
                    }
                    const INVALID_INDEX: (bool, usize) = search_for_invalid_index(
                        &indices,
                    );
                    if INVALID_INDEX.0 {
                        let msg = ::const_format::pmr::__AssertStr {
                            x: {
                                use ::const_format::__cf_osRcTFl4A;
                                ({
                                    #[doc(hidden)]
                                    #[allow(unused_mut, non_snake_case)]
                                    const CONCATP_NHPMWYD3NJA: &[__cf_osRcTFl4A::pmr::PArgument] = {
                                        let fmt = __cf_osRcTFl4A::pmr::FormattingFlags::NEW;
                                        &[
                                            __cf_osRcTFl4A::pmr::PConvWrapper("Found variant `")
                                                .to_pargument_display(fmt),
                                            __cf_osRcTFl4A::pmr::PConvWrapper(
                                                    indices[INVALID_INDEX.1].1,
                                                )
                                                .to_pargument_display(fmt),
                                            __cf_osRcTFl4A::pmr::PConvWrapper("` with invalid index: `")
                                                .to_pargument_display(fmt),
                                            __cf_osRcTFl4A::pmr::PConvWrapper(
                                                    indices[INVALID_INDEX.1].0,
                                                )
                                                .to_pargument_display(fmt),
                                            __cf_osRcTFl4A::pmr::PConvWrapper(
                                                    "`. Max supported index is 255.",
                                                )
                                                .to_pargument_display(fmt),
                                        ]
                                    };
                                    {
                                        #[doc(hidden)]
                                        const ARR_LEN: usize = ::const_format::pmr::PArgument::calc_len(
                                            CONCATP_NHPMWYD3NJA,
                                        );
                                        #[doc(hidden)]
                                        const CONCAT_ARR: &::const_format::pmr::LenAndArray<
                                            [u8; ARR_LEN],
                                        > = &::const_format::pmr::__priv_concatenate(
                                            CONCATP_NHPMWYD3NJA,
                                        );
                                        #[doc(hidden)]
                                        #[allow(clippy::transmute_ptr_to_ptr)]
                                        const CONCAT_STR: &str = unsafe {
                                            let slice = ::const_format::pmr::transmute::<
                                                &[u8; ARR_LEN],
                                                &[u8; CONCAT_ARR.len],
                                            >(&CONCAT_ARR.array);
                                            {
                                                let bytes: &'static [::const_format::pmr::u8] = slice;
                                                let string: &'static ::const_format::pmr::str = {
                                                    ::const_format::__hidden_utils::PtrToRef {
                                                        ptr: bytes as *const [::const_format::pmr::u8] as *const str,
                                                    }
                                                        .reff
                                                };
                                                string
                                            }
                                        };
                                        CONCAT_STR
                                    }
                                })
                            },
                        }
                            .x;
                        {
                            ::core::panicking::panic_display(&msg);
                        };
                    }
                    const fn duplicate_info(
                        array: &[(usize, &'static str); 1usize],
                    ) -> (bool, usize, usize) {
                        let len = 1usize;
                        let mut i = 0usize;
                        while i < len {
                            let mut j = i + 1;
                            while j < len {
                                if array[i].0 == array[j].0 {
                                    return (true, i, j);
                                }
                                j += 1;
                            }
                            i += 1;
                        }
                        (false, 0, 0)
                    }
                    const DUP_INFO: (bool, usize, usize) = duplicate_info(&indices);
                    if DUP_INFO.0 {
                        let msg = ::const_format::pmr::__AssertStr {
                            x: {
                                use ::const_format::__cf_osRcTFl4A;
                                ({
                                    #[doc(hidden)]
                                    #[allow(unused_mut, non_snake_case)]
                                    const CONCATP_NHPMWYD3NJA: &[__cf_osRcTFl4A::pmr::PArgument] = {
                                        let fmt = __cf_osRcTFl4A::pmr::FormattingFlags::NEW;
                                        &[
                                            __cf_osRcTFl4A::pmr::PConvWrapper(
                                                    "Found variants that have duplicate indexes. Both `",
                                                )
                                                .to_pargument_display(fmt),
                                            __cf_osRcTFl4A::pmr::PConvWrapper(indices[DUP_INFO.1].1)
                                                .to_pargument_display(fmt),
                                            __cf_osRcTFl4A::pmr::PConvWrapper("` and `")
                                                .to_pargument_display(fmt),
                                            __cf_osRcTFl4A::pmr::PConvWrapper(indices[DUP_INFO.2].1)
                                                .to_pargument_display(fmt),
                                            __cf_osRcTFl4A::pmr::PConvWrapper("` have the index `")
                                                .to_pargument_display(fmt),
                                            __cf_osRcTFl4A::pmr::PConvWrapper(indices[DUP_INFO.1].0)
                                                .to_pargument_display(fmt),
                                            __cf_osRcTFl4A::pmr::PConvWrapper(
                                                    "`. Use different indexes for each variant.",
                                                )
                                                .to_pargument_display(fmt),
                                        ]
                                    };
                                    {
                                        #[doc(hidden)]
                                        const ARR_LEN: usize = ::const_format::pmr::PArgument::calc_len(
                                            CONCATP_NHPMWYD3NJA,
                                        );
                                        #[doc(hidden)]
                                        const CONCAT_ARR: &::const_format::pmr::LenAndArray<
                                            [u8; ARR_LEN],
                                        > = &::const_format::pmr::__priv_concatenate(
                                            CONCATP_NHPMWYD3NJA,
                                        );
                                        #[doc(hidden)]
                                        #[allow(clippy::transmute_ptr_to_ptr)]
                                        const CONCAT_STR: &str = unsafe {
                                            let slice = ::const_format::pmr::transmute::<
                                                &[u8; ARR_LEN],
                                                &[u8; CONCAT_ARR.len],
                                            >(&CONCAT_ARR.array);
                                            {
                                                let bytes: &'static [::const_format::pmr::u8] = slice;
                                                let string: &'static ::const_format::pmr::str = {
                                                    ::const_format::__hidden_utils::PtrToRef {
                                                        ptr: bytes as *const [::const_format::pmr::u8] as *const str,
                                                    }
                                                        .reff
                                                };
                                                string
                                            }
                                        };
                                        CONCAT_STR
                                    }
                                })
                            },
                        }
                            .x;
                        {
                            ::core::panicking::panic_display(&msg);
                        };
                    }
                };
                match __codec_input_edqy
                    .read_byte()
                    .map_err(|e| {
                        e.chain("Could not decode `Call`, failed to read variant byte")
                    })?
                {
                    #[allow(clippy::unnecessary_cast)]
                    #[allow(clippy::cast_possible_truncation)]
                    __codec_x_edqy if __codec_x_edqy
                        == (1usize) as ::core::primitive::u8 => {
                        #[allow(clippy::redundant_closure_call)]
                        return (move || {
                            ::core::result::Result::Ok(Call::test_transactional::<T> {
                                val: {
                                    let __codec_res_edqy = <<u64 as ::codec::HasCompact>::Type as ::codec::Decode>::decode(
                                        __codec_input_edqy,
                                    );
                                    match __codec_res_edqy {
                                        ::core::result::Result::Err(e) => {
                                            return ::core::result::Result::Err(
                                                e.chain("Could not decode `Call::test_transactional::val`"),
                                            );
                                        }
                                        ::core::result::Result::Ok(__codec_res_edqy) => {
                                            __codec_res_edqy.into()
                                        }
                                    }
                                },
                            })
                        })();
                    }
                    _ => {
                        #[allow(clippy::redundant_closure_call)]
                        return (move || {
                            ::core::result::Result::Err(
                                <_ as ::core::convert::Into<
                                    _,
                                >>::into("Could not decode `Call`, variant doesn't exist"),
                            )
                        })();
                    }
                }
            }
        }
    };
    #[allow(deprecated)]
    const _: () = {
        #[allow(non_camel_case_types)]
        fn check_struct<T: Config>() {
            fn check_field<T: ::codec::DecodeWithMemTracking>() {}
            check_field::<<u64 as ::codec::HasCompact>::Type>();
        }
        #[automatically_derived]
        impl<T: Config> ::codec::DecodeWithMemTracking for Call<T> {}
    };
    #[allow(
        non_upper_case_globals,
        deprecated,
        unused_attributes,
        unused_qualifications
    )]
    const _: () = {
        impl<T: Config> ::scale_info::TypeInfo for Call<T>
        where
            ::core::marker::PhantomData<(T,)>: ::scale_info::TypeInfo + 'static,
            T: Config + 'static,
        {
            type Identity = Self;
            fn type_info() -> ::scale_info::Type {
                ::scale_info::Type::builder()
                    .path(
                        ::scale_info::Path::new_with_replace(
                            "Call",
                            "polkadot_test::pallet",
                            &[],
                        ),
                    )
                    .type_params(
                        <[_]>::into_vec(
                            ::alloc::boxed::box_new([
                                ::scale_info::TypeParameter::new(
                                    "T",
                                    ::core::option::Option::None,
                                ),
                            ]),
                        ),
                    )
                    .docs_always(
                        &[
                            "Contains a variant per dispatchable extrinsic that this pallet has.",
                        ],
                    )
                    .variant(
                        ::scale_info::build::Variants::new()
                            .variant(
                                "test_transactional",
                                |v| {
                                    v
                                        .index(1u8 as ::core::primitive::u8)
                                        .fields(
                                            ::scale_info::build::Fields::named()
                                                .field(|f| f.compact::<u64>().name("val").type_name("u64")),
                                        )
                                },
                            ),
                    )
            }
        }
    };
    impl<T: Config> Call<T> {
        ///Create a call with the variant `test_transactional`.
        pub fn new_call_variant_test_transactional(val: u64) -> Self {
            Self::test_transactional { val }
        }
    }
    impl<T: Config> polkadot_sdk::frame_support::dispatch::GetDispatchInfo for Call<T> {
        fn get_dispatch_info(
            &self,
        ) -> polkadot_sdk::frame_support::dispatch::DispatchInfo {
            match *self {
                Self::test_transactional { ref val } => {
                    let __pallet_base_weight = 1_000_000;
                    let __pallet_weight = <dyn polkadot_sdk::frame_support::dispatch::WeighData<
                        (&u64,),
                    >>::weigh_data(&__pallet_base_weight, (val,));
                    let __pallet_class = <dyn polkadot_sdk::frame_support::dispatch::ClassifyDispatch<
                        (&u64,),
                    >>::classify_dispatch(&__pallet_base_weight, (val,));
                    let __pallet_pays_fee = <dyn polkadot_sdk::frame_support::dispatch::PaysFee<
                        (&u64,),
                    >>::pays_fee(&__pallet_base_weight, (val,));
                    polkadot_sdk::frame_support::dispatch::DispatchInfo {
                        call_weight: __pallet_weight,
                        extension_weight: Default::default(),
                        class: __pallet_class,
                        pays_fee: __pallet_pays_fee,
                    }
                }
                Self::__Ignore(_, _) => {
                    ::core::panicking::panic_fmt(
                        format_args!(
                            "internal error: entered unreachable code: {0}",
                            format_args!("__Ignore cannot be used"),
                        ),
                    );
                }
            }
        }
    }
    impl<T: Config> polkadot_sdk::frame_support::dispatch::CheckIfFeeless for Call<T> {
        type Origin = polkadot_sdk::frame_system::pallet_prelude::OriginFor<T>;
        #[allow(unused_variables)]
        fn is_feeless(&self, origin: &Self::Origin) -> bool {
            match *self {
                Self::test_transactional { ref val } => {
                    let feeless_check = |_origin, val| { false };
                    feeless_check(origin, val)
                }
                Self::__Ignore(_, _) => {
                    ::core::panicking::panic_fmt(
                        format_args!(
                            "internal error: entered unreachable code: {0}",
                            format_args!("__Ignore cannot be used"),
                        ),
                    );
                }
            }
        }
    }
    impl<T: Config> polkadot_sdk::frame_support::traits::GetCallName for Call<T> {
        fn get_call_name(&self) -> &'static str {
            match *self {
                Self::test_transactional { .. } => "test_transactional",
                Self::__Ignore(_, _) => {
                    ::core::panicking::panic_fmt(
                        format_args!(
                            "internal error: entered unreachable code: {0}",
                            format_args!("__PhantomItem cannot be used."),
                        ),
                    );
                }
            }
        }
        fn get_call_names() -> &'static [&'static str] {
            &["test_transactional"]
        }
    }
    impl<T: Config> polkadot_sdk::frame_support::traits::GetCallIndex for Call<T> {
        fn get_call_index(&self) -> u8 {
            match *self {
                Self::test_transactional { .. } => 1u8,
                Self::__Ignore(_, _) => {
                    ::core::panicking::panic_fmt(
                        format_args!(
                            "internal error: entered unreachable code: {0}",
                            format_args!("__PhantomItem cannot be used."),
                        ),
                    );
                }
            }
        }
        fn get_call_indices() -> &'static [u8] {
            &[1u8]
        }
    }
    impl<T: Config> polkadot_sdk::frame_support::traits::UnfilteredDispatchable
    for Call<T> {
        type RuntimeOrigin = polkadot_sdk::frame_system::pallet_prelude::OriginFor<T>;
        fn dispatch_bypass_filter(
            self,
            origin: Self::RuntimeOrigin,
        ) -> polkadot_sdk::frame_support::dispatch::DispatchResultWithPostInfo {
            polkadot_sdk::frame_support::dispatch_context::run_in_context(|| {
                match self {
                    Self::test_transactional { val } => {
                        #[allow(clippy::useless_conversion)]
                        <Pallet<T>>::test_transactional(origin, val)
                            .map(Into::into)
                            .map_err(Into::into)
                    }
                    Self::__Ignore(_, _) => {
                        let _ = origin;
                        {
                            ::core::panicking::panic_fmt(
                                format_args!(
                                    "internal error: entered unreachable code: {0}",
                                    format_args!("__PhantomItem cannot be used."),
                                ),
                            );
                        };
                    }
                }
            })
        }
    }
    impl<T: Config> polkadot_sdk::frame_support::dispatch::Callable<T> for Pallet<T> {
        type RuntimeCall = Call<T>;
    }
    impl<T: Config> Pallet<T> {
        #[allow(dead_code)]
        #[doc(hidden)]
        pub fn call_functions() -> polkadot_sdk::frame_support::__private::metadata_ir::PalletCallMetadataIR {
            polkadot_sdk::frame_support::__private::metadata_ir::PalletCallMetadataIR {
                ty: polkadot_sdk::frame_support::__private::scale_info::meta_type::<
                    Call<T>,
                >(),
                deprecation_info: polkadot_sdk::frame_support::__private::metadata_ir::EnumDeprecationInfoIR::nothing_deprecated(),
            }
        }
    }
    impl<T: Config> polkadot_sdk::frame_support::traits::Authorize for Call<T> {
        fn authorize(
            &self,
            source: polkadot_sdk::frame_support::pallet_prelude::TransactionSource,
        ) -> ::core::option::Option<
            ::core::result::Result<
                (
                    polkadot_sdk::frame_support::pallet_prelude::ValidTransaction,
                    polkadot_sdk::frame_support::pallet_prelude::Weight,
                ),
                polkadot_sdk::frame_support::pallet_prelude::TransactionValidityError,
            >,
        > {
            match *self {
                Self::test_transactional { ref val } => None,
                Self::__Ignore(_, _) => {
                    let _ = source;
                    {
                        ::core::panicking::panic_fmt(
                            format_args!(
                                "internal error: entered unreachable code: {0}",
                                format_args!("__Ignore cannot be used"),
                            ),
                        );
                    }
                }
            }
        }
        fn weight_of_authorize(
            &self,
        ) -> polkadot_sdk::frame_support::pallet_prelude::Weight {
            match *self {
                Self::test_transactional { ref val } => {
                    polkadot_sdk::frame_support::pallet_prelude::Weight::zero()
                }
                Self::__Ignore(_, _) => {
                    ::core::panicking::panic_fmt(
                        format_args!(
                            "internal error: entered unreachable code: {0}",
                            format_args!("__Ignore cannot be used"),
                        ),
                    );
                }
            }
        }
    }
    pub use __tt_error_token_1 as tt_error_token;
    #[doc(hidden)]
    pub mod __substrate_event_check {
        #[doc(hidden)]
        pub use __is_event_part_defined_2 as is_event_part_defined;
    }
    impl<T: Config> Pallet<T> {
        #[doc(hidden)]
        pub fn storage_metadata() -> polkadot_sdk::frame_support::__private::metadata_ir::PalletStorageMetadataIR {
            polkadot_sdk::frame_support::__private::metadata_ir::PalletStorageMetadataIR {
                prefix: <<T as polkadot_sdk::frame_system::Config>::PalletInfo as polkadot_sdk::frame_support::traits::PalletInfo>::name::<
                    Pallet<T>,
                >()
                    .expect(
                        "No name found for the pallet in the runtime! This usually means that the pallet wasn't added to `construct_runtime!`.",
                    ),
                entries: {
                    #[allow(unused_mut)]
                    let mut entries = ::alloc::vec::Vec::new();
                    (|entries: &mut polkadot_sdk::frame_support::__private::Vec<_>| {
                        {
                            <TestCounter<
                                T,
                            > as polkadot_sdk::frame_support::storage::StorageEntryMetadataBuilder>::build_metadata(
                                polkadot_sdk::frame_support::__private::metadata_ir::ItemDeprecationInfoIR::NotDeprecated,
                                ::alloc::vec::Vec::new(),
                                entries,
                            );
                        }
                    })(&mut entries);
                    entries
                },
            }
        }
    }
    #[doc(hidden)]
    pub struct _GeneratedPrefixForStorageTestCounter<T>(core::marker::PhantomData<(T,)>);
    impl<T: Config> polkadot_sdk::frame_support::traits::StorageInstance
    for _GeneratedPrefixForStorageTestCounter<T> {
        fn pallet_prefix() -> &'static str {
            <<T as polkadot_sdk::frame_system::Config>::PalletInfo as polkadot_sdk::frame_support::traits::PalletInfo>::name::<
                Pallet<T>,
            >()
                .expect(
                    "No name found for the pallet in the runtime! This usually means that the pallet wasn't added to `construct_runtime!`.",
                )
        }
        fn pallet_prefix_hash() -> [u8; 16] {
            <<T as polkadot_sdk::frame_system::Config>::PalletInfo as polkadot_sdk::frame_support::traits::PalletInfo>::name_hash::<
                Pallet<T>,
            >()
                .expect(
                    "No name_hash found for the pallet in the runtime! This usually means that the pallet wasn't added to `construct_runtime!`.",
                )
        }
        const STORAGE_PREFIX: &'static str = "TestCounter";
        fn storage_prefix_hash() -> [u8; 16] {
            [
                70u8, 239u8, 121u8, 253u8, 222u8, 93u8, 0u8, 223u8, 195u8, 188u8, 215u8,
                53u8, 110u8, 201u8, 33u8, 224u8,
            ]
        }
    }
    #[allow(deprecated)]
    impl<T: Config> polkadot_sdk::frame_support::traits::TryDecodeEntireStorage
    for Pallet<T> {
        fn try_decode_entire_state() -> Result<
            usize,
            polkadot_sdk::frame_support::__private::Vec<
                polkadot_sdk::frame_support::traits::TryDecodeEntireStorageError,
            >,
        > {
            let pallet_name = <<T as polkadot_sdk::frame_system::Config>::PalletInfo as polkadot_sdk::frame_support::traits::PalletInfo>::name::<
                Pallet<T>,
            >()
                .expect("Every active pallet has a name in the runtime; qed");
            {
                {
                    {
                        let lvl = ::log::Level::Debug;
                        if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                            ::log::__private_api::log(
                                { ::log::__private_api::GlobalLogger },
                                format_args!("trying to decode pallet: {0}", pallet_name),
                                lvl,
                                &(
                                    "runtime::try-decode-state",
                                    "polkadot_test::pallet",
                                    ::log::__private_api::loc(),
                                ),
                                (),
                            );
                        }
                    }
                }
            };
            let mut errors = polkadot_sdk::frame_support::__private::Vec::new();
            let mut decoded = 0usize;
            if errors.is_empty() { Ok(decoded) } else { Err(errors) }
        }
    }
    impl<T: Config> polkadot_sdk::frame_support::view_functions::ViewFunctionIdPrefix
    for Pallet<T> {
        fn prefix() -> [::core::primitive::u8; 16usize] {
            <<T as polkadot_sdk::frame_system::Config>::PalletInfo as polkadot_sdk::frame_support::traits::PalletInfo>::name_hash::<
                Pallet<T>,
            >()
                .expect(
                    "No name_hash found for the pallet in the runtime! This usually means that the pallet wasn't added to `construct_runtime!`.",
                )
        }
    }
    impl<T: Config> polkadot_sdk::frame_support::view_functions::DispatchViewFunction
    for Pallet<T> {
        #[deny(unreachable_patterns)]
        fn dispatch_view_function<
            O: polkadot_sdk::frame_support::__private::codec::Output,
        >(
            id: &polkadot_sdk::frame_support::view_functions::ViewFunctionId,
            input: &mut &[u8],
            output: &mut O,
        ) -> Result<
            (),
            polkadot_sdk::frame_support::view_functions::ViewFunctionDispatchError,
        > {
            match id.suffix {
                _ => {
                    Err(
                        polkadot_sdk::frame_support::view_functions::ViewFunctionDispatchError::NotFound(
                            id.clone(),
                        ),
                    )
                }
            }
        }
    }
    impl<T: Config> Pallet<T> {
        #[doc(hidden)]
        pub fn pallet_view_functions_metadata() -> polkadot_sdk::frame_support::__private::Vec<
            polkadot_sdk::frame_support::__private::metadata_ir::PalletViewFunctionMetadataIR,
        > {
            ::alloc::vec::Vec::new()
        }
    }
    #[doc(hidden)]
    pub mod __substrate_inherent_check {
        #[doc(hidden)]
        pub use __is_inherent_part_defined_3 as is_inherent_part_defined;
    }
    /// Hidden instance generated to be internally used when module is used without
    /// instance.
    #[doc(hidden)]
    pub type __InherentHiddenInstance = ();
    impl<
        T: Config,
    > polkadot_sdk::frame_support::traits::Hooks<
        polkadot_sdk::frame_system::pallet_prelude::BlockNumberFor<T>,
    > for Pallet<T> {}
    impl<
        T: Config,
    > polkadot_sdk::frame_support::traits::OnFinalize<
        polkadot_sdk::frame_system::pallet_prelude::BlockNumberFor<T>,
    > for Pallet<T> {
        fn on_finalize(
            n: polkadot_sdk::frame_system::pallet_prelude::BlockNumberFor<T>,
        ) {
            <Self as polkadot_sdk::frame_support::traits::Hooks<
                polkadot_sdk::frame_system::pallet_prelude::BlockNumberFor<T>,
            >>::on_finalize(n)
        }
    }
    impl<
        T: Config,
    > polkadot_sdk::frame_support::traits::OnIdle<
        polkadot_sdk::frame_system::pallet_prelude::BlockNumberFor<T>,
    > for Pallet<T> {
        fn on_idle(
            n: polkadot_sdk::frame_system::pallet_prelude::BlockNumberFor<T>,
            remaining_weight: polkadot_sdk::frame_support::weights::Weight,
        ) -> polkadot_sdk::frame_support::weights::Weight {
            <Self as polkadot_sdk::frame_support::traits::Hooks<
                polkadot_sdk::frame_system::pallet_prelude::BlockNumberFor<T>,
            >>::on_idle(n, remaining_weight)
        }
    }
    impl<
        T: Config,
    > polkadot_sdk::frame_support::traits::OnPoll<
        polkadot_sdk::frame_system::pallet_prelude::BlockNumberFor<T>,
    > for Pallet<T> {
        fn on_poll(
            n: polkadot_sdk::frame_system::pallet_prelude::BlockNumberFor<T>,
            weight: &mut polkadot_sdk::frame_support::weights::WeightMeter,
        ) {
            <Self as polkadot_sdk::frame_support::traits::Hooks<
                polkadot_sdk::frame_system::pallet_prelude::BlockNumberFor<T>,
            >>::on_poll(n, weight);
        }
    }
    impl<
        T: Config,
    > polkadot_sdk::frame_support::traits::OnInitialize<
        polkadot_sdk::frame_system::pallet_prelude::BlockNumberFor<T>,
    > for Pallet<T> {
        fn on_initialize(
            n: polkadot_sdk::frame_system::pallet_prelude::BlockNumberFor<T>,
        ) -> polkadot_sdk::frame_support::weights::Weight {
            <Self as polkadot_sdk::frame_support::traits::Hooks<
                polkadot_sdk::frame_system::pallet_prelude::BlockNumberFor<T>,
            >>::on_initialize(n)
        }
    }
    impl<T: Config> polkadot_sdk::frame_support::traits::BeforeAllRuntimeMigrations
    for Pallet<T> {
        fn before_all_runtime_migrations() -> polkadot_sdk::frame_support::weights::Weight {
            use polkadot_sdk::frame_support::traits::{Get, PalletInfoAccess};
            use polkadot_sdk::frame_support::__private::hashing::twox_128;
            use polkadot_sdk::frame_support::storage::unhashed::contains_prefixed_key;
            let pallet_hashed_prefix = <Self as PalletInfoAccess>::name_hash();
            let exists = contains_prefixed_key(&pallet_hashed_prefix);
            if !exists {
                let default_version = polkadot_sdk::frame_support::traits::StorageVersion::new(
                    0,
                );
                {
                    {
                        {
                            let lvl = ::log::Level::Info;
                            if lvl <= ::log::STATIC_MAX_LEVEL
                                && lvl <= ::log::max_level()
                            {
                                ::log::__private_api::log(
                                    { ::log::__private_api::GlobalLogger },
                                    format_args!(
                                        "🐥 New pallet {0:?} detected in the runtime. The pallet has no defined storage version, so the on-chain version is being initialized to {1:?}.",
                                        <<T as polkadot_sdk::frame_system::Config>::PalletInfo as polkadot_sdk::frame_support::traits::PalletInfo>::name::<
                                            Self,
                                        >()
                                            .unwrap_or("<unknown pallet name>"),
                                        default_version,
                                    ),
                                    lvl,
                                    &(
                                        polkadot_sdk::frame_support::LOG_TARGET,
                                        "polkadot_test::pallet",
                                        ::log::__private_api::loc(),
                                    ),
                                    (),
                                );
                            }
                        }
                    }
                };
                default_version.put::<Self>();
                <T as polkadot_sdk::frame_system::Config>::DbWeight::get()
                    .reads_writes(1, 1)
            } else {
                <T as polkadot_sdk::frame_system::Config>::DbWeight::get().reads(1)
            }
        }
    }
    impl<T: Config> polkadot_sdk::frame_support::traits::OnRuntimeUpgrade for Pallet<T> {
        fn on_runtime_upgrade() -> polkadot_sdk::frame_support::weights::Weight {
            {
                {
                    {
                        let lvl = ::log::Level::Debug;
                        if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                            ::log::__private_api::log(
                                { ::log::__private_api::GlobalLogger },
                                format_args!(
                                    "✅ no migration for {0}",
                                    <<T as polkadot_sdk::frame_system::Config>::PalletInfo as polkadot_sdk::frame_support::traits::PalletInfo>::name::<
                                        Self,
                                    >()
                                        .unwrap_or("<unknown pallet name>"),
                                ),
                                lvl,
                                &(
                                    polkadot_sdk::frame_support::LOG_TARGET,
                                    "polkadot_test::pallet",
                                    ::log::__private_api::loc(),
                                ),
                                (),
                            );
                        }
                    }
                }
            };
            <Self as polkadot_sdk::frame_support::traits::Hooks<
                polkadot_sdk::frame_system::pallet_prelude::BlockNumberFor<T>,
            >>::on_runtime_upgrade()
        }
        fn pre_upgrade() -> Result<
            polkadot_sdk::frame_support::__private::Vec<u8>,
            polkadot_sdk::frame_support::sp_runtime::TryRuntimeError,
        > {
            <Self as polkadot_sdk::frame_support::traits::Hooks<
                polkadot_sdk::frame_system::pallet_prelude::BlockNumberFor<T>,
            >>::pre_upgrade()
        }
        fn post_upgrade(
            state: polkadot_sdk::frame_support::__private::Vec<u8>,
        ) -> Result<(), polkadot_sdk::frame_support::sp_runtime::TryRuntimeError> {
            let on_chain_version = <Self as polkadot_sdk::frame_support::traits::GetStorageVersion>::on_chain_storage_version();
            if on_chain_version
                != polkadot_sdk::frame_support::traits::StorageVersion::new(0)
            {
                {
                    {
                        {
                            let lvl = ::log::Level::Error;
                            if lvl <= ::log::STATIC_MAX_LEVEL
                                && lvl <= ::log::max_level()
                            {
                                ::log::__private_api::log(
                                    { ::log::__private_api::GlobalLogger },
                                    format_args!(
                                        "{0}: On chain storage version {1:?} is set to non zero, while the pallet is missing the `#[pallet::storage_version(VERSION)]` attribute.",
                                        <<T as polkadot_sdk::frame_system::Config>::PalletInfo as polkadot_sdk::frame_support::traits::PalletInfo>::name::<
                                            Self,
                                        >()
                                            .unwrap_or("<unknown pallet name>"),
                                        on_chain_version,
                                    ),
                                    lvl,
                                    &(
                                        polkadot_sdk::frame_support::LOG_TARGET,
                                        "polkadot_test::pallet",
                                        ::log::__private_api::loc(),
                                    ),
                                    (),
                                );
                            }
                        }
                    }
                };
                return Err(
                    "On chain storage version set, while the pallet doesn't \
							have the `#[pallet::storage_version(VERSION)]` attribute."
                        .into(),
                );
            }
            <Self as polkadot_sdk::frame_support::traits::Hooks<
                polkadot_sdk::frame_system::pallet_prelude::BlockNumberFor<T>,
            >>::post_upgrade(state)
        }
    }
    impl<
        T: Config,
    > polkadot_sdk::frame_support::traits::OffchainWorker<
        polkadot_sdk::frame_system::pallet_prelude::BlockNumberFor<T>,
    > for Pallet<T> {
        fn offchain_worker(
            n: polkadot_sdk::frame_system::pallet_prelude::BlockNumberFor<T>,
        ) {
            <Self as polkadot_sdk::frame_support::traits::Hooks<
                polkadot_sdk::frame_system::pallet_prelude::BlockNumberFor<T>,
            >>::offchain_worker(n)
        }
    }
    impl<
        T: Config,
    > polkadot_sdk::frame_support::traits::TryState<
        polkadot_sdk::frame_system::pallet_prelude::BlockNumberFor<T>,
    > for Pallet<T> {
        fn try_state(
            n: polkadot_sdk::frame_system::pallet_prelude::BlockNumberFor<T>,
            _s: polkadot_sdk::frame_support::traits::TryStateSelect,
        ) -> Result<(), polkadot_sdk::frame_support::sp_runtime::TryRuntimeError> {
            {
                {
                    {
                        let lvl = ::log::Level::Info;
                        if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                            ::log::__private_api::log(
                                { ::log::__private_api::GlobalLogger },
                                format_args!(
                                    "🩺 Running {0:?} try-state checks",
                                    <<T as polkadot_sdk::frame_system::Config>::PalletInfo as polkadot_sdk::frame_support::traits::PalletInfo>::name::<
                                        Self,
                                    >()
                                        .unwrap_or("<unknown pallet name>"),
                                ),
                                lvl,
                                &(
                                    polkadot_sdk::frame_support::LOG_TARGET,
                                    "polkadot_test::pallet",
                                    ::log::__private_api::loc(),
                                ),
                                (),
                            );
                        }
                    }
                }
            };
            <Self as polkadot_sdk::frame_support::traits::Hooks<
                polkadot_sdk::frame_system::pallet_prelude::BlockNumberFor<T>,
            >>::try_state(n)
                .inspect_err(|err| {
                    {
                        {
                            {
                                let lvl = ::log::Level::Error;
                                if lvl <= ::log::STATIC_MAX_LEVEL
                                    && lvl <= ::log::max_level()
                                {
                                    ::log::__private_api::log(
                                        { ::log::__private_api::GlobalLogger },
                                        format_args!(
                                            "❌ {0:?} try_state checks failed: {1:?}",
                                            <<T as polkadot_sdk::frame_system::Config>::PalletInfo as polkadot_sdk::frame_support::traits::PalletInfo>::name::<
                                                Self,
                                            >()
                                                .unwrap_or("<unknown pallet name>"),
                                            err,
                                        ),
                                        lvl,
                                        &(
                                            polkadot_sdk::frame_support::LOG_TARGET,
                                            "polkadot_test::pallet",
                                            ::log::__private_api::loc(),
                                        ),
                                        (),
                                    );
                                }
                            }
                        }
                    };
                })
        }
    }
    #[doc(hidden)]
    pub mod __substrate_genesis_config_check {
        #[doc(hidden)]
        pub use __is_genesis_config_defined_4 as is_genesis_config_defined;
        #[doc(hidden)]
        pub use __is_std_macro_defined_for_genesis_4 as is_std_enabled_for_genesis;
    }
    #[doc(hidden)]
    pub mod __substrate_origin_check {
        #[doc(hidden)]
        pub use __is_origin_part_defined_5 as is_origin_part_defined;
    }
    #[doc(hidden)]
    pub mod __substrate_validate_unsigned_check {
        #[doc(hidden)]
        pub use __is_validate_unsigned_part_defined_6 as is_validate_unsigned_part_defined;
    }
    pub use __tt_default_parts_7 as tt_default_parts;
    pub use __tt_extra_parts_7 as tt_extra_parts;
    pub use __tt_default_parts_v2_7 as tt_default_parts_v2;
}
pub trait Config {
    type SpecialType;
    type RegularType;
}
pub struct ExtendMyConfig;
#[doc(hidden)]
mod sp_api_hidden_includes_construct_runtime {
    pub use polkadot_sdk::frame_support as hidden_include;
}
const _: () = {
    #[allow(unused)]
    type __HiddenUseOfUncheckedExtrinsic = <<Runtime as polkadot_sdk::frame_system::Config>::Block as self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_runtime::traits::Block>::Extrinsic;
};
pub struct Runtime;
#[automatically_derived]
impl ::core::clone::Clone for Runtime {
    #[inline]
    fn clone(&self) -> Runtime {
        *self
    }
}
#[automatically_derived]
impl ::core::marker::Copy for Runtime {}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for Runtime {}
#[automatically_derived]
impl ::core::cmp::PartialEq for Runtime {
    #[inline]
    fn eq(&self, other: &Runtime) -> bool {
        true
    }
}
#[automatically_derived]
impl ::core::cmp::Eq for Runtime {
    #[inline]
    #[doc(hidden)]
    #[coverage(off)]
    fn assert_receiver_is_total_eq(&self) -> () {}
}
impl core::fmt::Debug for Runtime {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
        fmt.debug_tuple("Runtime").finish()
    }
}
#[allow(non_upper_case_globals, deprecated, unused_attributes, unused_qualifications)]
const _: () = {
    impl ::scale_info::TypeInfo for Runtime {
        type Identity = Self;
        fn type_info() -> ::scale_info::Type {
            ::scale_info::Type::builder()
                .path(
                    ::scale_info::Path::new_with_replace("Runtime", "polkadot_test", &[]),
                )
                .type_params(::alloc::vec::Vec::new())
                .composite(::scale_info::build::Fields::unit())
        }
    }
};
impl self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_runtime::traits::GetRuntimeBlockType
for Runtime {
    type RuntimeBlock = <Runtime as polkadot_sdk::frame_system::Config>::Block;
}
#[doc(hidden)]
trait InternalConstructRuntime {
    #[inline(always)]
    fn runtime_metadata(
        &self,
    ) -> self::sp_api_hidden_includes_construct_runtime::hidden_include::__private::Vec<
        self::sp_api_hidden_includes_construct_runtime::hidden_include::__private::metadata_ir::RuntimeApiMetadataIR,
    > {
        Default::default()
    }
}
#[doc(hidden)]
impl InternalConstructRuntime for &Runtime {}
#[allow(non_camel_case_types)]
pub enum RuntimeEvent {
    #[codec(index = 0u8)]
    System(frame_system::Event<Runtime>),
    #[codec(index = 2u8)]
    Balances(pallet_balances::Event<Runtime>),
    #[codec(index = 3u8)]
    Parameters(pallet_parameters::Event<Runtime>),
}
#[automatically_derived]
#[allow(non_camel_case_types)]
impl ::core::clone::Clone for RuntimeEvent {
    #[inline]
    fn clone(&self) -> RuntimeEvent {
        match self {
            RuntimeEvent::System(__self_0) => {
                RuntimeEvent::System(::core::clone::Clone::clone(__self_0))
            }
            RuntimeEvent::Balances(__self_0) => {
                RuntimeEvent::Balances(::core::clone::Clone::clone(__self_0))
            }
            RuntimeEvent::Parameters(__self_0) => {
                RuntimeEvent::Parameters(::core::clone::Clone::clone(__self_0))
            }
        }
    }
}
#[automatically_derived]
#[allow(non_camel_case_types)]
impl ::core::marker::StructuralPartialEq for RuntimeEvent {}
#[automatically_derived]
#[allow(non_camel_case_types)]
impl ::core::cmp::PartialEq for RuntimeEvent {
    #[inline]
    fn eq(&self, other: &RuntimeEvent) -> bool {
        let __self_discr = ::core::intrinsics::discriminant_value(self);
        let __arg1_discr = ::core::intrinsics::discriminant_value(other);
        __self_discr == __arg1_discr
            && match (self, other) {
                (RuntimeEvent::System(__self_0), RuntimeEvent::System(__arg1_0)) => {
                    __self_0 == __arg1_0
                }
                (RuntimeEvent::Balances(__self_0), RuntimeEvent::Balances(__arg1_0)) => {
                    __self_0 == __arg1_0
                }
                (
                    RuntimeEvent::Parameters(__self_0),
                    RuntimeEvent::Parameters(__arg1_0),
                ) => __self_0 == __arg1_0,
                _ => unsafe { ::core::intrinsics::unreachable() }
            }
    }
}
#[automatically_derived]
#[allow(non_camel_case_types)]
impl ::core::cmp::Eq for RuntimeEvent {
    #[inline]
    #[doc(hidden)]
    #[coverage(off)]
    fn assert_receiver_is_total_eq(&self) -> () {
        let _: ::core::cmp::AssertParamIsEq<frame_system::Event<Runtime>>;
        let _: ::core::cmp::AssertParamIsEq<pallet_balances::Event<Runtime>>;
        let _: ::core::cmp::AssertParamIsEq<pallet_parameters::Event<Runtime>>;
    }
}
#[allow(deprecated)]
const _: () = {
    #[allow(non_camel_case_types)]
    #[automatically_derived]
    impl ::codec::Encode for RuntimeEvent {
        fn size_hint(&self) -> usize {
            1_usize
                + match *self {
                    RuntimeEvent::System(ref aa) => {
                        0_usize.saturating_add(::codec::Encode::size_hint(aa))
                    }
                    RuntimeEvent::Balances(ref aa) => {
                        0_usize.saturating_add(::codec::Encode::size_hint(aa))
                    }
                    RuntimeEvent::Parameters(ref aa) => {
                        0_usize.saturating_add(::codec::Encode::size_hint(aa))
                    }
                    _ => 0_usize,
                }
        }
        fn encode_to<__CodecOutputEdqy: ::codec::Output + ?::core::marker::Sized>(
            &self,
            __codec_dest_edqy: &mut __CodecOutputEdqy,
        ) {
            #[automatically_derived]
            const _: () = {
                #[allow(clippy::unnecessary_cast)]
                #[allow(clippy::cast_possible_truncation)]
                const indices: [(usize, &'static str); 3usize] = [
                    ((0usize) as ::core::primitive::usize, "System"),
                    ((2usize) as ::core::primitive::usize, "Balances"),
                    ((3usize) as ::core::primitive::usize, "Parameters"),
                ];
                const fn search_for_invalid_index(
                    array: &[(usize, &'static str); 3usize],
                ) -> (bool, usize) {
                    let mut i = 0;
                    while i < 3usize {
                        if array[i].0 > 255 {
                            return (true, i);
                        }
                        i += 1;
                    }
                    (false, 0)
                }
                const INVALID_INDEX: (bool, usize) = search_for_invalid_index(&indices);
                if INVALID_INDEX.0 {
                    let msg = ::const_format::pmr::__AssertStr {
                        x: {
                            use ::const_format::__cf_osRcTFl4A;
                            ({
                                #[doc(hidden)]
                                #[allow(unused_mut, non_snake_case)]
                                const CONCATP_NHPMWYD3NJA: &[__cf_osRcTFl4A::pmr::PArgument] = {
                                    let fmt = __cf_osRcTFl4A::pmr::FormattingFlags::NEW;
                                    &[
                                        __cf_osRcTFl4A::pmr::PConvWrapper("Found variant `")
                                            .to_pargument_display(fmt),
                                        __cf_osRcTFl4A::pmr::PConvWrapper(
                                                indices[INVALID_INDEX.1].1,
                                            )
                                            .to_pargument_display(fmt),
                                        __cf_osRcTFl4A::pmr::PConvWrapper("` with invalid index: `")
                                            .to_pargument_display(fmt),
                                        __cf_osRcTFl4A::pmr::PConvWrapper(
                                                indices[INVALID_INDEX.1].0,
                                            )
                                            .to_pargument_display(fmt),
                                        __cf_osRcTFl4A::pmr::PConvWrapper(
                                                "`. Max supported index is 255.",
                                            )
                                            .to_pargument_display(fmt),
                                    ]
                                };
                                {
                                    #[doc(hidden)]
                                    const ARR_LEN: usize = ::const_format::pmr::PArgument::calc_len(
                                        CONCATP_NHPMWYD3NJA,
                                    );
                                    #[doc(hidden)]
                                    const CONCAT_ARR: &::const_format::pmr::LenAndArray<
                                        [u8; ARR_LEN],
                                    > = &::const_format::pmr::__priv_concatenate(
                                        CONCATP_NHPMWYD3NJA,
                                    );
                                    #[doc(hidden)]
                                    #[allow(clippy::transmute_ptr_to_ptr)]
                                    const CONCAT_STR: &str = unsafe {
                                        let slice = ::const_format::pmr::transmute::<
                                            &[u8; ARR_LEN],
                                            &[u8; CONCAT_ARR.len],
                                        >(&CONCAT_ARR.array);
                                        {
                                            let bytes: &'static [::const_format::pmr::u8] = slice;
                                            let string: &'static ::const_format::pmr::str = {
                                                ::const_format::__hidden_utils::PtrToRef {
                                                    ptr: bytes as *const [::const_format::pmr::u8] as *const str,
                                                }
                                                    .reff
                                            };
                                            string
                                        }
                                    };
                                    CONCAT_STR
                                }
                            })
                        },
                    }
                        .x;
                    {
                        ::core::panicking::panic_display(&msg);
                    };
                }
                const fn duplicate_info(
                    array: &[(usize, &'static str); 3usize],
                ) -> (bool, usize, usize) {
                    let len = 3usize;
                    let mut i = 0usize;
                    while i < len {
                        let mut j = i + 1;
                        while j < len {
                            if array[i].0 == array[j].0 {
                                return (true, i, j);
                            }
                            j += 1;
                        }
                        i += 1;
                    }
                    (false, 0, 0)
                }
                const DUP_INFO: (bool, usize, usize) = duplicate_info(&indices);
                if DUP_INFO.0 {
                    let msg = ::const_format::pmr::__AssertStr {
                        x: {
                            use ::const_format::__cf_osRcTFl4A;
                            ({
                                #[doc(hidden)]
                                #[allow(unused_mut, non_snake_case)]
                                const CONCATP_NHPMWYD3NJA: &[__cf_osRcTFl4A::pmr::PArgument] = {
                                    let fmt = __cf_osRcTFl4A::pmr::FormattingFlags::NEW;
                                    &[
                                        __cf_osRcTFl4A::pmr::PConvWrapper(
                                                "Found variants that have duplicate indexes. Both `",
                                            )
                                            .to_pargument_display(fmt),
                                        __cf_osRcTFl4A::pmr::PConvWrapper(indices[DUP_INFO.1].1)
                                            .to_pargument_display(fmt),
                                        __cf_osRcTFl4A::pmr::PConvWrapper("` and `")
                                            .to_pargument_display(fmt),
                                        __cf_osRcTFl4A::pmr::PConvWrapper(indices[DUP_INFO.2].1)
                                            .to_pargument_display(fmt),
                                        __cf_osRcTFl4A::pmr::PConvWrapper("` have the index `")
                                            .to_pargument_display(fmt),
                                        __cf_osRcTFl4A::pmr::PConvWrapper(indices[DUP_INFO.1].0)
                                            .to_pargument_display(fmt),
                                        __cf_osRcTFl4A::pmr::PConvWrapper(
                                                "`. Use different indexes for each variant.",
                                            )
                                            .to_pargument_display(fmt),
                                    ]
                                };
                                {
                                    #[doc(hidden)]
                                    const ARR_LEN: usize = ::const_format::pmr::PArgument::calc_len(
                                        CONCATP_NHPMWYD3NJA,
                                    );
                                    #[doc(hidden)]
                                    const CONCAT_ARR: &::const_format::pmr::LenAndArray<
                                        [u8; ARR_LEN],
                                    > = &::const_format::pmr::__priv_concatenate(
                                        CONCATP_NHPMWYD3NJA,
                                    );
                                    #[doc(hidden)]
                                    #[allow(clippy::transmute_ptr_to_ptr)]
                                    const CONCAT_STR: &str = unsafe {
                                        let slice = ::const_format::pmr::transmute::<
                                            &[u8; ARR_LEN],
                                            &[u8; CONCAT_ARR.len],
                                        >(&CONCAT_ARR.array);
                                        {
                                            let bytes: &'static [::const_format::pmr::u8] = slice;
                                            let string: &'static ::const_format::pmr::str = {
                                                ::const_format::__hidden_utils::PtrToRef {
                                                    ptr: bytes as *const [::const_format::pmr::u8] as *const str,
                                                }
                                                    .reff
                                            };
                                            string
                                        }
                                    };
                                    CONCAT_STR
                                }
                            })
                        },
                    }
                        .x;
                    {
                        ::core::panicking::panic_display(&msg);
                    };
                }
            };
            match *self {
                RuntimeEvent::System(ref aa) => {
                    #[allow(clippy::unnecessary_cast)]
                    __codec_dest_edqy.push_byte((0usize) as ::core::primitive::u8);
                    ::codec::Encode::encode_to(aa, __codec_dest_edqy);
                }
                RuntimeEvent::Balances(ref aa) => {
                    #[allow(clippy::unnecessary_cast)]
                    __codec_dest_edqy.push_byte((2usize) as ::core::primitive::u8);
                    ::codec::Encode::encode_to(aa, __codec_dest_edqy);
                }
                RuntimeEvent::Parameters(ref aa) => {
                    #[allow(clippy::unnecessary_cast)]
                    __codec_dest_edqy.push_byte((3usize) as ::core::primitive::u8);
                    ::codec::Encode::encode_to(aa, __codec_dest_edqy);
                }
                _ => {}
            }
        }
    }
    #[automatically_derived]
    impl ::codec::EncodeLike for RuntimeEvent {}
};
#[allow(deprecated)]
const _: () = {
    #[allow(non_camel_case_types)]
    #[automatically_derived]
    impl ::codec::Decode for RuntimeEvent {
        fn decode<__CodecInputEdqy: ::codec::Input>(
            __codec_input_edqy: &mut __CodecInputEdqy,
        ) -> ::core::result::Result<Self, ::codec::Error> {
            #[automatically_derived]
            const _: () = {
                #[allow(clippy::unnecessary_cast)]
                #[allow(clippy::cast_possible_truncation)]
                const indices: [(usize, &'static str); 3usize] = [
                    ((0usize) as ::core::primitive::usize, "System"),
                    ((2usize) as ::core::primitive::usize, "Balances"),
                    ((3usize) as ::core::primitive::usize, "Parameters"),
                ];
                const fn search_for_invalid_index(
                    array: &[(usize, &'static str); 3usize],
                ) -> (bool, usize) {
                    let mut i = 0;
                    while i < 3usize {
                        if array[i].0 > 255 {
                            return (true, i);
                        }
                        i += 1;
                    }
                    (false, 0)
                }
                const INVALID_INDEX: (bool, usize) = search_for_invalid_index(&indices);
                if INVALID_INDEX.0 {
                    let msg = ::const_format::pmr::__AssertStr {
                        x: {
                            use ::const_format::__cf_osRcTFl4A;
                            ({
                                #[doc(hidden)]
                                #[allow(unused_mut, non_snake_case)]
                                const CONCATP_NHPMWYD3NJA: &[__cf_osRcTFl4A::pmr::PArgument] = {
                                    let fmt = __cf_osRcTFl4A::pmr::FormattingFlags::NEW;
                                    &[
                                        __cf_osRcTFl4A::pmr::PConvWrapper("Found variant `")
                                            .to_pargument_display(fmt),
                                        __cf_osRcTFl4A::pmr::PConvWrapper(
                                                indices[INVALID_INDEX.1].1,
                                            )
                                            .to_pargument_display(fmt),
                                        __cf_osRcTFl4A::pmr::PConvWrapper("` with invalid index: `")
                                            .to_pargument_display(fmt),
                                        __cf_osRcTFl4A::pmr::PConvWrapper(
                                                indices[INVALID_INDEX.1].0,
                                            )
                                            .to_pargument_display(fmt),
                                        __cf_osRcTFl4A::pmr::PConvWrapper(
                                                "`. Max supported index is 255.",
                                            )
                                            .to_pargument_display(fmt),
                                    ]
                                };
                                {
                                    #[doc(hidden)]
                                    const ARR_LEN: usize = ::const_format::pmr::PArgument::calc_len(
                                        CONCATP_NHPMWYD3NJA,
                                    );
                                    #[doc(hidden)]
                                    const CONCAT_ARR: &::const_format::pmr::LenAndArray<
                                        [u8; ARR_LEN],
                                    > = &::const_format::pmr::__priv_concatenate(
                                        CONCATP_NHPMWYD3NJA,
                                    );
                                    #[doc(hidden)]
                                    #[allow(clippy::transmute_ptr_to_ptr)]
                                    const CONCAT_STR: &str = unsafe {
                                        let slice = ::const_format::pmr::transmute::<
                                            &[u8; ARR_LEN],
                                            &[u8; CONCAT_ARR.len],
                                        >(&CONCAT_ARR.array);
                                        {
                                            let bytes: &'static [::const_format::pmr::u8] = slice;
                                            let string: &'static ::const_format::pmr::str = {
                                                ::const_format::__hidden_utils::PtrToRef {
                                                    ptr: bytes as *const [::const_format::pmr::u8] as *const str,
                                                }
                                                    .reff
                                            };
                                            string
                                        }
                                    };
                                    CONCAT_STR
                                }
                            })
                        },
                    }
                        .x;
                    {
                        ::core::panicking::panic_display(&msg);
                    };
                }
                const fn duplicate_info(
                    array: &[(usize, &'static str); 3usize],
                ) -> (bool, usize, usize) {
                    let len = 3usize;
                    let mut i = 0usize;
                    while i < len {
                        let mut j = i + 1;
                        while j < len {
                            if array[i].0 == array[j].0 {
                                return (true, i, j);
                            }
                            j += 1;
                        }
                        i += 1;
                    }
                    (false, 0, 0)
                }
                const DUP_INFO: (bool, usize, usize) = duplicate_info(&indices);
                if DUP_INFO.0 {
                    let msg = ::const_format::pmr::__AssertStr {
                        x: {
                            use ::const_format::__cf_osRcTFl4A;
                            ({
                                #[doc(hidden)]
                                #[allow(unused_mut, non_snake_case)]
                                const CONCATP_NHPMWYD3NJA: &[__cf_osRcTFl4A::pmr::PArgument] = {
                                    let fmt = __cf_osRcTFl4A::pmr::FormattingFlags::NEW;
                                    &[
                                        __cf_osRcTFl4A::pmr::PConvWrapper(
                                                "Found variants that have duplicate indexes. Both `",
                                            )
                                            .to_pargument_display(fmt),
                                        __cf_osRcTFl4A::pmr::PConvWrapper(indices[DUP_INFO.1].1)
                                            .to_pargument_display(fmt),
                                        __cf_osRcTFl4A::pmr::PConvWrapper("` and `")
                                            .to_pargument_display(fmt),
                                        __cf_osRcTFl4A::pmr::PConvWrapper(indices[DUP_INFO.2].1)
                                            .to_pargument_display(fmt),
                                        __cf_osRcTFl4A::pmr::PConvWrapper("` have the index `")
                                            .to_pargument_display(fmt),
                                        __cf_osRcTFl4A::pmr::PConvWrapper(indices[DUP_INFO.1].0)
                                            .to_pargument_display(fmt),
                                        __cf_osRcTFl4A::pmr::PConvWrapper(
                                                "`. Use different indexes for each variant.",
                                            )
                                            .to_pargument_display(fmt),
                                    ]
                                };
                                {
                                    #[doc(hidden)]
                                    const ARR_LEN: usize = ::const_format::pmr::PArgument::calc_len(
                                        CONCATP_NHPMWYD3NJA,
                                    );
                                    #[doc(hidden)]
                                    const CONCAT_ARR: &::const_format::pmr::LenAndArray<
                                        [u8; ARR_LEN],
                                    > = &::const_format::pmr::__priv_concatenate(
                                        CONCATP_NHPMWYD3NJA,
                                    );
                                    #[doc(hidden)]
                                    #[allow(clippy::transmute_ptr_to_ptr)]
                                    const CONCAT_STR: &str = unsafe {
                                        let slice = ::const_format::pmr::transmute::<
                                            &[u8; ARR_LEN],
                                            &[u8; CONCAT_ARR.len],
                                        >(&CONCAT_ARR.array);
                                        {
                                            let bytes: &'static [::const_format::pmr::u8] = slice;
                                            let string: &'static ::const_format::pmr::str = {
                                                ::const_format::__hidden_utils::PtrToRef {
                                                    ptr: bytes as *const [::const_format::pmr::u8] as *const str,
                                                }
                                                    .reff
                                            };
                                            string
                                        }
                                    };
                                    CONCAT_STR
                                }
                            })
                        },
                    }
                        .x;
                    {
                        ::core::panicking::panic_display(&msg);
                    };
                }
            };
            match __codec_input_edqy
                .read_byte()
                .map_err(|e| {
                    e
                        .chain(
                            "Could not decode `RuntimeEvent`, failed to read variant byte",
                        )
                })?
            {
                #[allow(clippy::unnecessary_cast)]
                #[allow(clippy::cast_possible_truncation)]
                __codec_x_edqy if __codec_x_edqy
                    == (0usize) as ::core::primitive::u8 => {
                    #[allow(clippy::redundant_closure_call)]
                    return (move || {
                        ::core::result::Result::Ok(
                            RuntimeEvent::System({
                                let __codec_res_edqy = <frame_system::Event<
                                    Runtime,
                                > as ::codec::Decode>::decode(__codec_input_edqy);
                                match __codec_res_edqy {
                                    ::core::result::Result::Err(e) => {
                                        return ::core::result::Result::Err(
                                            e.chain("Could not decode `RuntimeEvent::System.0`"),
                                        );
                                    }
                                    ::core::result::Result::Ok(__codec_res_edqy) => {
                                        __codec_res_edqy
                                    }
                                }
                            }),
                        )
                    })();
                }
                #[allow(clippy::unnecessary_cast)]
                #[allow(clippy::cast_possible_truncation)]
                __codec_x_edqy if __codec_x_edqy
                    == (2usize) as ::core::primitive::u8 => {
                    #[allow(clippy::redundant_closure_call)]
                    return (move || {
                        ::core::result::Result::Ok(
                            RuntimeEvent::Balances({
                                let __codec_res_edqy = <pallet_balances::Event<
                                    Runtime,
                                > as ::codec::Decode>::decode(__codec_input_edqy);
                                match __codec_res_edqy {
                                    ::core::result::Result::Err(e) => {
                                        return ::core::result::Result::Err(
                                            e.chain("Could not decode `RuntimeEvent::Balances.0`"),
                                        );
                                    }
                                    ::core::result::Result::Ok(__codec_res_edqy) => {
                                        __codec_res_edqy
                                    }
                                }
                            }),
                        )
                    })();
                }
                #[allow(clippy::unnecessary_cast)]
                #[allow(clippy::cast_possible_truncation)]
                __codec_x_edqy if __codec_x_edqy
                    == (3usize) as ::core::primitive::u8 => {
                    #[allow(clippy::redundant_closure_call)]
                    return (move || {
                        ::core::result::Result::Ok(
                            RuntimeEvent::Parameters({
                                let __codec_res_edqy = <pallet_parameters::Event<
                                    Runtime,
                                > as ::codec::Decode>::decode(__codec_input_edqy);
                                match __codec_res_edqy {
                                    ::core::result::Result::Err(e) => {
                                        return ::core::result::Result::Err(
                                            e.chain("Could not decode `RuntimeEvent::Parameters.0`"),
                                        );
                                    }
                                    ::core::result::Result::Ok(__codec_res_edqy) => {
                                        __codec_res_edqy
                                    }
                                }
                            }),
                        )
                    })();
                }
                _ => {
                    #[allow(clippy::redundant_closure_call)]
                    return (move || {
                        ::core::result::Result::Err(
                            <_ as ::core::convert::Into<
                                _,
                            >>::into(
                                "Could not decode `RuntimeEvent`, variant doesn't exist",
                            ),
                        )
                    })();
                }
            }
        }
    }
};
#[allow(deprecated)]
const _: () = {
    #[allow(non_camel_case_types)]
    fn check_struct() {
        fn check_field<T: ::codec::DecodeWithMemTracking>() {}
        check_field::<frame_system::Event<Runtime>>();
        check_field::<pallet_balances::Event<Runtime>>();
        check_field::<pallet_parameters::Event<Runtime>>();
    }
    #[automatically_derived]
    impl ::codec::DecodeWithMemTracking for RuntimeEvent {}
};
#[allow(non_upper_case_globals, deprecated, unused_attributes, unused_qualifications)]
const _: () = {
    impl ::scale_info::TypeInfo for RuntimeEvent {
        type Identity = Self;
        fn type_info() -> ::scale_info::Type {
            ::scale_info::Type::builder()
                .path(
                    ::scale_info::Path::new_with_replace(
                        "RuntimeEvent",
                        "polkadot_test",
                        &[],
                    ),
                )
                .type_params(::alloc::vec::Vec::new())
                .variant(
                    ::scale_info::build::Variants::new()
                        .variant(
                            "System",
                            |v| {
                                v
                                    .index(0u8 as ::core::primitive::u8)
                                    .fields(
                                        ::scale_info::build::Fields::unnamed()
                                            .field(|f| {
                                                f
                                                    .ty::<frame_system::Event<Runtime>>()
                                                    .type_name("frame_system::Event<Runtime>")
                                            }),
                                    )
                            },
                        )
                        .variant(
                            "Balances",
                            |v| {
                                v
                                    .index(2u8 as ::core::primitive::u8)
                                    .fields(
                                        ::scale_info::build::Fields::unnamed()
                                            .field(|f| {
                                                f
                                                    .ty::<pallet_balances::Event<Runtime>>()
                                                    .type_name("pallet_balances::Event<Runtime>")
                                            }),
                                    )
                            },
                        )
                        .variant(
                            "Parameters",
                            |v| {
                                v
                                    .index(3u8 as ::core::primitive::u8)
                                    .fields(
                                        ::scale_info::build::Fields::unnamed()
                                            .field(|f| {
                                                f
                                                    .ty::<pallet_parameters::Event<Runtime>>()
                                                    .type_name("pallet_parameters::Event<Runtime>")
                                            }),
                                    )
                            },
                        ),
                )
        }
    }
};
#[automatically_derived]
#[allow(non_camel_case_types)]
impl ::core::fmt::Debug for RuntimeEvent {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match self {
            RuntimeEvent::System(__self_0) => {
                ::core::fmt::Formatter::debug_tuple_field1_finish(f, "System", &__self_0)
            }
            RuntimeEvent::Balances(__self_0) => {
                ::core::fmt::Formatter::debug_tuple_field1_finish(
                    f,
                    "Balances",
                    &__self_0,
                )
            }
            RuntimeEvent::Parameters(__self_0) => {
                ::core::fmt::Formatter::debug_tuple_field1_finish(
                    f,
                    "Parameters",
                    &__self_0,
                )
            }
        }
    }
}
#[allow(deprecated)]
impl From<frame_system::Event<Runtime>> for RuntimeEvent {
    fn from(x: frame_system::Event<Runtime>) -> Self {
        RuntimeEvent::System(x)
    }
}
#[allow(deprecated)]
impl TryInto<frame_system::Event<Runtime>> for RuntimeEvent {
    type Error = ();
    fn try_into(
        self,
    ) -> ::core::result::Result<frame_system::Event<Runtime>, Self::Error> {
        match self {
            Self::System(evt) => Ok(evt),
            _ => Err(()),
        }
    }
}
#[allow(deprecated)]
impl From<pallet_balances::Event<Runtime>> for RuntimeEvent {
    fn from(x: pallet_balances::Event<Runtime>) -> Self {
        RuntimeEvent::Balances(x)
    }
}
#[allow(deprecated)]
impl TryInto<pallet_balances::Event<Runtime>> for RuntimeEvent {
    type Error = ();
    fn try_into(
        self,
    ) -> ::core::result::Result<pallet_balances::Event<Runtime>, Self::Error> {
        match self {
            Self::Balances(evt) => Ok(evt),
            _ => Err(()),
        }
    }
}
#[allow(deprecated)]
impl From<pallet_parameters::Event<Runtime>> for RuntimeEvent {
    fn from(x: pallet_parameters::Event<Runtime>) -> Self {
        RuntimeEvent::Parameters(x)
    }
}
#[allow(deprecated)]
impl TryInto<pallet_parameters::Event<Runtime>> for RuntimeEvent {
    type Error = ();
    fn try_into(
        self,
    ) -> ::core::result::Result<pallet_parameters::Event<Runtime>, Self::Error> {
        match self {
            Self::Parameters(evt) => Ok(evt),
            _ => Err(()),
        }
    }
}
#[allow(non_camel_case_types)]
pub enum RuntimeError {
    #[codec(index = 0u8)]
    System(frame_system::Error<Runtime>),
    #[codec(index = 2u8)]
    Balances(pallet_balances::Error<Runtime>),
}
#[allow(deprecated)]
const _: () = {
    #[allow(non_camel_case_types)]
    #[automatically_derived]
    impl ::codec::Encode for RuntimeError {
        fn size_hint(&self) -> usize {
            1_usize
                + match *self {
                    RuntimeError::System(ref aa) => {
                        0_usize.saturating_add(::codec::Encode::size_hint(aa))
                    }
                    RuntimeError::Balances(ref aa) => {
                        0_usize.saturating_add(::codec::Encode::size_hint(aa))
                    }
                    _ => 0_usize,
                }
        }
        fn encode_to<__CodecOutputEdqy: ::codec::Output + ?::core::marker::Sized>(
            &self,
            __codec_dest_edqy: &mut __CodecOutputEdqy,
        ) {
            #[automatically_derived]
            const _: () = {
                #[allow(clippy::unnecessary_cast)]
                #[allow(clippy::cast_possible_truncation)]
                const indices: [(usize, &'static str); 2usize] = [
                    ((0usize) as ::core::primitive::usize, "System"),
                    ((2usize) as ::core::primitive::usize, "Balances"),
                ];
                const fn search_for_invalid_index(
                    array: &[(usize, &'static str); 2usize],
                ) -> (bool, usize) {
                    let mut i = 0;
                    while i < 2usize {
                        if array[i].0 > 255 {
                            return (true, i);
                        }
                        i += 1;
                    }
                    (false, 0)
                }
                const INVALID_INDEX: (bool, usize) = search_for_invalid_index(&indices);
                if INVALID_INDEX.0 {
                    let msg = ::const_format::pmr::__AssertStr {
                        x: {
                            use ::const_format::__cf_osRcTFl4A;
                            ({
                                #[doc(hidden)]
                                #[allow(unused_mut, non_snake_case)]
                                const CONCATP_NHPMWYD3NJA: &[__cf_osRcTFl4A::pmr::PArgument] = {
                                    let fmt = __cf_osRcTFl4A::pmr::FormattingFlags::NEW;
                                    &[
                                        __cf_osRcTFl4A::pmr::PConvWrapper("Found variant `")
                                            .to_pargument_display(fmt),
                                        __cf_osRcTFl4A::pmr::PConvWrapper(
                                                indices[INVALID_INDEX.1].1,
                                            )
                                            .to_pargument_display(fmt),
                                        __cf_osRcTFl4A::pmr::PConvWrapper("` with invalid index: `")
                                            .to_pargument_display(fmt),
                                        __cf_osRcTFl4A::pmr::PConvWrapper(
                                                indices[INVALID_INDEX.1].0,
                                            )
                                            .to_pargument_display(fmt),
                                        __cf_osRcTFl4A::pmr::PConvWrapper(
                                                "`. Max supported index is 255.",
                                            )
                                            .to_pargument_display(fmt),
                                    ]
                                };
                                {
                                    #[doc(hidden)]
                                    const ARR_LEN: usize = ::const_format::pmr::PArgument::calc_len(
                                        CONCATP_NHPMWYD3NJA,
                                    );
                                    #[doc(hidden)]
                                    const CONCAT_ARR: &::const_format::pmr::LenAndArray<
                                        [u8; ARR_LEN],
                                    > = &::const_format::pmr::__priv_concatenate(
                                        CONCATP_NHPMWYD3NJA,
                                    );
                                    #[doc(hidden)]
                                    #[allow(clippy::transmute_ptr_to_ptr)]
                                    const CONCAT_STR: &str = unsafe {
                                        let slice = ::const_format::pmr::transmute::<
                                            &[u8; ARR_LEN],
                                            &[u8; CONCAT_ARR.len],
                                        >(&CONCAT_ARR.array);
                                        {
                                            let bytes: &'static [::const_format::pmr::u8] = slice;
                                            let string: &'static ::const_format::pmr::str = {
                                                ::const_format::__hidden_utils::PtrToRef {
                                                    ptr: bytes as *const [::const_format::pmr::u8] as *const str,
                                                }
                                                    .reff
                                            };
                                            string
                                        }
                                    };
                                    CONCAT_STR
                                }
                            })
                        },
                    }
                        .x;
                    {
                        ::core::panicking::panic_display(&msg);
                    };
                }
                const fn duplicate_info(
                    array: &[(usize, &'static str); 2usize],
                ) -> (bool, usize, usize) {
                    let len = 2usize;
                    let mut i = 0usize;
                    while i < len {
                        let mut j = i + 1;
                        while j < len {
                            if array[i].0 == array[j].0 {
                                return (true, i, j);
                            }
                            j += 1;
                        }
                        i += 1;
                    }
                    (false, 0, 0)
                }
                const DUP_INFO: (bool, usize, usize) = duplicate_info(&indices);
                if DUP_INFO.0 {
                    let msg = ::const_format::pmr::__AssertStr {
                        x: {
                            use ::const_format::__cf_osRcTFl4A;
                            ({
                                #[doc(hidden)]
                                #[allow(unused_mut, non_snake_case)]
                                const CONCATP_NHPMWYD3NJA: &[__cf_osRcTFl4A::pmr::PArgument] = {
                                    let fmt = __cf_osRcTFl4A::pmr::FormattingFlags::NEW;
                                    &[
                                        __cf_osRcTFl4A::pmr::PConvWrapper(
                                                "Found variants that have duplicate indexes. Both `",
                                            )
                                            .to_pargument_display(fmt),
                                        __cf_osRcTFl4A::pmr::PConvWrapper(indices[DUP_INFO.1].1)
                                            .to_pargument_display(fmt),
                                        __cf_osRcTFl4A::pmr::PConvWrapper("` and `")
                                            .to_pargument_display(fmt),
                                        __cf_osRcTFl4A::pmr::PConvWrapper(indices[DUP_INFO.2].1)
                                            .to_pargument_display(fmt),
                                        __cf_osRcTFl4A::pmr::PConvWrapper("` have the index `")
                                            .to_pargument_display(fmt),
                                        __cf_osRcTFl4A::pmr::PConvWrapper(indices[DUP_INFO.1].0)
                                            .to_pargument_display(fmt),
                                        __cf_osRcTFl4A::pmr::PConvWrapper(
                                                "`. Use different indexes for each variant.",
                                            )
                                            .to_pargument_display(fmt),
                                    ]
                                };
                                {
                                    #[doc(hidden)]
                                    const ARR_LEN: usize = ::const_format::pmr::PArgument::calc_len(
                                        CONCATP_NHPMWYD3NJA,
                                    );
                                    #[doc(hidden)]
                                    const CONCAT_ARR: &::const_format::pmr::LenAndArray<
                                        [u8; ARR_LEN],
                                    > = &::const_format::pmr::__priv_concatenate(
                                        CONCATP_NHPMWYD3NJA,
                                    );
                                    #[doc(hidden)]
                                    #[allow(clippy::transmute_ptr_to_ptr)]
                                    const CONCAT_STR: &str = unsafe {
                                        let slice = ::const_format::pmr::transmute::<
                                            &[u8; ARR_LEN],
                                            &[u8; CONCAT_ARR.len],
                                        >(&CONCAT_ARR.array);
                                        {
                                            let bytes: &'static [::const_format::pmr::u8] = slice;
                                            let string: &'static ::const_format::pmr::str = {
                                                ::const_format::__hidden_utils::PtrToRef {
                                                    ptr: bytes as *const [::const_format::pmr::u8] as *const str,
                                                }
                                                    .reff
                                            };
                                            string
                                        }
                                    };
                                    CONCAT_STR
                                }
                            })
                        },
                    }
                        .x;
                    {
                        ::core::panicking::panic_display(&msg);
                    };
                }
            };
            match *self {
                RuntimeError::System(ref aa) => {
                    #[allow(clippy::unnecessary_cast)]
                    __codec_dest_edqy.push_byte((0usize) as ::core::primitive::u8);
                    ::codec::Encode::encode_to(aa, __codec_dest_edqy);
                }
                RuntimeError::Balances(ref aa) => {
                    #[allow(clippy::unnecessary_cast)]
                    __codec_dest_edqy.push_byte((2usize) as ::core::primitive::u8);
                    ::codec::Encode::encode_to(aa, __codec_dest_edqy);
                }
                _ => {}
            }
        }
    }
    #[automatically_derived]
    impl ::codec::EncodeLike for RuntimeError {}
};
#[allow(deprecated)]
const _: () = {
    #[allow(non_camel_case_types)]
    #[automatically_derived]
    impl ::codec::Decode for RuntimeError {
        fn decode<__CodecInputEdqy: ::codec::Input>(
            __codec_input_edqy: &mut __CodecInputEdqy,
        ) -> ::core::result::Result<Self, ::codec::Error> {
            #[automatically_derived]
            const _: () = {
                #[allow(clippy::unnecessary_cast)]
                #[allow(clippy::cast_possible_truncation)]
                const indices: [(usize, &'static str); 2usize] = [
                    ((0usize) as ::core::primitive::usize, "System"),
                    ((2usize) as ::core::primitive::usize, "Balances"),
                ];
                const fn search_for_invalid_index(
                    array: &[(usize, &'static str); 2usize],
                ) -> (bool, usize) {
                    let mut i = 0;
                    while i < 2usize {
                        if array[i].0 > 255 {
                            return (true, i);
                        }
                        i += 1;
                    }
                    (false, 0)
                }
                const INVALID_INDEX: (bool, usize) = search_for_invalid_index(&indices);
                if INVALID_INDEX.0 {
                    let msg = ::const_format::pmr::__AssertStr {
                        x: {
                            use ::const_format::__cf_osRcTFl4A;
                            ({
                                #[doc(hidden)]
                                #[allow(unused_mut, non_snake_case)]
                                const CONCATP_NHPMWYD3NJA: &[__cf_osRcTFl4A::pmr::PArgument] = {
                                    let fmt = __cf_osRcTFl4A::pmr::FormattingFlags::NEW;
                                    &[
                                        __cf_osRcTFl4A::pmr::PConvWrapper("Found variant `")
                                            .to_pargument_display(fmt),
                                        __cf_osRcTFl4A::pmr::PConvWrapper(
                                                indices[INVALID_INDEX.1].1,
                                            )
                                            .to_pargument_display(fmt),
                                        __cf_osRcTFl4A::pmr::PConvWrapper("` with invalid index: `")
                                            .to_pargument_display(fmt),
                                        __cf_osRcTFl4A::pmr::PConvWrapper(
                                                indices[INVALID_INDEX.1].0,
                                            )
                                            .to_pargument_display(fmt),
                                        __cf_osRcTFl4A::pmr::PConvWrapper(
                                                "`. Max supported index is 255.",
                                            )
                                            .to_pargument_display(fmt),
                                    ]
                                };
                                {
                                    #[doc(hidden)]
                                    const ARR_LEN: usize = ::const_format::pmr::PArgument::calc_len(
                                        CONCATP_NHPMWYD3NJA,
                                    );
                                    #[doc(hidden)]
                                    const CONCAT_ARR: &::const_format::pmr::LenAndArray<
                                        [u8; ARR_LEN],
                                    > = &::const_format::pmr::__priv_concatenate(
                                        CONCATP_NHPMWYD3NJA,
                                    );
                                    #[doc(hidden)]
                                    #[allow(clippy::transmute_ptr_to_ptr)]
                                    const CONCAT_STR: &str = unsafe {
                                        let slice = ::const_format::pmr::transmute::<
                                            &[u8; ARR_LEN],
                                            &[u8; CONCAT_ARR.len],
                                        >(&CONCAT_ARR.array);
                                        {
                                            let bytes: &'static [::const_format::pmr::u8] = slice;
                                            let string: &'static ::const_format::pmr::str = {
                                                ::const_format::__hidden_utils::PtrToRef {
                                                    ptr: bytes as *const [::const_format::pmr::u8] as *const str,
                                                }
                                                    .reff
                                            };
                                            string
                                        }
                                    };
                                    CONCAT_STR
                                }
                            })
                        },
                    }
                        .x;
                    {
                        ::core::panicking::panic_display(&msg);
                    };
                }
                const fn duplicate_info(
                    array: &[(usize, &'static str); 2usize],
                ) -> (bool, usize, usize) {
                    let len = 2usize;
                    let mut i = 0usize;
                    while i < len {
                        let mut j = i + 1;
                        while j < len {
                            if array[i].0 == array[j].0 {
                                return (true, i, j);
                            }
                            j += 1;
                        }
                        i += 1;
                    }
                    (false, 0, 0)
                }
                const DUP_INFO: (bool, usize, usize) = duplicate_info(&indices);
                if DUP_INFO.0 {
                    let msg = ::const_format::pmr::__AssertStr {
                        x: {
                            use ::const_format::__cf_osRcTFl4A;
                            ({
                                #[doc(hidden)]
                                #[allow(unused_mut, non_snake_case)]
                                const CONCATP_NHPMWYD3NJA: &[__cf_osRcTFl4A::pmr::PArgument] = {
                                    let fmt = __cf_osRcTFl4A::pmr::FormattingFlags::NEW;
                                    &[
                                        __cf_osRcTFl4A::pmr::PConvWrapper(
                                                "Found variants that have duplicate indexes. Both `",
                                            )
                                            .to_pargument_display(fmt),
                                        __cf_osRcTFl4A::pmr::PConvWrapper(indices[DUP_INFO.1].1)
                                            .to_pargument_display(fmt),
                                        __cf_osRcTFl4A::pmr::PConvWrapper("` and `")
                                            .to_pargument_display(fmt),
                                        __cf_osRcTFl4A::pmr::PConvWrapper(indices[DUP_INFO.2].1)
                                            .to_pargument_display(fmt),
                                        __cf_osRcTFl4A::pmr::PConvWrapper("` have the index `")
                                            .to_pargument_display(fmt),
                                        __cf_osRcTFl4A::pmr::PConvWrapper(indices[DUP_INFO.1].0)
                                            .to_pargument_display(fmt),
                                        __cf_osRcTFl4A::pmr::PConvWrapper(
                                                "`. Use different indexes for each variant.",
                                            )
                                            .to_pargument_display(fmt),
                                    ]
                                };
                                {
                                    #[doc(hidden)]
                                    const ARR_LEN: usize = ::const_format::pmr::PArgument::calc_len(
                                        CONCATP_NHPMWYD3NJA,
                                    );
                                    #[doc(hidden)]
                                    const CONCAT_ARR: &::const_format::pmr::LenAndArray<
                                        [u8; ARR_LEN],
                                    > = &::const_format::pmr::__priv_concatenate(
                                        CONCATP_NHPMWYD3NJA,
                                    );
                                    #[doc(hidden)]
                                    #[allow(clippy::transmute_ptr_to_ptr)]
                                    const CONCAT_STR: &str = unsafe {
                                        let slice = ::const_format::pmr::transmute::<
                                            &[u8; ARR_LEN],
                                            &[u8; CONCAT_ARR.len],
                                        >(&CONCAT_ARR.array);
                                        {
                                            let bytes: &'static [::const_format::pmr::u8] = slice;
                                            let string: &'static ::const_format::pmr::str = {
                                                ::const_format::__hidden_utils::PtrToRef {
                                                    ptr: bytes as *const [::const_format::pmr::u8] as *const str,
                                                }
                                                    .reff
                                            };
                                            string
                                        }
                                    };
                                    CONCAT_STR
                                }
                            })
                        },
                    }
                        .x;
                    {
                        ::core::panicking::panic_display(&msg);
                    };
                }
            };
            match __codec_input_edqy
                .read_byte()
                .map_err(|e| {
                    e
                        .chain(
                            "Could not decode `RuntimeError`, failed to read variant byte",
                        )
                })?
            {
                #[allow(clippy::unnecessary_cast)]
                #[allow(clippy::cast_possible_truncation)]
                __codec_x_edqy if __codec_x_edqy
                    == (0usize) as ::core::primitive::u8 => {
                    #[allow(clippy::redundant_closure_call)]
                    return (move || {
                        ::core::result::Result::Ok(
                            RuntimeError::System({
                                let __codec_res_edqy = <frame_system::Error<
                                    Runtime,
                                > as ::codec::Decode>::decode(__codec_input_edqy);
                                match __codec_res_edqy {
                                    ::core::result::Result::Err(e) => {
                                        return ::core::result::Result::Err(
                                            e.chain("Could not decode `RuntimeError::System.0`"),
                                        );
                                    }
                                    ::core::result::Result::Ok(__codec_res_edqy) => {
                                        __codec_res_edqy
                                    }
                                }
                            }),
                        )
                    })();
                }
                #[allow(clippy::unnecessary_cast)]
                #[allow(clippy::cast_possible_truncation)]
                __codec_x_edqy if __codec_x_edqy
                    == (2usize) as ::core::primitive::u8 => {
                    #[allow(clippy::redundant_closure_call)]
                    return (move || {
                        ::core::result::Result::Ok(
                            RuntimeError::Balances({
                                let __codec_res_edqy = <pallet_balances::Error<
                                    Runtime,
                                > as ::codec::Decode>::decode(__codec_input_edqy);
                                match __codec_res_edqy {
                                    ::core::result::Result::Err(e) => {
                                        return ::core::result::Result::Err(
                                            e.chain("Could not decode `RuntimeError::Balances.0`"),
                                        );
                                    }
                                    ::core::result::Result::Ok(__codec_res_edqy) => {
                                        __codec_res_edqy
                                    }
                                }
                            }),
                        )
                    })();
                }
                _ => {
                    #[allow(clippy::redundant_closure_call)]
                    return (move || {
                        ::core::result::Result::Err(
                            <_ as ::core::convert::Into<
                                _,
                            >>::into(
                                "Could not decode `RuntimeError`, variant doesn't exist",
                            ),
                        )
                    })();
                }
            }
        }
    }
};
#[allow(deprecated)]
const _: () = {
    #[allow(non_camel_case_types)]
    fn check_struct() {
        fn check_field<T: ::codec::DecodeWithMemTracking>() {}
        check_field::<frame_system::Error<Runtime>>();
        check_field::<pallet_balances::Error<Runtime>>();
    }
    #[automatically_derived]
    impl ::codec::DecodeWithMemTracking for RuntimeError {}
};
#[allow(non_upper_case_globals, deprecated, unused_attributes, unused_qualifications)]
const _: () = {
    impl ::scale_info::TypeInfo for RuntimeError {
        type Identity = Self;
        fn type_info() -> ::scale_info::Type {
            ::scale_info::Type::builder()
                .path(
                    ::scale_info::Path::new_with_replace(
                        "RuntimeError",
                        "polkadot_test",
                        &[],
                    ),
                )
                .type_params(::alloc::vec::Vec::new())
                .variant(
                    ::scale_info::build::Variants::new()
                        .variant(
                            "System",
                            |v| {
                                v
                                    .index(0u8 as ::core::primitive::u8)
                                    .fields(
                                        ::scale_info::build::Fields::unnamed()
                                            .field(|f| {
                                                f
                                                    .ty::<frame_system::Error<Runtime>>()
                                                    .type_name("frame_system::Error<Runtime>")
                                            }),
                                    )
                            },
                        )
                        .variant(
                            "Balances",
                            |v| {
                                v
                                    .index(2u8 as ::core::primitive::u8)
                                    .fields(
                                        ::scale_info::build::Fields::unnamed()
                                            .field(|f| {
                                                f
                                                    .ty::<pallet_balances::Error<Runtime>>()
                                                    .type_name("pallet_balances::Error<Runtime>")
                                            }),
                                    )
                            },
                        ),
                )
        }
    }
};
#[automatically_derived]
#[allow(non_camel_case_types)]
impl ::core::fmt::Debug for RuntimeError {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match self {
            RuntimeError::System(__self_0) => {
                ::core::fmt::Formatter::debug_tuple_field1_finish(f, "System", &__self_0)
            }
            RuntimeError::Balances(__self_0) => {
                ::core::fmt::Formatter::debug_tuple_field1_finish(
                    f,
                    "Balances",
                    &__self_0,
                )
            }
        }
    }
}
#[allow(deprecated)]
impl From<frame_system::Error<Runtime>> for RuntimeError {
    fn from(x: frame_system::Error<Runtime>) -> Self {
        RuntimeError::System(x)
    }
}
#[allow(deprecated)]
impl TryInto<frame_system::Error<Runtime>> for RuntimeError {
    type Error = ();
    fn try_into(
        self,
    ) -> ::core::result::Result<frame_system::Error<Runtime>, Self::Error> {
        match self {
            Self::System(evt) => Ok(evt),
            _ => Err(()),
        }
    }
}
#[allow(deprecated)]
impl From<pallet_balances::Error<Runtime>> for RuntimeError {
    fn from(x: pallet_balances::Error<Runtime>) -> Self {
        RuntimeError::Balances(x)
    }
}
#[allow(deprecated)]
impl TryInto<pallet_balances::Error<Runtime>> for RuntimeError {
    type Error = ();
    fn try_into(
        self,
    ) -> ::core::result::Result<pallet_balances::Error<Runtime>, Self::Error> {
        match self {
            Self::Balances(evt) => Ok(evt),
            _ => Err(()),
        }
    }
}
impl RuntimeError {
    /// Optionally convert the `DispatchError` into the `RuntimeError`.
    ///
    /// Returns `Some` if the error matches the `DispatchError::Module` variant, otherwise `None`.
    pub fn from_dispatch_error(
        err: self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_runtime::DispatchError,
    ) -> Option<Self> {
        let self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_runtime::DispatchError::Module(
            module_error,
        ) = err else { return None };
        let bytes = self::sp_api_hidden_includes_construct_runtime::hidden_include::__private::codec::Encode::encode(
            &module_error,
        );
        self::sp_api_hidden_includes_construct_runtime::hidden_include::__private::codec::Decode::decode(
                &mut &bytes[..],
            )
            .ok()
    }
}
/// The runtime origin type representing the origin of a call.
///
/// Origin is always created with the base filter configured in [`frame_system::Config::BaseCallFilter`].
pub struct RuntimeOrigin {
    pub caller: OriginCaller,
    filter: self::sp_api_hidden_includes_construct_runtime::hidden_include::__private::Rc<
        self::sp_api_hidden_includes_construct_runtime::hidden_include::__private::Box<
            dyn Fn(&<Runtime as frame_system::Config>::RuntimeCall) -> bool,
        >,
    >,
}
#[automatically_derived]
impl ::core::clone::Clone for RuntimeOrigin {
    #[inline]
    fn clone(&self) -> RuntimeOrigin {
        RuntimeOrigin {
            caller: ::core::clone::Clone::clone(&self.caller),
            filter: ::core::clone::Clone::clone(&self.filter),
        }
    }
}
impl core::fmt::Debug for RuntimeOrigin {
    fn fmt(
        &self,
        fmt: &mut core::fmt::Formatter,
    ) -> core::result::Result<(), core::fmt::Error> {
        fmt.debug_struct("Origin")
            .field("caller", &self.caller)
            .field("filter", &"[function ptr]")
            .finish()
    }
}
impl self::sp_api_hidden_includes_construct_runtime::hidden_include::traits::OriginTrait
for RuntimeOrigin {
    type Call = <Runtime as frame_system::Config>::RuntimeCall;
    type PalletsOrigin = OriginCaller;
    type AccountId = <Runtime as frame_system::Config>::AccountId;
    fn add_filter(&mut self, filter: impl Fn(&Self::Call) -> bool + 'static) {
        let f = self.filter.clone();
        self.filter = self::sp_api_hidden_includes_construct_runtime::hidden_include::__private::Rc::new(
            self::sp_api_hidden_includes_construct_runtime::hidden_include::__private::Box::new(move |
                call|
            { f(call) && filter(call) }),
        );
    }
    fn reset_filter(&mut self) {
        let filter = <<Runtime as frame_system::Config>::BaseCallFilter as self::sp_api_hidden_includes_construct_runtime::hidden_include::traits::Contains<
            <Runtime as frame_system::Config>::RuntimeCall,
        >>::contains;
        self.filter = self::sp_api_hidden_includes_construct_runtime::hidden_include::__private::Rc::new(
            self::sp_api_hidden_includes_construct_runtime::hidden_include::__private::Box::new(
                filter,
            ),
        );
    }
    fn set_caller(&mut self, caller: OriginCaller) {
        self.caller = caller;
    }
    fn set_caller_from(&mut self, other: impl Into<Self>) {
        self.caller = other.into().caller;
    }
    fn filter_call(&self, call: &Self::Call) -> bool {
        match self.caller {
            OriginCaller::system(frame_system::Origin::<Runtime>::Root) => true,
            _ => (self.filter)(call),
        }
    }
    fn caller(&self) -> &Self::PalletsOrigin {
        &self.caller
    }
    fn into_caller(self) -> Self::PalletsOrigin {
        self.caller
    }
    fn try_with_caller<R>(
        mut self,
        f: impl FnOnce(Self::PalletsOrigin) -> Result<R, Self::PalletsOrigin>,
    ) -> Result<R, Self> {
        match f(self.caller) {
            Ok(r) => Ok(r),
            Err(caller) => {
                self.caller = caller;
                Err(self)
            }
        }
    }
    fn none() -> Self {
        frame_system::RawOrigin::None.into()
    }
    fn root() -> Self {
        frame_system::RawOrigin::Root.into()
    }
    fn signed(by: Self::AccountId) -> Self {
        frame_system::RawOrigin::Signed(by).into()
    }
}
#[allow(non_camel_case_types)]
pub enum OriginCaller {
    #[codec(index = 0u8)]
    system(frame_system::Origin<Runtime>),
    #[allow(dead_code)]
    #[codec(skip)]
    Void(
        self::sp_api_hidden_includes_construct_runtime::hidden_include::__private::Void,
    ),
}
#[automatically_derived]
#[allow(non_camel_case_types)]
impl ::core::clone::Clone for OriginCaller {
    #[inline]
    fn clone(&self) -> OriginCaller {
        match self {
            OriginCaller::system(__self_0) => {
                OriginCaller::system(::core::clone::Clone::clone(__self_0))
            }
            OriginCaller::Void(__self_0) => {
                OriginCaller::Void(::core::clone::Clone::clone(__self_0))
            }
        }
    }
}
#[automatically_derived]
#[allow(non_camel_case_types)]
impl ::core::marker::StructuralPartialEq for OriginCaller {}
#[automatically_derived]
#[allow(non_camel_case_types)]
impl ::core::cmp::PartialEq for OriginCaller {
    #[inline]
    fn eq(&self, other: &OriginCaller) -> bool {
        let __self_discr = ::core::intrinsics::discriminant_value(self);
        let __arg1_discr = ::core::intrinsics::discriminant_value(other);
        __self_discr == __arg1_discr
            && match (self, other) {
                (OriginCaller::system(__self_0), OriginCaller::system(__arg1_0)) => {
                    __self_0 == __arg1_0
                }
                (OriginCaller::Void(__self_0), OriginCaller::Void(__arg1_0)) => {
                    __self_0 == __arg1_0
                }
                _ => unsafe { ::core::intrinsics::unreachable() }
            }
    }
}
#[automatically_derived]
#[allow(non_camel_case_types)]
impl ::core::cmp::Eq for OriginCaller {
    #[inline]
    #[doc(hidden)]
    #[coverage(off)]
    fn assert_receiver_is_total_eq(&self) -> () {
        let _: ::core::cmp::AssertParamIsEq<frame_system::Origin<Runtime>>;
        let _: ::core::cmp::AssertParamIsEq<
            self::sp_api_hidden_includes_construct_runtime::hidden_include::__private::Void,
        >;
    }
}
#[automatically_derived]
#[allow(non_camel_case_types)]
impl ::core::fmt::Debug for OriginCaller {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match self {
            OriginCaller::system(__self_0) => {
                ::core::fmt::Formatter::debug_tuple_field1_finish(f, "system", &__self_0)
            }
            OriginCaller::Void(__self_0) => {
                ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Void", &__self_0)
            }
        }
    }
}
#[allow(deprecated)]
const _: () = {
    #[allow(non_camel_case_types)]
    #[automatically_derived]
    impl ::codec::Encode for OriginCaller {
        fn size_hint(&self) -> usize {
            1_usize
                + match *self {
                    OriginCaller::system(ref aa) => {
                        0_usize.saturating_add(::codec::Encode::size_hint(aa))
                    }
                    _ => 0_usize,
                }
        }
        fn encode_to<__CodecOutputEdqy: ::codec::Output + ?::core::marker::Sized>(
            &self,
            __codec_dest_edqy: &mut __CodecOutputEdqy,
        ) {
            #[automatically_derived]
            const _: () = {
                #[allow(clippy::unnecessary_cast)]
                #[allow(clippy::cast_possible_truncation)]
                const indices: [(usize, &'static str); 1usize] = [
                    ((0usize) as ::core::primitive::usize, "system"),
                ];
                const fn search_for_invalid_index(
                    array: &[(usize, &'static str); 1usize],
                ) -> (bool, usize) {
                    let mut i = 0;
                    while i < 1usize {
                        if array[i].0 > 255 {
                            return (true, i);
                        }
                        i += 1;
                    }
                    (false, 0)
                }
                const INVALID_INDEX: (bool, usize) = search_for_invalid_index(&indices);
                if INVALID_INDEX.0 {
                    let msg = ::const_format::pmr::__AssertStr {
                        x: {
                            use ::const_format::__cf_osRcTFl4A;
                            ({
                                #[doc(hidden)]
                                #[allow(unused_mut, non_snake_case)]
                                const CONCATP_NHPMWYD3NJA: &[__cf_osRcTFl4A::pmr::PArgument] = {
                                    let fmt = __cf_osRcTFl4A::pmr::FormattingFlags::NEW;
                                    &[
                                        __cf_osRcTFl4A::pmr::PConvWrapper("Found variant `")
                                            .to_pargument_display(fmt),
                                        __cf_osRcTFl4A::pmr::PConvWrapper(
                                                indices[INVALID_INDEX.1].1,
                                            )
                                            .to_pargument_display(fmt),
                                        __cf_osRcTFl4A::pmr::PConvWrapper("` with invalid index: `")
                                            .to_pargument_display(fmt),
                                        __cf_osRcTFl4A::pmr::PConvWrapper(
                                                indices[INVALID_INDEX.1].0,
                                            )
                                            .to_pargument_display(fmt),
                                        __cf_osRcTFl4A::pmr::PConvWrapper(
                                                "`. Max supported index is 255.",
                                            )
                                            .to_pargument_display(fmt),
                                    ]
                                };
                                {
                                    #[doc(hidden)]
                                    const ARR_LEN: usize = ::const_format::pmr::PArgument::calc_len(
                                        CONCATP_NHPMWYD3NJA,
                                    );
                                    #[doc(hidden)]
                                    const CONCAT_ARR: &::const_format::pmr::LenAndArray<
                                        [u8; ARR_LEN],
                                    > = &::const_format::pmr::__priv_concatenate(
                                        CONCATP_NHPMWYD3NJA,
                                    );
                                    #[doc(hidden)]
                                    #[allow(clippy::transmute_ptr_to_ptr)]
                                    const CONCAT_STR: &str = unsafe {
                                        let slice = ::const_format::pmr::transmute::<
                                            &[u8; ARR_LEN],
                                            &[u8; CONCAT_ARR.len],
                                        >(&CONCAT_ARR.array);
                                        {
                                            let bytes: &'static [::const_format::pmr::u8] = slice;
                                            let string: &'static ::const_format::pmr::str = {
                                                ::const_format::__hidden_utils::PtrToRef {
                                                    ptr: bytes as *const [::const_format::pmr::u8] as *const str,
                                                }
                                                    .reff
                                            };
                                            string
                                        }
                                    };
                                    CONCAT_STR
                                }
                            })
                        },
                    }
                        .x;
                    {
                        ::core::panicking::panic_display(&msg);
                    };
                }
                const fn duplicate_info(
                    array: &[(usize, &'static str); 1usize],
                ) -> (bool, usize, usize) {
                    let len = 1usize;
                    let mut i = 0usize;
                    while i < len {
                        let mut j = i + 1;
                        while j < len {
                            if array[i].0 == array[j].0 {
                                return (true, i, j);
                            }
                            j += 1;
                        }
                        i += 1;
                    }
                    (false, 0, 0)
                }
                const DUP_INFO: (bool, usize, usize) = duplicate_info(&indices);
                if DUP_INFO.0 {
                    let msg = ::const_format::pmr::__AssertStr {
                        x: {
                            use ::const_format::__cf_osRcTFl4A;
                            ({
                                #[doc(hidden)]
                                #[allow(unused_mut, non_snake_case)]
                                const CONCATP_NHPMWYD3NJA: &[__cf_osRcTFl4A::pmr::PArgument] = {
                                    let fmt = __cf_osRcTFl4A::pmr::FormattingFlags::NEW;
                                    &[
                                        __cf_osRcTFl4A::pmr::PConvWrapper(
                                                "Found variants that have duplicate indexes. Both `",
                                            )
                                            .to_pargument_display(fmt),
                                        __cf_osRcTFl4A::pmr::PConvWrapper(indices[DUP_INFO.1].1)
                                            .to_pargument_display(fmt),
                                        __cf_osRcTFl4A::pmr::PConvWrapper("` and `")
                                            .to_pargument_display(fmt),
                                        __cf_osRcTFl4A::pmr::PConvWrapper(indices[DUP_INFO.2].1)
                                            .to_pargument_display(fmt),
                                        __cf_osRcTFl4A::pmr::PConvWrapper("` have the index `")
                                            .to_pargument_display(fmt),
                                        __cf_osRcTFl4A::pmr::PConvWrapper(indices[DUP_INFO.1].0)
                                            .to_pargument_display(fmt),
                                        __cf_osRcTFl4A::pmr::PConvWrapper(
                                                "`. Use different indexes for each variant.",
                                            )
                                            .to_pargument_display(fmt),
                                    ]
                                };
                                {
                                    #[doc(hidden)]
                                    const ARR_LEN: usize = ::const_format::pmr::PArgument::calc_len(
                                        CONCATP_NHPMWYD3NJA,
                                    );
                                    #[doc(hidden)]
                                    const CONCAT_ARR: &::const_format::pmr::LenAndArray<
                                        [u8; ARR_LEN],
                                    > = &::const_format::pmr::__priv_concatenate(
                                        CONCATP_NHPMWYD3NJA,
                                    );
                                    #[doc(hidden)]
                                    #[allow(clippy::transmute_ptr_to_ptr)]
                                    const CONCAT_STR: &str = unsafe {
                                        let slice = ::const_format::pmr::transmute::<
                                            &[u8; ARR_LEN],
                                            &[u8; CONCAT_ARR.len],
                                        >(&CONCAT_ARR.array);
                                        {
                                            let bytes: &'static [::const_format::pmr::u8] = slice;
                                            let string: &'static ::const_format::pmr::str = {
                                                ::const_format::__hidden_utils::PtrToRef {
                                                    ptr: bytes as *const [::const_format::pmr::u8] as *const str,
                                                }
                                                    .reff
                                            };
                                            string
                                        }
                                    };
                                    CONCAT_STR
                                }
                            })
                        },
                    }
                        .x;
                    {
                        ::core::panicking::panic_display(&msg);
                    };
                }
            };
            match *self {
                OriginCaller::system(ref aa) => {
                    #[allow(clippy::unnecessary_cast)]
                    __codec_dest_edqy.push_byte((0usize) as ::core::primitive::u8);
                    ::codec::Encode::encode_to(aa, __codec_dest_edqy);
                }
                _ => {}
            }
        }
    }
    #[automatically_derived]
    impl ::codec::EncodeLike for OriginCaller {}
};
#[allow(deprecated)]
const _: () = {
    #[allow(non_camel_case_types)]
    #[automatically_derived]
    impl ::codec::Decode for OriginCaller {
        fn decode<__CodecInputEdqy: ::codec::Input>(
            __codec_input_edqy: &mut __CodecInputEdqy,
        ) -> ::core::result::Result<Self, ::codec::Error> {
            #[automatically_derived]
            const _: () = {
                #[allow(clippy::unnecessary_cast)]
                #[allow(clippy::cast_possible_truncation)]
                const indices: [(usize, &'static str); 1usize] = [
                    ((0usize) as ::core::primitive::usize, "system"),
                ];
                const fn search_for_invalid_index(
                    array: &[(usize, &'static str); 1usize],
                ) -> (bool, usize) {
                    let mut i = 0;
                    while i < 1usize {
                        if array[i].0 > 255 {
                            return (true, i);
                        }
                        i += 1;
                    }
                    (false, 0)
                }
                const INVALID_INDEX: (bool, usize) = search_for_invalid_index(&indices);
                if INVALID_INDEX.0 {
                    let msg = ::const_format::pmr::__AssertStr {
                        x: {
                            use ::const_format::__cf_osRcTFl4A;
                            ({
                                #[doc(hidden)]
                                #[allow(unused_mut, non_snake_case)]
                                const CONCATP_NHPMWYD3NJA: &[__cf_osRcTFl4A::pmr::PArgument] = {
                                    let fmt = __cf_osRcTFl4A::pmr::FormattingFlags::NEW;
                                    &[
                                        __cf_osRcTFl4A::pmr::PConvWrapper("Found variant `")
                                            .to_pargument_display(fmt),
                                        __cf_osRcTFl4A::pmr::PConvWrapper(
                                                indices[INVALID_INDEX.1].1,
                                            )
                                            .to_pargument_display(fmt),
                                        __cf_osRcTFl4A::pmr::PConvWrapper("` with invalid index: `")
                                            .to_pargument_display(fmt),
                                        __cf_osRcTFl4A::pmr::PConvWrapper(
                                                indices[INVALID_INDEX.1].0,
                                            )
                                            .to_pargument_display(fmt),
                                        __cf_osRcTFl4A::pmr::PConvWrapper(
                                                "`. Max supported index is 255.",
                                            )
                                            .to_pargument_display(fmt),
                                    ]
                                };
                                {
                                    #[doc(hidden)]
                                    const ARR_LEN: usize = ::const_format::pmr::PArgument::calc_len(
                                        CONCATP_NHPMWYD3NJA,
                                    );
                                    #[doc(hidden)]
                                    const CONCAT_ARR: &::const_format::pmr::LenAndArray<
                                        [u8; ARR_LEN],
                                    > = &::const_format::pmr::__priv_concatenate(
                                        CONCATP_NHPMWYD3NJA,
                                    );
                                    #[doc(hidden)]
                                    #[allow(clippy::transmute_ptr_to_ptr)]
                                    const CONCAT_STR: &str = unsafe {
                                        let slice = ::const_format::pmr::transmute::<
                                            &[u8; ARR_LEN],
                                            &[u8; CONCAT_ARR.len],
                                        >(&CONCAT_ARR.array);
                                        {
                                            let bytes: &'static [::const_format::pmr::u8] = slice;
                                            let string: &'static ::const_format::pmr::str = {
                                                ::const_format::__hidden_utils::PtrToRef {
                                                    ptr: bytes as *const [::const_format::pmr::u8] as *const str,
                                                }
                                                    .reff
                                            };
                                            string
                                        }
                                    };
                                    CONCAT_STR
                                }
                            })
                        },
                    }
                        .x;
                    {
                        ::core::panicking::panic_display(&msg);
                    };
                }
                const fn duplicate_info(
                    array: &[(usize, &'static str); 1usize],
                ) -> (bool, usize, usize) {
                    let len = 1usize;
                    let mut i = 0usize;
                    while i < len {
                        let mut j = i + 1;
                        while j < len {
                            if array[i].0 == array[j].0 {
                                return (true, i, j);
                            }
                            j += 1;
                        }
                        i += 1;
                    }
                    (false, 0, 0)
                }
                const DUP_INFO: (bool, usize, usize) = duplicate_info(&indices);
                if DUP_INFO.0 {
                    let msg = ::const_format::pmr::__AssertStr {
                        x: {
                            use ::const_format::__cf_osRcTFl4A;
                            ({
                                #[doc(hidden)]
                                #[allow(unused_mut, non_snake_case)]
                                const CONCATP_NHPMWYD3NJA: &[__cf_osRcTFl4A::pmr::PArgument] = {
                                    let fmt = __cf_osRcTFl4A::pmr::FormattingFlags::NEW;
                                    &[
                                        __cf_osRcTFl4A::pmr::PConvWrapper(
                                                "Found variants that have duplicate indexes. Both `",
                                            )
                                            .to_pargument_display(fmt),
                                        __cf_osRcTFl4A::pmr::PConvWrapper(indices[DUP_INFO.1].1)
                                            .to_pargument_display(fmt),
                                        __cf_osRcTFl4A::pmr::PConvWrapper("` and `")
                                            .to_pargument_display(fmt),
                                        __cf_osRcTFl4A::pmr::PConvWrapper(indices[DUP_INFO.2].1)
                                            .to_pargument_display(fmt),
                                        __cf_osRcTFl4A::pmr::PConvWrapper("` have the index `")
                                            .to_pargument_display(fmt),
                                        __cf_osRcTFl4A::pmr::PConvWrapper(indices[DUP_INFO.1].0)
                                            .to_pargument_display(fmt),
                                        __cf_osRcTFl4A::pmr::PConvWrapper(
                                                "`. Use different indexes for each variant.",
                                            )
                                            .to_pargument_display(fmt),
                                    ]
                                };
                                {
                                    #[doc(hidden)]
                                    const ARR_LEN: usize = ::const_format::pmr::PArgument::calc_len(
                                        CONCATP_NHPMWYD3NJA,
                                    );
                                    #[doc(hidden)]
                                    const CONCAT_ARR: &::const_format::pmr::LenAndArray<
                                        [u8; ARR_LEN],
                                    > = &::const_format::pmr::__priv_concatenate(
                                        CONCATP_NHPMWYD3NJA,
                                    );
                                    #[doc(hidden)]
                                    #[allow(clippy::transmute_ptr_to_ptr)]
                                    const CONCAT_STR: &str = unsafe {
                                        let slice = ::const_format::pmr::transmute::<
                                            &[u8; ARR_LEN],
                                            &[u8; CONCAT_ARR.len],
                                        >(&CONCAT_ARR.array);
                                        {
                                            let bytes: &'static [::const_format::pmr::u8] = slice;
                                            let string: &'static ::const_format::pmr::str = {
                                                ::const_format::__hidden_utils::PtrToRef {
                                                    ptr: bytes as *const [::const_format::pmr::u8] as *const str,
                                                }
                                                    .reff
                                            };
                                            string
                                        }
                                    };
                                    CONCAT_STR
                                }
                            })
                        },
                    }
                        .x;
                    {
                        ::core::panicking::panic_display(&msg);
                    };
                }
            };
            match __codec_input_edqy
                .read_byte()
                .map_err(|e| {
                    e
                        .chain(
                            "Could not decode `OriginCaller`, failed to read variant byte",
                        )
                })?
            {
                #[allow(clippy::unnecessary_cast)]
                #[allow(clippy::cast_possible_truncation)]
                __codec_x_edqy if __codec_x_edqy
                    == (0usize) as ::core::primitive::u8 => {
                    #[allow(clippy::redundant_closure_call)]
                    return (move || {
                        ::core::result::Result::Ok(
                            OriginCaller::system({
                                let __codec_res_edqy = <frame_system::Origin<
                                    Runtime,
                                > as ::codec::Decode>::decode(__codec_input_edqy);
                                match __codec_res_edqy {
                                    ::core::result::Result::Err(e) => {
                                        return ::core::result::Result::Err(
                                            e.chain("Could not decode `OriginCaller::system.0`"),
                                        );
                                    }
                                    ::core::result::Result::Ok(__codec_res_edqy) => {
                                        __codec_res_edqy
                                    }
                                }
                            }),
                        )
                    })();
                }
                _ => {
                    #[allow(clippy::redundant_closure_call)]
                    return (move || {
                        ::core::result::Result::Err(
                            <_ as ::core::convert::Into<
                                _,
                            >>::into(
                                "Could not decode `OriginCaller`, variant doesn't exist",
                            ),
                        )
                    })();
                }
            }
        }
    }
};
#[allow(deprecated)]
const _: () = {
    #[allow(non_camel_case_types)]
    fn check_struct() {
        fn check_field<T: ::codec::DecodeWithMemTracking>() {}
        check_field::<frame_system::Origin<Runtime>>();
    }
    #[automatically_derived]
    impl ::codec::DecodeWithMemTracking for OriginCaller {}
};
#[allow(non_upper_case_globals, deprecated, unused_attributes, unused_qualifications)]
const _: () = {
    impl ::scale_info::TypeInfo for OriginCaller {
        type Identity = Self;
        fn type_info() -> ::scale_info::Type {
            ::scale_info::Type::builder()
                .path(
                    ::scale_info::Path::new_with_replace(
                        "OriginCaller",
                        "polkadot_test",
                        &[],
                    ),
                )
                .type_params(::alloc::vec::Vec::new())
                .variant(
                    ::scale_info::build::Variants::new()
                        .variant(
                            "system",
                            |v| {
                                v
                                    .index(0u8 as ::core::primitive::u8)
                                    .fields(
                                        ::scale_info::build::Fields::unnamed()
                                            .field(|f| {
                                                f
                                                    .ty::<frame_system::Origin<Runtime>>()
                                                    .type_name("frame_system::Origin<Runtime>")
                                            }),
                                    )
                            },
                        ),
                )
        }
    }
};
const _: () = {
    #[automatically_derived]
    impl ::codec::MaxEncodedLen for OriginCaller {
        fn max_encoded_len() -> ::core::primitive::usize {
            0_usize
                .max(
                    0_usize
                        .saturating_add(
                            <frame_system::Origin<
                                Runtime,
                            > as ::codec::MaxEncodedLen>::max_encoded_len(),
                        ),
                )
                .saturating_add(1)
        }
    }
};
#[allow(dead_code)]
impl RuntimeOrigin {
    /// Create with system none origin and [`frame_system::Config::BaseCallFilter`].
    pub fn none() -> Self {
        <RuntimeOrigin as self::sp_api_hidden_includes_construct_runtime::hidden_include::traits::OriginTrait>::none()
    }
    /// Create with system root origin and [`frame_system::Config::BaseCallFilter`].
    pub fn root() -> Self {
        <RuntimeOrigin as self::sp_api_hidden_includes_construct_runtime::hidden_include::traits::OriginTrait>::root()
    }
    /// Create with system signed origin and [`frame_system::Config::BaseCallFilter`].
    pub fn signed(by: <Runtime as frame_system::Config>::AccountId) -> Self {
        <RuntimeOrigin as self::sp_api_hidden_includes_construct_runtime::hidden_include::traits::OriginTrait>::signed(
            by,
        )
    }
}
impl From<frame_system::Origin<Runtime>> for OriginCaller {
    fn from(x: frame_system::Origin<Runtime>) -> Self {
        OriginCaller::system(x)
    }
}
impl self::sp_api_hidden_includes_construct_runtime::hidden_include::traits::CallerTrait<
    <Runtime as frame_system::Config>::AccountId,
> for OriginCaller {
    fn into_system(
        self,
    ) -> Option<frame_system::RawOrigin<<Runtime as frame_system::Config>::AccountId>> {
        match self {
            OriginCaller::system(x) => Some(x),
            _ => None,
        }
    }
    fn as_system_ref(
        &self,
    ) -> Option<&frame_system::RawOrigin<<Runtime as frame_system::Config>::AccountId>> {
        match &self {
            OriginCaller::system(o) => Some(o),
            _ => None,
        }
    }
}
impl TryFrom<OriginCaller> for frame_system::Origin<Runtime> {
    type Error = OriginCaller;
    fn try_from(
        x: OriginCaller,
    ) -> core::result::Result<frame_system::Origin<Runtime>, OriginCaller> {
        if let OriginCaller::system(l) = x { Ok(l) } else { Err(x) }
    }
}
impl From<frame_system::Origin<Runtime>> for RuntimeOrigin {
    /// Convert to runtime origin, using as filter: [`frame_system::Config::BaseCallFilter`].
    fn from(x: frame_system::Origin<Runtime>) -> Self {
        let o: OriginCaller = x.into();
        o.into()
    }
}
impl From<OriginCaller> for RuntimeOrigin {
    fn from(x: OriginCaller) -> Self {
        let mut o = RuntimeOrigin {
            caller: x,
            filter: self::sp_api_hidden_includes_construct_runtime::hidden_include::__private::Rc::new(
                self::sp_api_hidden_includes_construct_runtime::hidden_include::__private::Box::new(|
                    _|
                true),
            ),
        };
        self::sp_api_hidden_includes_construct_runtime::hidden_include::traits::OriginTrait::reset_filter(
            &mut o,
        );
        o
    }
}
impl From<RuntimeOrigin>
for core::result::Result<frame_system::Origin<Runtime>, RuntimeOrigin> {
    /// NOTE: converting to pallet origin loses the origin filter information.
    fn from(val: RuntimeOrigin) -> Self {
        if let OriginCaller::system(l) = val.caller { Ok(l) } else { Err(val) }
    }
}
impl From<Option<<Runtime as frame_system::Config>::AccountId>> for RuntimeOrigin {
    /// Convert to runtime origin with caller being system signed or none and use filter [`frame_system::Config::BaseCallFilter`].
    fn from(x: Option<<Runtime as frame_system::Config>::AccountId>) -> Self {
        <frame_system::Origin<Runtime>>::from(x).into()
    }
}
impl self::sp_api_hidden_includes_construct_runtime::hidden_include::__private::AsSystemOriginSigner<
    <Runtime as frame_system::Config>::AccountId,
> for RuntimeOrigin {
    fn as_system_origin_signer(
        &self,
    ) -> Option<&<Runtime as frame_system::Config>::AccountId> {
        if let OriginCaller::system(
            frame_system::Origin::<Runtime>::Signed(ref signed),
        ) = &self.caller
        {
            Some(signed)
        } else {
            None
        }
    }
}
impl self::sp_api_hidden_includes_construct_runtime::hidden_include::__private::AsTransactionAuthorizedOrigin
for RuntimeOrigin {
    fn is_transaction_authorized(&self) -> bool {
        !#[allow(non_exhaustive_omitted_patterns)]
        match &self.caller {
            OriginCaller::system(frame_system::Origin::<Runtime>::None) => true,
            _ => false,
        }
    }
}
/// Mandatory system pallet that should always be included in a FRAME runtime.
pub type System = frame_system::Pallet<Runtime>;
/// Provides the ability to keep track of balances.
pub type Balances = pallet_balances::Pallet<Runtime>;
pub type Parameters = pallet_parameters::Pallet<Runtime>;
/// All pallets included in the runtime as a nested tuple of types.
pub type AllPalletsWithSystem = (System, Balances, Parameters);
/// All pallets included in the runtime as a nested tuple of types.
/// Excludes the System pallet.
pub type AllPalletsWithoutSystem = (Balances, Parameters);
/// Provides an implementation of `PalletInfo` to provide information
/// about the pallet setup in the runtime.
pub struct PalletInfo;
impl self::sp_api_hidden_includes_construct_runtime::hidden_include::traits::PalletInfo
for PalletInfo {
    fn index<P: 'static>() -> Option<usize> {
        let type_id = core::any::TypeId::of::<P>();
        if type_id == core::any::TypeId::of::<System>() {
            return Some(0usize);
        }
        if type_id == core::any::TypeId::of::<Balances>() {
            return Some(2usize);
        }
        if type_id == core::any::TypeId::of::<Parameters>() {
            return Some(3usize);
        }
        None
    }
    fn name<P: 'static>() -> Option<&'static str> {
        let type_id = core::any::TypeId::of::<P>();
        if type_id == core::any::TypeId::of::<System>() {
            return Some("System");
        }
        if type_id == core::any::TypeId::of::<Balances>() {
            return Some("Balances");
        }
        if type_id == core::any::TypeId::of::<Parameters>() {
            return Some("Parameters");
        }
        None
    }
    fn name_hash<P: 'static>() -> Option<[u8; 16]> {
        let type_id = core::any::TypeId::of::<P>();
        if type_id == core::any::TypeId::of::<System>() {
            return Some([
                38u8, 170u8, 57u8, 78u8, 234u8, 86u8, 48u8, 224u8, 124u8, 72u8, 174u8,
                12u8, 149u8, 88u8, 206u8, 247u8,
            ]);
        }
        if type_id == core::any::TypeId::of::<Balances>() {
            return Some([
                194u8, 38u8, 18u8, 118u8, 204u8, 157u8, 31u8, 133u8, 152u8, 234u8, 75u8,
                106u8, 116u8, 177u8, 92u8, 47u8,
            ]);
        }
        if type_id == core::any::TypeId::of::<Parameters>() {
            return Some([
                198u8, 59u8, 221u8, 74u8, 57u8, 9u8, 92u8, 207u8, 85u8, 98u8, 58u8,
                111u8, 40u8, 114u8, 191u8, 138u8,
            ]);
        }
        None
    }
    fn module_name<P: 'static>() -> Option<&'static str> {
        let type_id = core::any::TypeId::of::<P>();
        if type_id == core::any::TypeId::of::<System>() {
            return Some("frame_system");
        }
        if type_id == core::any::TypeId::of::<Balances>() {
            return Some("pallet_balances");
        }
        if type_id == core::any::TypeId::of::<Parameters>() {
            return Some("pallet_parameters");
        }
        None
    }
    fn crate_version<P: 'static>() -> Option<
        self::sp_api_hidden_includes_construct_runtime::hidden_include::traits::CrateVersion,
    > {
        let type_id = core::any::TypeId::of::<P>();
        if type_id == core::any::TypeId::of::<System>() {
            return Some(
                <frame_system::Pallet<
                    Runtime,
                > as self::sp_api_hidden_includes_construct_runtime::hidden_include::traits::PalletInfoAccess>::crate_version(),
            );
        }
        if type_id == core::any::TypeId::of::<Balances>() {
            return Some(
                <pallet_balances::Pallet<
                    Runtime,
                > as self::sp_api_hidden_includes_construct_runtime::hidden_include::traits::PalletInfoAccess>::crate_version(),
            );
        }
        if type_id == core::any::TypeId::of::<Parameters>() {
            return Some(
                <pallet_parameters::Pallet<
                    Runtime,
                > as self::sp_api_hidden_includes_construct_runtime::hidden_include::traits::PalletInfoAccess>::crate_version(),
            );
        }
        None
    }
}
/// The aggregated runtime call type.
pub enum RuntimeCall {
    #[codec(index = 0u8)]
    System(
        self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::CallableCallFor<
            System,
            Runtime,
        >,
    ),
    #[codec(index = 2u8)]
    Balances(
        self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::CallableCallFor<
            Balances,
            Runtime,
        >,
    ),
    #[codec(index = 3u8)]
    Parameters(
        self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::CallableCallFor<
            Parameters,
            Runtime,
        >,
    ),
}
#[automatically_derived]
impl ::core::clone::Clone for RuntimeCall {
    #[inline]
    fn clone(&self) -> RuntimeCall {
        match self {
            RuntimeCall::System(__self_0) => {
                RuntimeCall::System(::core::clone::Clone::clone(__self_0))
            }
            RuntimeCall::Balances(__self_0) => {
                RuntimeCall::Balances(::core::clone::Clone::clone(__self_0))
            }
            RuntimeCall::Parameters(__self_0) => {
                RuntimeCall::Parameters(::core::clone::Clone::clone(__self_0))
            }
        }
    }
}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for RuntimeCall {}
#[automatically_derived]
impl ::core::cmp::PartialEq for RuntimeCall {
    #[inline]
    fn eq(&self, other: &RuntimeCall) -> bool {
        let __self_discr = ::core::intrinsics::discriminant_value(self);
        let __arg1_discr = ::core::intrinsics::discriminant_value(other);
        __self_discr == __arg1_discr
            && match (self, other) {
                (RuntimeCall::System(__self_0), RuntimeCall::System(__arg1_0)) => {
                    __self_0 == __arg1_0
                }
                (RuntimeCall::Balances(__self_0), RuntimeCall::Balances(__arg1_0)) => {
                    __self_0 == __arg1_0
                }
                (
                    RuntimeCall::Parameters(__self_0),
                    RuntimeCall::Parameters(__arg1_0),
                ) => __self_0 == __arg1_0,
                _ => unsafe { ::core::intrinsics::unreachable() }
            }
    }
}
#[automatically_derived]
impl ::core::cmp::Eq for RuntimeCall {
    #[inline]
    #[doc(hidden)]
    #[coverage(off)]
    fn assert_receiver_is_total_eq(&self) -> () {
        let _: ::core::cmp::AssertParamIsEq<
            self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::CallableCallFor<
                System,
                Runtime,
            >,
        >;
        let _: ::core::cmp::AssertParamIsEq<
            self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::CallableCallFor<
                Balances,
                Runtime,
            >,
        >;
        let _: ::core::cmp::AssertParamIsEq<
            self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::CallableCallFor<
                Parameters,
                Runtime,
            >,
        >;
    }
}
#[allow(deprecated)]
const _: () = {
    #[automatically_derived]
    impl ::codec::Encode for RuntimeCall {
        fn size_hint(&self) -> usize {
            1_usize
                + match *self {
                    RuntimeCall::System(ref aa) => {
                        0_usize.saturating_add(::codec::Encode::size_hint(aa))
                    }
                    RuntimeCall::Balances(ref aa) => {
                        0_usize.saturating_add(::codec::Encode::size_hint(aa))
                    }
                    RuntimeCall::Parameters(ref aa) => {
                        0_usize.saturating_add(::codec::Encode::size_hint(aa))
                    }
                    _ => 0_usize,
                }
        }
        fn encode_to<__CodecOutputEdqy: ::codec::Output + ?::core::marker::Sized>(
            &self,
            __codec_dest_edqy: &mut __CodecOutputEdqy,
        ) {
            #[automatically_derived]
            const _: () = {
                #[allow(clippy::unnecessary_cast)]
                #[allow(clippy::cast_possible_truncation)]
                const indices: [(usize, &'static str); 3usize] = [
                    ((0usize) as ::core::primitive::usize, "System"),
                    ((2usize) as ::core::primitive::usize, "Balances"),
                    ((3usize) as ::core::primitive::usize, "Parameters"),
                ];
                const fn search_for_invalid_index(
                    array: &[(usize, &'static str); 3usize],
                ) -> (bool, usize) {
                    let mut i = 0;
                    while i < 3usize {
                        if array[i].0 > 255 {
                            return (true, i);
                        }
                        i += 1;
                    }
                    (false, 0)
                }
                const INVALID_INDEX: (bool, usize) = search_for_invalid_index(&indices);
                if INVALID_INDEX.0 {
                    let msg = ::const_format::pmr::__AssertStr {
                        x: {
                            use ::const_format::__cf_osRcTFl4A;
                            ({
                                #[doc(hidden)]
                                #[allow(unused_mut, non_snake_case)]
                                const CONCATP_NHPMWYD3NJA: &[__cf_osRcTFl4A::pmr::PArgument] = {
                                    let fmt = __cf_osRcTFl4A::pmr::FormattingFlags::NEW;
                                    &[
                                        __cf_osRcTFl4A::pmr::PConvWrapper("Found variant `")
                                            .to_pargument_display(fmt),
                                        __cf_osRcTFl4A::pmr::PConvWrapper(
                                                indices[INVALID_INDEX.1].1,
                                            )
                                            .to_pargument_display(fmt),
                                        __cf_osRcTFl4A::pmr::PConvWrapper("` with invalid index: `")
                                            .to_pargument_display(fmt),
                                        __cf_osRcTFl4A::pmr::PConvWrapper(
                                                indices[INVALID_INDEX.1].0,
                                            )
                                            .to_pargument_display(fmt),
                                        __cf_osRcTFl4A::pmr::PConvWrapper(
                                                "`. Max supported index is 255.",
                                            )
                                            .to_pargument_display(fmt),
                                    ]
                                };
                                {
                                    #[doc(hidden)]
                                    const ARR_LEN: usize = ::const_format::pmr::PArgument::calc_len(
                                        CONCATP_NHPMWYD3NJA,
                                    );
                                    #[doc(hidden)]
                                    const CONCAT_ARR: &::const_format::pmr::LenAndArray<
                                        [u8; ARR_LEN],
                                    > = &::const_format::pmr::__priv_concatenate(
                                        CONCATP_NHPMWYD3NJA,
                                    );
                                    #[doc(hidden)]
                                    #[allow(clippy::transmute_ptr_to_ptr)]
                                    const CONCAT_STR: &str = unsafe {
                                        let slice = ::const_format::pmr::transmute::<
                                            &[u8; ARR_LEN],
                                            &[u8; CONCAT_ARR.len],
                                        >(&CONCAT_ARR.array);
                                        {
                                            let bytes: &'static [::const_format::pmr::u8] = slice;
                                            let string: &'static ::const_format::pmr::str = {
                                                ::const_format::__hidden_utils::PtrToRef {
                                                    ptr: bytes as *const [::const_format::pmr::u8] as *const str,
                                                }
                                                    .reff
                                            };
                                            string
                                        }
                                    };
                                    CONCAT_STR
                                }
                            })
                        },
                    }
                        .x;
                    {
                        ::core::panicking::panic_display(&msg);
                    };
                }
                const fn duplicate_info(
                    array: &[(usize, &'static str); 3usize],
                ) -> (bool, usize, usize) {
                    let len = 3usize;
                    let mut i = 0usize;
                    while i < len {
                        let mut j = i + 1;
                        while j < len {
                            if array[i].0 == array[j].0 {
                                return (true, i, j);
                            }
                            j += 1;
                        }
                        i += 1;
                    }
                    (false, 0, 0)
                }
                const DUP_INFO: (bool, usize, usize) = duplicate_info(&indices);
                if DUP_INFO.0 {
                    let msg = ::const_format::pmr::__AssertStr {
                        x: {
                            use ::const_format::__cf_osRcTFl4A;
                            ({
                                #[doc(hidden)]
                                #[allow(unused_mut, non_snake_case)]
                                const CONCATP_NHPMWYD3NJA: &[__cf_osRcTFl4A::pmr::PArgument] = {
                                    let fmt = __cf_osRcTFl4A::pmr::FormattingFlags::NEW;
                                    &[
                                        __cf_osRcTFl4A::pmr::PConvWrapper(
                                                "Found variants that have duplicate indexes. Both `",
                                            )
                                            .to_pargument_display(fmt),
                                        __cf_osRcTFl4A::pmr::PConvWrapper(indices[DUP_INFO.1].1)
                                            .to_pargument_display(fmt),
                                        __cf_osRcTFl4A::pmr::PConvWrapper("` and `")
                                            .to_pargument_display(fmt),
                                        __cf_osRcTFl4A::pmr::PConvWrapper(indices[DUP_INFO.2].1)
                                            .to_pargument_display(fmt),
                                        __cf_osRcTFl4A::pmr::PConvWrapper("` have the index `")
                                            .to_pargument_display(fmt),
                                        __cf_osRcTFl4A::pmr::PConvWrapper(indices[DUP_INFO.1].0)
                                            .to_pargument_display(fmt),
                                        __cf_osRcTFl4A::pmr::PConvWrapper(
                                                "`. Use different indexes for each variant.",
                                            )
                                            .to_pargument_display(fmt),
                                    ]
                                };
                                {
                                    #[doc(hidden)]
                                    const ARR_LEN: usize = ::const_format::pmr::PArgument::calc_len(
                                        CONCATP_NHPMWYD3NJA,
                                    );
                                    #[doc(hidden)]
                                    const CONCAT_ARR: &::const_format::pmr::LenAndArray<
                                        [u8; ARR_LEN],
                                    > = &::const_format::pmr::__priv_concatenate(
                                        CONCATP_NHPMWYD3NJA,
                                    );
                                    #[doc(hidden)]
                                    #[allow(clippy::transmute_ptr_to_ptr)]
                                    const CONCAT_STR: &str = unsafe {
                                        let slice = ::const_format::pmr::transmute::<
                                            &[u8; ARR_LEN],
                                            &[u8; CONCAT_ARR.len],
                                        >(&CONCAT_ARR.array);
                                        {
                                            let bytes: &'static [::const_format::pmr::u8] = slice;
                                            let string: &'static ::const_format::pmr::str = {
                                                ::const_format::__hidden_utils::PtrToRef {
                                                    ptr: bytes as *const [::const_format::pmr::u8] as *const str,
                                                }
                                                    .reff
                                            };
                                            string
                                        }
                                    };
                                    CONCAT_STR
                                }
                            })
                        },
                    }
                        .x;
                    {
                        ::core::panicking::panic_display(&msg);
                    };
                }
            };
            match *self {
                RuntimeCall::System(ref aa) => {
                    #[allow(clippy::unnecessary_cast)]
                    __codec_dest_edqy.push_byte((0usize) as ::core::primitive::u8);
                    ::codec::Encode::encode_to(aa, __codec_dest_edqy);
                }
                RuntimeCall::Balances(ref aa) => {
                    #[allow(clippy::unnecessary_cast)]
                    __codec_dest_edqy.push_byte((2usize) as ::core::primitive::u8);
                    ::codec::Encode::encode_to(aa, __codec_dest_edqy);
                }
                RuntimeCall::Parameters(ref aa) => {
                    #[allow(clippy::unnecessary_cast)]
                    __codec_dest_edqy.push_byte((3usize) as ::core::primitive::u8);
                    ::codec::Encode::encode_to(aa, __codec_dest_edqy);
                }
                _ => {}
            }
        }
    }
    #[automatically_derived]
    impl ::codec::EncodeLike for RuntimeCall {}
};
#[allow(deprecated)]
const _: () = {
    #[automatically_derived]
    impl ::codec::Decode for RuntimeCall {
        fn decode<__CodecInputEdqy: ::codec::Input>(
            __codec_input_edqy: &mut __CodecInputEdqy,
        ) -> ::core::result::Result<Self, ::codec::Error> {
            #[automatically_derived]
            const _: () = {
                #[allow(clippy::unnecessary_cast)]
                #[allow(clippy::cast_possible_truncation)]
                const indices: [(usize, &'static str); 3usize] = [
                    ((0usize) as ::core::primitive::usize, "System"),
                    ((2usize) as ::core::primitive::usize, "Balances"),
                    ((3usize) as ::core::primitive::usize, "Parameters"),
                ];
                const fn search_for_invalid_index(
                    array: &[(usize, &'static str); 3usize],
                ) -> (bool, usize) {
                    let mut i = 0;
                    while i < 3usize {
                        if array[i].0 > 255 {
                            return (true, i);
                        }
                        i += 1;
                    }
                    (false, 0)
                }
                const INVALID_INDEX: (bool, usize) = search_for_invalid_index(&indices);
                if INVALID_INDEX.0 {
                    let msg = ::const_format::pmr::__AssertStr {
                        x: {
                            use ::const_format::__cf_osRcTFl4A;
                            ({
                                #[doc(hidden)]
                                #[allow(unused_mut, non_snake_case)]
                                const CONCATP_NHPMWYD3NJA: &[__cf_osRcTFl4A::pmr::PArgument] = {
                                    let fmt = __cf_osRcTFl4A::pmr::FormattingFlags::NEW;
                                    &[
                                        __cf_osRcTFl4A::pmr::PConvWrapper("Found variant `")
                                            .to_pargument_display(fmt),
                                        __cf_osRcTFl4A::pmr::PConvWrapper(
                                                indices[INVALID_INDEX.1].1,
                                            )
                                            .to_pargument_display(fmt),
                                        __cf_osRcTFl4A::pmr::PConvWrapper("` with invalid index: `")
                                            .to_pargument_display(fmt),
                                        __cf_osRcTFl4A::pmr::PConvWrapper(
                                                indices[INVALID_INDEX.1].0,
                                            )
                                            .to_pargument_display(fmt),
                                        __cf_osRcTFl4A::pmr::PConvWrapper(
                                                "`. Max supported index is 255.",
                                            )
                                            .to_pargument_display(fmt),
                                    ]
                                };
                                {
                                    #[doc(hidden)]
                                    const ARR_LEN: usize = ::const_format::pmr::PArgument::calc_len(
                                        CONCATP_NHPMWYD3NJA,
                                    );
                                    #[doc(hidden)]
                                    const CONCAT_ARR: &::const_format::pmr::LenAndArray<
                                        [u8; ARR_LEN],
                                    > = &::const_format::pmr::__priv_concatenate(
                                        CONCATP_NHPMWYD3NJA,
                                    );
                                    #[doc(hidden)]
                                    #[allow(clippy::transmute_ptr_to_ptr)]
                                    const CONCAT_STR: &str = unsafe {
                                        let slice = ::const_format::pmr::transmute::<
                                            &[u8; ARR_LEN],
                                            &[u8; CONCAT_ARR.len],
                                        >(&CONCAT_ARR.array);
                                        {
                                            let bytes: &'static [::const_format::pmr::u8] = slice;
                                            let string: &'static ::const_format::pmr::str = {
                                                ::const_format::__hidden_utils::PtrToRef {
                                                    ptr: bytes as *const [::const_format::pmr::u8] as *const str,
                                                }
                                                    .reff
                                            };
                                            string
                                        }
                                    };
                                    CONCAT_STR
                                }
                            })
                        },
                    }
                        .x;
                    {
                        ::core::panicking::panic_display(&msg);
                    };
                }
                const fn duplicate_info(
                    array: &[(usize, &'static str); 3usize],
                ) -> (bool, usize, usize) {
                    let len = 3usize;
                    let mut i = 0usize;
                    while i < len {
                        let mut j = i + 1;
                        while j < len {
                            if array[i].0 == array[j].0 {
                                return (true, i, j);
                            }
                            j += 1;
                        }
                        i += 1;
                    }
                    (false, 0, 0)
                }
                const DUP_INFO: (bool, usize, usize) = duplicate_info(&indices);
                if DUP_INFO.0 {
                    let msg = ::const_format::pmr::__AssertStr {
                        x: {
                            use ::const_format::__cf_osRcTFl4A;
                            ({
                                #[doc(hidden)]
                                #[allow(unused_mut, non_snake_case)]
                                const CONCATP_NHPMWYD3NJA: &[__cf_osRcTFl4A::pmr::PArgument] = {
                                    let fmt = __cf_osRcTFl4A::pmr::FormattingFlags::NEW;
                                    &[
                                        __cf_osRcTFl4A::pmr::PConvWrapper(
                                                "Found variants that have duplicate indexes. Both `",
                                            )
                                            .to_pargument_display(fmt),
                                        __cf_osRcTFl4A::pmr::PConvWrapper(indices[DUP_INFO.1].1)
                                            .to_pargument_display(fmt),
                                        __cf_osRcTFl4A::pmr::PConvWrapper("` and `")
                                            .to_pargument_display(fmt),
                                        __cf_osRcTFl4A::pmr::PConvWrapper(indices[DUP_INFO.2].1)
                                            .to_pargument_display(fmt),
                                        __cf_osRcTFl4A::pmr::PConvWrapper("` have the index `")
                                            .to_pargument_display(fmt),
                                        __cf_osRcTFl4A::pmr::PConvWrapper(indices[DUP_INFO.1].0)
                                            .to_pargument_display(fmt),
                                        __cf_osRcTFl4A::pmr::PConvWrapper(
                                                "`. Use different indexes for each variant.",
                                            )
                                            .to_pargument_display(fmt),
                                    ]
                                };
                                {
                                    #[doc(hidden)]
                                    const ARR_LEN: usize = ::const_format::pmr::PArgument::calc_len(
                                        CONCATP_NHPMWYD3NJA,
                                    );
                                    #[doc(hidden)]
                                    const CONCAT_ARR: &::const_format::pmr::LenAndArray<
                                        [u8; ARR_LEN],
                                    > = &::const_format::pmr::__priv_concatenate(
                                        CONCATP_NHPMWYD3NJA,
                                    );
                                    #[doc(hidden)]
                                    #[allow(clippy::transmute_ptr_to_ptr)]
                                    const CONCAT_STR: &str = unsafe {
                                        let slice = ::const_format::pmr::transmute::<
                                            &[u8; ARR_LEN],
                                            &[u8; CONCAT_ARR.len],
                                        >(&CONCAT_ARR.array);
                                        {
                                            let bytes: &'static [::const_format::pmr::u8] = slice;
                                            let string: &'static ::const_format::pmr::str = {
                                                ::const_format::__hidden_utils::PtrToRef {
                                                    ptr: bytes as *const [::const_format::pmr::u8] as *const str,
                                                }
                                                    .reff
                                            };
                                            string
                                        }
                                    };
                                    CONCAT_STR
                                }
                            })
                        },
                    }
                        .x;
                    {
                        ::core::panicking::panic_display(&msg);
                    };
                }
            };
            match __codec_input_edqy
                .read_byte()
                .map_err(|e| {
                    e
                        .chain(
                            "Could not decode `RuntimeCall`, failed to read variant byte",
                        )
                })?
            {
                #[allow(clippy::unnecessary_cast)]
                #[allow(clippy::cast_possible_truncation)]
                __codec_x_edqy if __codec_x_edqy
                    == (0usize) as ::core::primitive::u8 => {
                    #[allow(clippy::redundant_closure_call)]
                    return (move || {
                        ::core::result::Result::Ok(
                            RuntimeCall::System({
                                let __codec_res_edqy = <self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::CallableCallFor<
                                    System,
                                    Runtime,
                                > as ::codec::Decode>::decode(__codec_input_edqy);
                                match __codec_res_edqy {
                                    ::core::result::Result::Err(e) => {
                                        return ::core::result::Result::Err(
                                            e.chain("Could not decode `RuntimeCall::System.0`"),
                                        );
                                    }
                                    ::core::result::Result::Ok(__codec_res_edqy) => {
                                        __codec_res_edqy
                                    }
                                }
                            }),
                        )
                    })();
                }
                #[allow(clippy::unnecessary_cast)]
                #[allow(clippy::cast_possible_truncation)]
                __codec_x_edqy if __codec_x_edqy
                    == (2usize) as ::core::primitive::u8 => {
                    #[allow(clippy::redundant_closure_call)]
                    return (move || {
                        ::core::result::Result::Ok(
                            RuntimeCall::Balances({
                                let __codec_res_edqy = <self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::CallableCallFor<
                                    Balances,
                                    Runtime,
                                > as ::codec::Decode>::decode(__codec_input_edqy);
                                match __codec_res_edqy {
                                    ::core::result::Result::Err(e) => {
                                        return ::core::result::Result::Err(
                                            e.chain("Could not decode `RuntimeCall::Balances.0`"),
                                        );
                                    }
                                    ::core::result::Result::Ok(__codec_res_edqy) => {
                                        __codec_res_edqy
                                    }
                                }
                            }),
                        )
                    })();
                }
                #[allow(clippy::unnecessary_cast)]
                #[allow(clippy::cast_possible_truncation)]
                __codec_x_edqy if __codec_x_edqy
                    == (3usize) as ::core::primitive::u8 => {
                    #[allow(clippy::redundant_closure_call)]
                    return (move || {
                        ::core::result::Result::Ok(
                            RuntimeCall::Parameters({
                                let __codec_res_edqy = <self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::CallableCallFor<
                                    Parameters,
                                    Runtime,
                                > as ::codec::Decode>::decode(__codec_input_edqy);
                                match __codec_res_edqy {
                                    ::core::result::Result::Err(e) => {
                                        return ::core::result::Result::Err(
                                            e.chain("Could not decode `RuntimeCall::Parameters.0`"),
                                        );
                                    }
                                    ::core::result::Result::Ok(__codec_res_edqy) => {
                                        __codec_res_edqy
                                    }
                                }
                            }),
                        )
                    })();
                }
                _ => {
                    #[allow(clippy::redundant_closure_call)]
                    return (move || {
                        ::core::result::Result::Err(
                            <_ as ::core::convert::Into<
                                _,
                            >>::into(
                                "Could not decode `RuntimeCall`, variant doesn't exist",
                            ),
                        )
                    })();
                }
            }
        }
    }
};
#[allow(deprecated)]
const _: () = {
    fn check_struct() {
        fn check_field<T: ::codec::DecodeWithMemTracking>() {}
        check_field::<
            self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::CallableCallFor<
                System,
                Runtime,
            >,
        >();
        check_field::<
            self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::CallableCallFor<
                Balances,
                Runtime,
            >,
        >();
        check_field::<
            self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::CallableCallFor<
                Parameters,
                Runtime,
            >,
        >();
    }
    #[automatically_derived]
    impl ::codec::DecodeWithMemTracking for RuntimeCall {}
};
#[allow(non_upper_case_globals, deprecated, unused_attributes, unused_qualifications)]
const _: () = {
    impl ::scale_info::TypeInfo for RuntimeCall {
        type Identity = Self;
        fn type_info() -> ::scale_info::Type {
            ::scale_info::Type::builder()
                .path(
                    ::scale_info::Path::new_with_replace(
                        "RuntimeCall",
                        "polkadot_test",
                        &[],
                    ),
                )
                .type_params(::alloc::vec::Vec::new())
                .docs(&["The aggregated runtime call type."])
                .variant(
                    ::scale_info::build::Variants::new()
                        .variant(
                            "System",
                            |v| {
                                v
                                    .index(0u8 as ::core::primitive::u8)
                                    .fields(
                                        ::scale_info::build::Fields::unnamed()
                                            .field(|f| {
                                                f
                                                    .ty::<
                                                        self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::CallableCallFor<
                                                            System,
                                                            Runtime,
                                                        >,
                                                    >()
                                                    .type_name(
                                                        "self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch\n::CallableCallFor<System, Runtime>",
                                                    )
                                            }),
                                    )
                            },
                        )
                        .variant(
                            "Balances",
                            |v| {
                                v
                                    .index(2u8 as ::core::primitive::u8)
                                    .fields(
                                        ::scale_info::build::Fields::unnamed()
                                            .field(|f| {
                                                f
                                                    .ty::<
                                                        self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::CallableCallFor<
                                                            Balances,
                                                            Runtime,
                                                        >,
                                                    >()
                                                    .type_name(
                                                        "self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch\n::CallableCallFor<Balances, Runtime>",
                                                    )
                                            }),
                                    )
                            },
                        )
                        .variant(
                            "Parameters",
                            |v| {
                                v
                                    .index(3u8 as ::core::primitive::u8)
                                    .fields(
                                        ::scale_info::build::Fields::unnamed()
                                            .field(|f| {
                                                f
                                                    .ty::<
                                                        self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::CallableCallFor<
                                                            Parameters,
                                                            Runtime,
                                                        >,
                                                    >()
                                                    .type_name(
                                                        "self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch\n::CallableCallFor<Parameters, Runtime>",
                                                    )
                                            }),
                                    )
                            },
                        ),
                )
        }
    }
};
impl core::fmt::Debug for RuntimeCall {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::System(ref a0) => {
                fmt.debug_tuple("RuntimeCall::System").field(a0).finish()
            }
            Self::Balances(ref a0) => {
                fmt.debug_tuple("RuntimeCall::Balances").field(a0).finish()
            }
            Self::Parameters(ref a0) => {
                fmt.debug_tuple("RuntimeCall::Parameters").field(a0).finish()
            }
            _ => Ok(()),
        }
    }
}
impl self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::GetDispatchInfo
for RuntimeCall {
    fn get_dispatch_info(
        &self,
    ) -> self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::DispatchInfo {
        match self {
            RuntimeCall::System(call) => call.get_dispatch_info(),
            RuntimeCall::Balances(call) => call.get_dispatch_info(),
            RuntimeCall::Parameters(call) => call.get_dispatch_info(),
        }
    }
}
impl self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::CheckIfFeeless
for RuntimeCall {
    type Origin = frame_system::pallet_prelude::OriginFor<Runtime>;
    fn is_feeless(&self, origin: &Self::Origin) -> bool {
        match self {
            RuntimeCall::System(call) => call.is_feeless(origin),
            RuntimeCall::Balances(call) => call.is_feeless(origin),
            RuntimeCall::Parameters(call) => call.is_feeless(origin),
        }
    }
}
impl self::sp_api_hidden_includes_construct_runtime::hidden_include::traits::GetCallMetadata
for RuntimeCall {
    fn get_call_metadata(
        &self,
    ) -> self::sp_api_hidden_includes_construct_runtime::hidden_include::traits::CallMetadata {
        use self::sp_api_hidden_includes_construct_runtime::hidden_include::traits::GetCallName;
        match self {
            RuntimeCall::System(call) => {
                let function_name = call.get_call_name();
                let pallet_name = "System";
                self::sp_api_hidden_includes_construct_runtime::hidden_include::traits::CallMetadata {
                    function_name,
                    pallet_name,
                }
            }
            RuntimeCall::Balances(call) => {
                let function_name = call.get_call_name();
                let pallet_name = "Balances";
                self::sp_api_hidden_includes_construct_runtime::hidden_include::traits::CallMetadata {
                    function_name,
                    pallet_name,
                }
            }
            RuntimeCall::Parameters(call) => {
                let function_name = call.get_call_name();
                let pallet_name = "Parameters";
                self::sp_api_hidden_includes_construct_runtime::hidden_include::traits::CallMetadata {
                    function_name,
                    pallet_name,
                }
            }
        }
    }
    fn get_module_names() -> &'static [&'static str] {
        &["System", "Balances", "Parameters"]
    }
    fn get_call_names(module: &str) -> &'static [&'static str] {
        use self::sp_api_hidden_includes_construct_runtime::hidden_include::{
            dispatch::Callable, traits::GetCallName,
        };
        match module {
            "System" => {
                <<System as Callable<
                    Runtime,
                >>::RuntimeCall as GetCallName>::get_call_names()
            }
            "Balances" => {
                <<Balances as Callable<
                    Runtime,
                >>::RuntimeCall as GetCallName>::get_call_names()
            }
            "Parameters" => {
                <<Parameters as Callable<
                    Runtime,
                >>::RuntimeCall as GetCallName>::get_call_names()
            }
            _ => ::core::panicking::panic("internal error: entered unreachable code"),
        }
    }
}
impl self::sp_api_hidden_includes_construct_runtime::hidden_include::__private::Dispatchable
for RuntimeCall {
    type RuntimeOrigin = RuntimeOrigin;
    type Config = RuntimeCall;
    type Info = self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::DispatchInfo;
    type PostInfo = self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::PostDispatchInfo;
    fn dispatch(
        self,
        origin: RuntimeOrigin,
    ) -> self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::DispatchResultWithPostInfo {
        if !<Self::RuntimeOrigin as self::sp_api_hidden_includes_construct_runtime::hidden_include::traits::OriginTrait>::filter_call(
            &origin,
            &self,
        ) {
            return ::core::result::Result::Err(
                frame_system::Error::<Runtime>::CallFiltered.into(),
            );
        }
        self::sp_api_hidden_includes_construct_runtime::hidden_include::traits::UnfilteredDispatchable::dispatch_bypass_filter(
            self,
            origin,
        )
    }
}
impl self::sp_api_hidden_includes_construct_runtime::hidden_include::traits::UnfilteredDispatchable
for RuntimeCall {
    type RuntimeOrigin = RuntimeOrigin;
    fn dispatch_bypass_filter(
        self,
        origin: RuntimeOrigin,
    ) -> self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::DispatchResultWithPostInfo {
        match self {
            RuntimeCall::System(call) => {
                self::sp_api_hidden_includes_construct_runtime::hidden_include::traits::UnfilteredDispatchable::dispatch_bypass_filter(
                    call,
                    origin,
                )
            }
            RuntimeCall::Balances(call) => {
                self::sp_api_hidden_includes_construct_runtime::hidden_include::traits::UnfilteredDispatchable::dispatch_bypass_filter(
                    call,
                    origin,
                )
            }
            RuntimeCall::Parameters(call) => {
                self::sp_api_hidden_includes_construct_runtime::hidden_include::traits::UnfilteredDispatchable::dispatch_bypass_filter(
                    call,
                    origin,
                )
            }
        }
    }
}
impl self::sp_api_hidden_includes_construct_runtime::hidden_include::traits::IsSubType<
    self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::CallableCallFor<
        System,
        Runtime,
    >,
> for RuntimeCall {
    #[allow(unreachable_patterns)]
    fn is_sub_type(
        &self,
    ) -> Option<
        &self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::CallableCallFor<
            System,
            Runtime,
        >,
    > {
        match self {
            RuntimeCall::System(call) => Some(call),
            _ => None,
        }
    }
}
impl From<
    self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::CallableCallFor<
        System,
        Runtime,
    >,
> for RuntimeCall {
    fn from(
        call: self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::CallableCallFor<
            System,
            Runtime,
        >,
    ) -> Self {
        RuntimeCall::System(call)
    }
}
impl self::sp_api_hidden_includes_construct_runtime::hidden_include::traits::IsSubType<
    self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::CallableCallFor<
        Balances,
        Runtime,
    >,
> for RuntimeCall {
    #[allow(unreachable_patterns)]
    fn is_sub_type(
        &self,
    ) -> Option<
        &self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::CallableCallFor<
            Balances,
            Runtime,
        >,
    > {
        match self {
            RuntimeCall::Balances(call) => Some(call),
            _ => None,
        }
    }
}
impl From<
    self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::CallableCallFor<
        Balances,
        Runtime,
    >,
> for RuntimeCall {
    fn from(
        call: self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::CallableCallFor<
            Balances,
            Runtime,
        >,
    ) -> Self {
        RuntimeCall::Balances(call)
    }
}
impl self::sp_api_hidden_includes_construct_runtime::hidden_include::traits::IsSubType<
    self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::CallableCallFor<
        Parameters,
        Runtime,
    >,
> for RuntimeCall {
    #[allow(unreachable_patterns)]
    fn is_sub_type(
        &self,
    ) -> Option<
        &self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::CallableCallFor<
            Parameters,
            Runtime,
        >,
    > {
        match self {
            RuntimeCall::Parameters(call) => Some(call),
            _ => None,
        }
    }
}
impl From<
    self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::CallableCallFor<
        Parameters,
        Runtime,
    >,
> for RuntimeCall {
    fn from(
        call: self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::CallableCallFor<
            Parameters,
            Runtime,
        >,
    ) -> Self {
        RuntimeCall::Parameters(call)
    }
}
impl self::sp_api_hidden_includes_construct_runtime::hidden_include::traits::Authorize
for RuntimeCall {
    fn authorize(
        &self,
        source: self::sp_api_hidden_includes_construct_runtime::hidden_include::pallet_prelude::TransactionSource,
    ) -> ::core::option::Option<
        ::core::result::Result<
            (
                self::sp_api_hidden_includes_construct_runtime::hidden_include::pallet_prelude::ValidTransaction,
                self::sp_api_hidden_includes_construct_runtime::hidden_include::pallet_prelude::Weight,
            ),
            self::sp_api_hidden_includes_construct_runtime::hidden_include::pallet_prelude::TransactionValidityError,
        >,
    > {
        match self {
            RuntimeCall::System(call) => {
                self::sp_api_hidden_includes_construct_runtime::hidden_include::traits::Authorize::authorize(
                    call,
                    source,
                )
            }
            RuntimeCall::Balances(call) => {
                self::sp_api_hidden_includes_construct_runtime::hidden_include::traits::Authorize::authorize(
                    call,
                    source,
                )
            }
            RuntimeCall::Parameters(call) => {
                self::sp_api_hidden_includes_construct_runtime::hidden_include::traits::Authorize::authorize(
                    call,
                    source,
                )
            }
        }
    }
    fn weight_of_authorize(
        &self,
    ) -> self::sp_api_hidden_includes_construct_runtime::hidden_include::pallet_prelude::Weight {
        match self {
            RuntimeCall::System(call) => {
                self::sp_api_hidden_includes_construct_runtime::hidden_include::traits::Authorize::weight_of_authorize(
                    call,
                )
            }
            RuntimeCall::Balances(call) => {
                self::sp_api_hidden_includes_construct_runtime::hidden_include::traits::Authorize::weight_of_authorize(
                    call,
                )
            }
            RuntimeCall::Parameters(call) => {
                self::sp_api_hidden_includes_construct_runtime::hidden_include::traits::Authorize::weight_of_authorize(
                    call,
                )
            }
        }
    }
}
/// An aggregation of all `Task` enums across all pallets included in the current runtime.
pub enum RuntimeTask {}
#[automatically_derived]
impl ::core::clone::Clone for RuntimeTask {
    #[inline]
    fn clone(&self) -> RuntimeTask {
        match *self {}
    }
}
#[automatically_derived]
impl ::core::cmp::Eq for RuntimeTask {
    #[inline]
    #[doc(hidden)]
    #[coverage(off)]
    fn assert_receiver_is_total_eq(&self) -> () {}
}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for RuntimeTask {}
#[automatically_derived]
impl ::core::cmp::PartialEq for RuntimeTask {
    #[inline]
    fn eq(&self, other: &RuntimeTask) -> bool {
        match *self {}
    }
}
#[allow(deprecated)]
const _: () = {
    #[automatically_derived]
    impl ::codec::Encode for RuntimeTask {}
    #[automatically_derived]
    impl ::codec::EncodeLike for RuntimeTask {}
};
#[allow(deprecated)]
const _: () = {
    #[automatically_derived]
    impl ::codec::Decode for RuntimeTask {
        fn decode<__CodecInputEdqy: ::codec::Input>(
            __codec_input_edqy: &mut __CodecInputEdqy,
        ) -> ::core::result::Result<Self, ::codec::Error> {
            match __codec_input_edqy
                .read_byte()
                .map_err(|e| {
                    e
                        .chain(
                            "Could not decode `RuntimeTask`, failed to read variant byte",
                        )
                })?
            {
                _ => {
                    #[allow(clippy::redundant_closure_call)]
                    return (move || {
                        ::core::result::Result::Err(
                            <_ as ::core::convert::Into<
                                _,
                            >>::into(
                                "Could not decode `RuntimeTask`, variant doesn't exist",
                            ),
                        )
                    })();
                }
            }
        }
    }
};
#[allow(deprecated)]
const _: () = {
    fn check_struct() {
        fn check_field<T: ::codec::DecodeWithMemTracking>() {}
    }
    #[automatically_derived]
    impl ::codec::DecodeWithMemTracking for RuntimeTask {}
};
#[allow(non_upper_case_globals, deprecated, unused_attributes, unused_qualifications)]
const _: () = {
    impl ::scale_info::TypeInfo for RuntimeTask {
        type Identity = Self;
        fn type_info() -> ::scale_info::Type {
            ::scale_info::Type::builder()
                .path(
                    ::scale_info::Path::new_with_replace(
                        "RuntimeTask",
                        "polkadot_test",
                        &[],
                    ),
                )
                .type_params(::alloc::vec::Vec::new())
                .docs(
                    &[
                        "An aggregation of all `Task` enums across all pallets included in the current runtime.",
                    ],
                )
                .variant(::scale_info::build::Variants::new())
        }
    }
};
impl core::fmt::Debug for RuntimeTask {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            _ => Ok(()),
        }
    }
}
#[automatically_derived]
impl self::sp_api_hidden_includes_construct_runtime::hidden_include::traits::Task
for RuntimeTask {
    type Enumeration = self::sp_api_hidden_includes_construct_runtime::hidden_include::traits::tasks::__private::IntoIter<
        RuntimeTask,
    >;
    fn is_valid(&self) -> bool {
        match self {
            _ => {
                ::core::panicking::panic_fmt(
                    format_args!(
                        "internal error: entered unreachable code: {0}",
                        format_args!(
                            "cannot have an instantiated RuntimeTask without some Task variant in the runtime. QED",
                        ),
                    ),
                );
            }
        }
    }
    fn run(
        &self,
    ) -> Result<
        (),
        self::sp_api_hidden_includes_construct_runtime::hidden_include::traits::tasks::__private::DispatchError,
    > {
        match self {
            _ => {
                ::core::panicking::panic_fmt(
                    format_args!(
                        "internal error: entered unreachable code: {0}",
                        format_args!(
                            "cannot have an instantiated RuntimeTask without some Task variant in the runtime. QED",
                        ),
                    ),
                );
            }
        }
    }
    fn weight(
        &self,
    ) -> self::sp_api_hidden_includes_construct_runtime::hidden_include::pallet_prelude::Weight {
        match self {
            _ => {
                ::core::panicking::panic_fmt(
                    format_args!(
                        "internal error: entered unreachable code: {0}",
                        format_args!(
                            "cannot have an instantiated RuntimeTask without some Task variant in the runtime. QED",
                        ),
                    ),
                );
            }
        }
    }
    fn task_index(&self) -> u32 {
        match self {
            _ => {
                ::core::panicking::panic_fmt(
                    format_args!(
                        "internal error: entered unreachable code: {0}",
                        format_args!(
                            "cannot have an instantiated RuntimeTask without some Task variant in the runtime. QED",
                        ),
                    ),
                );
            }
        }
    }
    fn iter() -> Self::Enumeration {
        let mut all_tasks = Vec::new();
        all_tasks.into_iter()
    }
}
/// Runtime query type.
pub enum RuntimeViewFunction {}
#[automatically_derived]
impl ::core::clone::Clone for RuntimeViewFunction {
    #[inline]
    fn clone(&self) -> RuntimeViewFunction {
        match *self {}
    }
}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for RuntimeViewFunction {}
#[automatically_derived]
impl ::core::cmp::PartialEq for RuntimeViewFunction {
    #[inline]
    fn eq(&self, other: &RuntimeViewFunction) -> bool {
        match *self {}
    }
}
#[automatically_derived]
impl ::core::cmp::Eq for RuntimeViewFunction {
    #[inline]
    #[doc(hidden)]
    #[coverage(off)]
    fn assert_receiver_is_total_eq(&self) -> () {}
}
#[allow(deprecated)]
const _: () = {
    #[automatically_derived]
    impl ::codec::Encode for RuntimeViewFunction {}
    #[automatically_derived]
    impl ::codec::EncodeLike for RuntimeViewFunction {}
};
#[allow(deprecated)]
const _: () = {
    #[automatically_derived]
    impl ::codec::Decode for RuntimeViewFunction {
        fn decode<__CodecInputEdqy: ::codec::Input>(
            __codec_input_edqy: &mut __CodecInputEdqy,
        ) -> ::core::result::Result<Self, ::codec::Error> {
            match __codec_input_edqy
                .read_byte()
                .map_err(|e| {
                    e
                        .chain(
                            "Could not decode `RuntimeViewFunction`, failed to read variant byte",
                        )
                })?
            {
                _ => {
                    #[allow(clippy::redundant_closure_call)]
                    return (move || {
                        ::core::result::Result::Err(
                            <_ as ::core::convert::Into<
                                _,
                            >>::into(
                                "Could not decode `RuntimeViewFunction`, variant doesn't exist",
                            ),
                        )
                    })();
                }
            }
        }
    }
};
#[allow(deprecated)]
const _: () = {
    fn check_struct() {
        fn check_field<T: ::codec::DecodeWithMemTracking>() {}
    }
    #[automatically_derived]
    impl ::codec::DecodeWithMemTracking for RuntimeViewFunction {}
};
#[allow(non_upper_case_globals, deprecated, unused_attributes, unused_qualifications)]
const _: () = {
    impl ::scale_info::TypeInfo for RuntimeViewFunction {
        type Identity = Self;
        fn type_info() -> ::scale_info::Type {
            ::scale_info::Type::builder()
                .path(
                    ::scale_info::Path::new_with_replace(
                        "RuntimeViewFunction",
                        "polkadot_test",
                        &[],
                    ),
                )
                .type_params(::alloc::vec::Vec::new())
                .docs(&["Runtime query type."])
                .variant(::scale_info::build::Variants::new())
        }
    }
};
impl core::fmt::Debug for RuntimeViewFunction {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            _ => Ok(()),
        }
    }
}
const _: () = {
    impl self::sp_api_hidden_includes_construct_runtime::hidden_include::view_functions::DispatchViewFunction
    for RuntimeViewFunction {
        fn dispatch_view_function<
            O: self::sp_api_hidden_includes_construct_runtime::hidden_include::__private::codec::Output,
        >(
            id: &self::sp_api_hidden_includes_construct_runtime::hidden_include::view_functions::ViewFunctionId,
            input: &mut &[u8],
            output: &mut O,
        ) -> Result<
            (),
            self::sp_api_hidden_includes_construct_runtime::hidden_include::view_functions::ViewFunctionDispatchError,
        > {
            if id.prefix
                == <System as self::sp_api_hidden_includes_construct_runtime::hidden_include::view_functions::ViewFunctionIdPrefix>::prefix()
            {
                return <System as self::sp_api_hidden_includes_construct_runtime::hidden_include::view_functions::DispatchViewFunction>::dispatch_view_function(
                    id,
                    input,
                    output,
                );
            }
            if id.prefix
                == <Balances as self::sp_api_hidden_includes_construct_runtime::hidden_include::view_functions::ViewFunctionIdPrefix>::prefix()
            {
                return <Balances as self::sp_api_hidden_includes_construct_runtime::hidden_include::view_functions::DispatchViewFunction>::dispatch_view_function(
                    id,
                    input,
                    output,
                );
            }
            if id.prefix
                == <Parameters as self::sp_api_hidden_includes_construct_runtime::hidden_include::view_functions::ViewFunctionIdPrefix>::prefix()
            {
                return <Parameters as self::sp_api_hidden_includes_construct_runtime::hidden_include::view_functions::DispatchViewFunction>::dispatch_view_function(
                    id,
                    input,
                    output,
                );
            }
            Err(
                self::sp_api_hidden_includes_construct_runtime::hidden_include::view_functions::ViewFunctionDispatchError::NotFound(
                    id.clone(),
                ),
            )
        }
    }
    impl Runtime {
        /// Convenience function for view functions dispatching and execution from the runtime API.
        pub fn execute_view_function(
            id: self::sp_api_hidden_includes_construct_runtime::hidden_include::view_functions::ViewFunctionId,
            input: self::sp_api_hidden_includes_construct_runtime::hidden_include::__private::Vec<
                ::core::primitive::u8,
            >,
        ) -> Result<
            self::sp_api_hidden_includes_construct_runtime::hidden_include::__private::Vec<
                ::core::primitive::u8,
            >,
            self::sp_api_hidden_includes_construct_runtime::hidden_include::view_functions::ViewFunctionDispatchError,
        > {
            let mut output = ::alloc::vec::Vec::new();
            <RuntimeViewFunction as self::sp_api_hidden_includes_construct_runtime::hidden_include::view_functions::DispatchViewFunction>::dispatch_view_function(
                &id,
                &mut &input[..],
                &mut output,
            )?;
            Ok(output)
        }
    }
};
impl Runtime {
    #[allow(deprecated)]
    fn metadata_ir() -> self::sp_api_hidden_includes_construct_runtime::hidden_include::__private::metadata_ir::MetadataIR {
        let rt = Runtime;
        let ty = self::sp_api_hidden_includes_construct_runtime::hidden_include::__private::scale_info::meta_type::<
            <<Runtime as polkadot_sdk::frame_system::Config>::Block as self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_runtime::traits::Block>::Extrinsic,
        >();
        let address_ty = self::sp_api_hidden_includes_construct_runtime::hidden_include::__private::scale_info::meta_type::<
            <<<Runtime as polkadot_sdk::frame_system::Config>::Block as self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_runtime::traits::Block>::Extrinsic as self::sp_api_hidden_includes_construct_runtime::hidden_include::traits::SignedTransactionBuilder>::Address,
        >();
        let call_ty = self::sp_api_hidden_includes_construct_runtime::hidden_include::__private::scale_info::meta_type::<
            <<<Runtime as polkadot_sdk::frame_system::Config>::Block as self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_runtime::traits::Block>::Extrinsic as self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_runtime::traits::ExtrinsicCall>::Call,
        >();
        let signature_ty = self::sp_api_hidden_includes_construct_runtime::hidden_include::__private::scale_info::meta_type::<
            <<<Runtime as polkadot_sdk::frame_system::Config>::Block as self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_runtime::traits::Block>::Extrinsic as self::sp_api_hidden_includes_construct_runtime::hidden_include::traits::SignedTransactionBuilder>::Signature,
        >();
        let extra_ty = self::sp_api_hidden_includes_construct_runtime::hidden_include::__private::scale_info::meta_type::<
            <<<Runtime as polkadot_sdk::frame_system::Config>::Block as self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_runtime::traits::Block>::Extrinsic as self::sp_api_hidden_includes_construct_runtime::hidden_include::traits::SignedTransactionBuilder>::Extension,
        >();
        use self::sp_api_hidden_includes_construct_runtime::hidden_include::__private::metadata_ir::InternalImplRuntimeApis;
        self::sp_api_hidden_includes_construct_runtime::hidden_include::__private::metadata_ir::MetadataIR {
            pallets: <[_]>::into_vec(
                ::alloc::boxed::box_new([
                    self::sp_api_hidden_includes_construct_runtime::hidden_include::__private::metadata_ir::PalletMetadataIR {
                        name: "System",
                        index: 0u8,
                        storage: Some(
                            frame_system::Pallet::<Runtime>::storage_metadata(),
                        ),
                        calls: Some(frame_system::Pallet::<Runtime>::call_functions()),
                        view_functions: frame_system::Pallet::<
                            Runtime,
                        >::pallet_view_functions_metadata(),
                        event: Some(
                            frame_system::Event::<
                                Runtime,
                            >::event_metadata::<frame_system::Event<Runtime>>(),
                        ),
                        constants: frame_system::Pallet::<
                            Runtime,
                        >::pallet_constants_metadata(),
                        error: frame_system::Pallet::<Runtime>::error_metadata(),
                        docs: frame_system::Pallet::<
                            Runtime,
                        >::pallet_documentation_metadata(),
                        associated_types: frame_system::Pallet::<
                            Runtime,
                        >::pallet_associated_types_metadata(),
                        deprecation_info: frame_system::Pallet::<
                            Runtime,
                        >::deprecation_info(),
                    },
                    self::sp_api_hidden_includes_construct_runtime::hidden_include::__private::metadata_ir::PalletMetadataIR {
                        name: "Balances",
                        index: 2u8,
                        storage: Some(
                            pallet_balances::Pallet::<Runtime>::storage_metadata(),
                        ),
                        calls: Some(
                            pallet_balances::Pallet::<Runtime>::call_functions(),
                        ),
                        view_functions: pallet_balances::Pallet::<
                            Runtime,
                        >::pallet_view_functions_metadata(),
                        event: Some(
                            pallet_balances::Event::<
                                Runtime,
                            >::event_metadata::<pallet_balances::Event<Runtime>>(),
                        ),
                        constants: pallet_balances::Pallet::<
                            Runtime,
                        >::pallet_constants_metadata(),
                        error: pallet_balances::Pallet::<Runtime>::error_metadata(),
                        docs: pallet_balances::Pallet::<
                            Runtime,
                        >::pallet_documentation_metadata(),
                        associated_types: pallet_balances::Pallet::<
                            Runtime,
                        >::pallet_associated_types_metadata(),
                        deprecation_info: pallet_balances::Pallet::<
                            Runtime,
                        >::deprecation_info(),
                    },
                    self::sp_api_hidden_includes_construct_runtime::hidden_include::__private::metadata_ir::PalletMetadataIR {
                        name: "Parameters",
                        index: 3u8,
                        storage: Some(
                            pallet_parameters::Pallet::<Runtime>::storage_metadata(),
                        ),
                        calls: Some(
                            pallet_parameters::Pallet::<Runtime>::call_functions(),
                        ),
                        view_functions: pallet_parameters::Pallet::<
                            Runtime,
                        >::pallet_view_functions_metadata(),
                        event: Some(
                            pallet_parameters::Event::<
                                Runtime,
                            >::event_metadata::<pallet_parameters::Event<Runtime>>(),
                        ),
                        constants: pallet_parameters::Pallet::<
                            Runtime,
                        >::pallet_constants_metadata(),
                        error: pallet_parameters::Pallet::<Runtime>::error_metadata(),
                        docs: pallet_parameters::Pallet::<
                            Runtime,
                        >::pallet_documentation_metadata(),
                        associated_types: pallet_parameters::Pallet::<
                            Runtime,
                        >::pallet_associated_types_metadata(),
                        deprecation_info: pallet_parameters::Pallet::<
                            Runtime,
                        >::deprecation_info(),
                    },
                ]),
            ),
            extrinsic: self::sp_api_hidden_includes_construct_runtime::hidden_include::__private::metadata_ir::ExtrinsicMetadataIR {
                ty,
                versions: <<<Runtime as polkadot_sdk::frame_system::Config>::Block as self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_runtime::traits::Block>::Extrinsic as self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_runtime::traits::ExtrinsicMetadata>::VERSIONS
                    .into_iter()
                    .map(|ref_version| *ref_version)
                    .collect(),
                address_ty,
                call_ty,
                signature_ty,
                extra_ty,
                extensions: <<<<Runtime as polkadot_sdk::frame_system::Config>::Block as self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_runtime::traits::Block>::Extrinsic as self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_runtime::traits::ExtrinsicMetadata>::TransactionExtensions as self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_runtime::traits::TransactionExtension<
                    <Runtime as frame_system::Config>::RuntimeCall,
                >>::metadata()
                    .into_iter()
                    .map(|meta| self::sp_api_hidden_includes_construct_runtime::hidden_include::__private::metadata_ir::TransactionExtensionMetadataIR {
                        identifier: meta.identifier,
                        ty: meta.ty,
                        implicit: meta.implicit,
                    })
                    .collect(),
            },
            ty: self::sp_api_hidden_includes_construct_runtime::hidden_include::__private::scale_info::meta_type::<
                Runtime,
            >(),
            apis: (&rt).runtime_metadata(),
            outer_enums: self::sp_api_hidden_includes_construct_runtime::hidden_include::__private::metadata_ir::OuterEnumsIR {
                call_enum_ty: self::sp_api_hidden_includes_construct_runtime::hidden_include::__private::scale_info::meta_type::<
                    <Runtime as frame_system::Config>::RuntimeCall,
                >(),
                event_enum_ty: self::sp_api_hidden_includes_construct_runtime::hidden_include::__private::scale_info::meta_type::<
                    RuntimeEvent,
                >(),
                error_enum_ty: self::sp_api_hidden_includes_construct_runtime::hidden_include::__private::scale_info::meta_type::<
                    RuntimeError,
                >(),
            },
        }
    }
    pub fn metadata() -> self::sp_api_hidden_includes_construct_runtime::hidden_include::__private::metadata::RuntimeMetadataPrefixed {
        self::sp_api_hidden_includes_construct_runtime::hidden_include::__private::metadata_ir::into_v14(
            Runtime::metadata_ir(),
        )
    }
    pub fn metadata_at_version(
        version: u32,
    ) -> Option<
        self::sp_api_hidden_includes_construct_runtime::hidden_include::__private::OpaqueMetadata,
    > {
        self::sp_api_hidden_includes_construct_runtime::hidden_include::__private::metadata_ir::into_version(
                Runtime::metadata_ir(),
                version,
            )
            .map(|prefixed| {
                self::sp_api_hidden_includes_construct_runtime::hidden_include::__private::OpaqueMetadata::new(
                    prefixed.into(),
                )
            })
    }
    pub fn metadata_versions() -> self::sp_api_hidden_includes_construct_runtime::hidden_include::__private::Vec<
        u32,
    > {
        self::sp_api_hidden_includes_construct_runtime::hidden_include::__private::metadata_ir::supported_versions()
    }
}
pub type SystemConfig = frame_system::GenesisConfig<Runtime>;
pub type BalancesConfig = pallet_balances::GenesisConfig<Runtime>;
use self::sp_api_hidden_includes_construct_runtime::hidden_include::__private::serde as __genesis_config_serde_import__;
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
#[serde(crate = "__genesis_config_serde_import__")]
pub struct RuntimeGenesisConfig {
    pub system: SystemConfig,
    pub balances: BalancesConfig,
}
#[doc(hidden)]
#[allow(
    non_upper_case_globals,
    unused_attributes,
    unused_qualifications,
    clippy::absolute_paths,
)]
const _: () = {
    use __genesis_config_serde_import__ as _serde;
    #[automatically_derived]
    impl _serde::Serialize for RuntimeGenesisConfig {
        fn serialize<__S>(
            &self,
            __serializer: __S,
        ) -> _serde::__private228::Result<__S::Ok, __S::Error>
        where
            __S: _serde::Serializer,
        {
            let mut __serde_state = _serde::Serializer::serialize_struct(
                __serializer,
                "RuntimeGenesisConfig",
                false as usize + 1 + 1,
            )?;
            _serde::ser::SerializeStruct::serialize_field(
                &mut __serde_state,
                "system",
                &self.system,
            )?;
            _serde::ser::SerializeStruct::serialize_field(
                &mut __serde_state,
                "balances",
                &self.balances,
            )?;
            _serde::ser::SerializeStruct::end(__serde_state)
        }
    }
};
#[doc(hidden)]
#[allow(
    non_upper_case_globals,
    unused_attributes,
    unused_qualifications,
    clippy::absolute_paths,
)]
const _: () = {
    use __genesis_config_serde_import__ as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for RuntimeGenesisConfig {
        fn deserialize<__D>(
            __deserializer: __D,
        ) -> _serde::__private228::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            #[doc(hidden)]
            enum __Field {
                __field0,
                __field1,
            }
            #[doc(hidden)]
            struct __FieldVisitor;
            #[automatically_derived]
            impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                type Value = __Field;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private228::Formatter,
                ) -> _serde::__private228::fmt::Result {
                    _serde::__private228::Formatter::write_str(
                        __formatter,
                        "field identifier",
                    )
                }
                fn visit_u64<__E>(
                    self,
                    __value: u64,
                ) -> _serde::__private228::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        0u64 => _serde::__private228::Ok(__Field::__field0),
                        1u64 => _serde::__private228::Ok(__Field::__field1),
                        _ => {
                            _serde::__private228::Err(
                                _serde::de::Error::invalid_value(
                                    _serde::de::Unexpected::Unsigned(__value),
                                    &"field index 0 <= i < 2",
                                ),
                            )
                        }
                    }
                }
                fn visit_str<__E>(
                    self,
                    __value: &str,
                ) -> _serde::__private228::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        "system" => _serde::__private228::Ok(__Field::__field0),
                        "balances" => _serde::__private228::Ok(__Field::__field1),
                        _ => {
                            _serde::__private228::Err(
                                _serde::de::Error::unknown_field(__value, FIELDS),
                            )
                        }
                    }
                }
                fn visit_bytes<__E>(
                    self,
                    __value: &[u8],
                ) -> _serde::__private228::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        b"system" => _serde::__private228::Ok(__Field::__field0),
                        b"balances" => _serde::__private228::Ok(__Field::__field1),
                        _ => {
                            let __value = &_serde::__private228::from_utf8_lossy(
                                __value,
                            );
                            _serde::__private228::Err(
                                _serde::de::Error::unknown_field(__value, FIELDS),
                            )
                        }
                    }
                }
            }
            #[automatically_derived]
            impl<'de> _serde::Deserialize<'de> for __Field {
                #[inline]
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private228::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    _serde::Deserializer::deserialize_identifier(
                        __deserializer,
                        __FieldVisitor,
                    )
                }
            }
            #[doc(hidden)]
            struct __Visitor<'de> {
                marker: _serde::__private228::PhantomData<RuntimeGenesisConfig>,
                lifetime: _serde::__private228::PhantomData<&'de ()>,
            }
            #[automatically_derived]
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = RuntimeGenesisConfig;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private228::Formatter,
                ) -> _serde::__private228::fmt::Result {
                    _serde::__private228::Formatter::write_str(
                        __formatter,
                        "struct RuntimeGenesisConfig",
                    )
                }
                #[inline]
                fn visit_seq<__A>(
                    self,
                    mut __seq: __A,
                ) -> _serde::__private228::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::SeqAccess<'de>,
                {
                    let __field0 = match _serde::de::SeqAccess::next_element::<
                        SystemConfig,
                    >(&mut __seq)? {
                        _serde::__private228::Some(__value) => __value,
                        _serde::__private228::None => {
                            return _serde::__private228::Err(
                                _serde::de::Error::invalid_length(
                                    0usize,
                                    &"struct RuntimeGenesisConfig with 2 elements",
                                ),
                            );
                        }
                    };
                    let __field1 = match _serde::de::SeqAccess::next_element::<
                        BalancesConfig,
                    >(&mut __seq)? {
                        _serde::__private228::Some(__value) => __value,
                        _serde::__private228::None => {
                            return _serde::__private228::Err(
                                _serde::de::Error::invalid_length(
                                    1usize,
                                    &"struct RuntimeGenesisConfig with 2 elements",
                                ),
                            );
                        }
                    };
                    _serde::__private228::Ok(RuntimeGenesisConfig {
                        system: __field0,
                        balances: __field1,
                    })
                }
                #[inline]
                fn visit_map<__A>(
                    self,
                    mut __map: __A,
                ) -> _serde::__private228::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::MapAccess<'de>,
                {
                    let mut __field0: _serde::__private228::Option<SystemConfig> = _serde::__private228::None;
                    let mut __field1: _serde::__private228::Option<BalancesConfig> = _serde::__private228::None;
                    while let _serde::__private228::Some(__key) = _serde::de::MapAccess::next_key::<
                        __Field,
                    >(&mut __map)? {
                        match __key {
                            __Field::__field0 => {
                                if _serde::__private228::Option::is_some(&__field0) {
                                    return _serde::__private228::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("system"),
                                    );
                                }
                                __field0 = _serde::__private228::Some(
                                    _serde::de::MapAccess::next_value::<
                                        SystemConfig,
                                    >(&mut __map)?,
                                );
                            }
                            __Field::__field1 => {
                                if _serde::__private228::Option::is_some(&__field1) {
                                    return _serde::__private228::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "balances",
                                        ),
                                    );
                                }
                                __field1 = _serde::__private228::Some(
                                    _serde::de::MapAccess::next_value::<
                                        BalancesConfig,
                                    >(&mut __map)?,
                                );
                            }
                        }
                    }
                    let __field0 = match __field0 {
                        _serde::__private228::Some(__field0) => __field0,
                        _serde::__private228::None => {
                            _serde::__private228::de::missing_field("system")?
                        }
                    };
                    let __field1 = match __field1 {
                        _serde::__private228::Some(__field1) => __field1,
                        _serde::__private228::None => {
                            _serde::__private228::de::missing_field("balances")?
                        }
                    };
                    _serde::__private228::Ok(RuntimeGenesisConfig {
                        system: __field0,
                        balances: __field1,
                    })
                }
            }
            #[doc(hidden)]
            const FIELDS: &'static [&'static str] = &["system", "balances"];
            _serde::Deserializer::deserialize_struct(
                __deserializer,
                "RuntimeGenesisConfig",
                FIELDS,
                __Visitor {
                    marker: _serde::__private228::PhantomData::<RuntimeGenesisConfig>,
                    lifetime: _serde::__private228::PhantomData,
                },
            )
        }
    }
};
#[automatically_derived]
impl ::core::default::Default for RuntimeGenesisConfig {
    #[inline]
    fn default() -> RuntimeGenesisConfig {
        RuntimeGenesisConfig {
            system: ::core::default::Default::default(),
            balances: ::core::default::Default::default(),
        }
    }
}
impl self::sp_api_hidden_includes_construct_runtime::hidden_include::traits::BuildGenesisConfig
for RuntimeGenesisConfig {
    fn build(&self) {
        <SystemConfig as self::sp_api_hidden_includes_construct_runtime::hidden_include::traits::BuildGenesisConfig>::build(
            &self.system,
        );
        <BalancesConfig as self::sp_api_hidden_includes_construct_runtime::hidden_include::traits::BuildGenesisConfig>::build(
            &self.balances,
        );
        <AllPalletsWithSystem as self::sp_api_hidden_includes_construct_runtime::hidden_include::traits::OnGenesis>::on_genesis();
    }
}
trait InherentDataExt {
    fn create_extrinsics(
        &self,
    ) -> self::sp_api_hidden_includes_construct_runtime::hidden_include::__private::Vec<
        <<Runtime as polkadot_sdk::frame_system::Config>::Block as self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_runtime::traits::Block>::Extrinsic,
    >;
    fn check_extrinsics(
        &self,
        block: &<Runtime as polkadot_sdk::frame_system::Config>::Block,
    ) -> self::sp_api_hidden_includes_construct_runtime::hidden_include::inherent::CheckInherentsResult;
}
impl InherentDataExt
for self::sp_api_hidden_includes_construct_runtime::hidden_include::inherent::InherentData {
    fn create_extrinsics(
        &self,
    ) -> self::sp_api_hidden_includes_construct_runtime::hidden_include::__private::Vec<
        <<Runtime as polkadot_sdk::frame_system::Config>::Block as self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_runtime::traits::Block>::Extrinsic,
    > {
        use self::sp_api_hidden_includes_construct_runtime::hidden_include::{
            inherent::ProvideInherent, traits::InherentBuilder,
        };
        let mut inherents = self::sp_api_hidden_includes_construct_runtime::hidden_include::__private::Vec::new();
        inherents
    }
    fn check_extrinsics(
        &self,
        block: &<Runtime as polkadot_sdk::frame_system::Config>::Block,
    ) -> self::sp_api_hidden_includes_construct_runtime::hidden_include::inherent::CheckInherentsResult {
        use self::sp_api_hidden_includes_construct_runtime::hidden_include::inherent::{
            ProvideInherent, IsFatalError,
        };
        use self::sp_api_hidden_includes_construct_runtime::hidden_include::traits::IsSubType;
        use self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_runtime::traits::{
            Block as _, ExtrinsicCall,
        };
        use self::sp_api_hidden_includes_construct_runtime::hidden_include::__private::{
            sp_inherents::Error, log,
        };
        let mut result = self::sp_api_hidden_includes_construct_runtime::hidden_include::inherent::CheckInherentsResult::new();
        fn handle_put_error_result(res: Result<(), Error>) {
            const LOG_TARGET: &str = "runtime::inherent";
            match res {
                Ok(()) => {}
                Err(Error::InherentDataExists(id)) => {
                    let lvl = ::log::Level::Debug;
                    if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                        ::log::__private_api::log(
                            { ::log::__private_api::GlobalLogger },
                            format_args!(
                                "Some error already reported for inherent {0:?}, new non fatal error is ignored",
                                id,
                            ),
                            lvl,
                            &(LOG_TARGET, "polkadot_test", ::log::__private_api::loc()),
                            (),
                        );
                    }
                }
                Err(Error::FatalErrorReported) => {
                    let lvl = ::log::Level::Error;
                    if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                        ::log::__private_api::log(
                            { ::log::__private_api::GlobalLogger },
                            format_args!(
                                "Fatal error already reported, unexpected considering there is only one fatal error",
                            ),
                            lvl,
                            &(LOG_TARGET, "polkadot_test", ::log::__private_api::loc()),
                            (),
                        );
                    }
                }
                Err(_) => {
                    let lvl = ::log::Level::Error;
                    if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                        ::log::__private_api::log(
                            { ::log::__private_api::GlobalLogger },
                            format_args!("Unexpected error from `put_error` operation"),
                            lvl,
                            &(LOG_TARGET, "polkadot_test", ::log::__private_api::loc()),
                            (),
                        );
                    }
                }
            }
        }
        let mut pallet_has_inherent = [false; 0usize];
        for xt in block.extrinsics() {
            if !(self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_runtime::traits::ExtrinsicLike::is_bare(
                xt,
            )) {
                break;
            }
            let mut is_inherent = false;
            let call = ExtrinsicCall::call(xt);
            if !is_inherent {
                break;
            }
        }
        result
    }
}
impl self::sp_api_hidden_includes_construct_runtime::hidden_include::traits::IsInherent<
    <<Runtime as polkadot_sdk::frame_system::Config>::Block as self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_runtime::traits::Block>::Extrinsic,
> for Runtime {
    fn is_inherent(
        ext: &<<Runtime as polkadot_sdk::frame_system::Config>::Block as self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_runtime::traits::Block>::Extrinsic,
    ) -> bool {
        use self::sp_api_hidden_includes_construct_runtime::hidden_include::inherent::ProvideInherent;
        use self::sp_api_hidden_includes_construct_runtime::hidden_include::traits::IsSubType;
        use self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_runtime::traits::ExtrinsicCall;
        let is_bare = self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_runtime::traits::ExtrinsicLike::is_bare(
            ext,
        );
        if !is_bare {
            return false;
        }
        let call = ExtrinsicCall::call(ext);
        false
    }
}
impl self::sp_api_hidden_includes_construct_runtime::hidden_include::unsigned::ValidateUnsigned
for Runtime {
    type Call = RuntimeCall;
    fn pre_dispatch(
        call: &Self::Call,
    ) -> Result<
        (),
        self::sp_api_hidden_includes_construct_runtime::hidden_include::unsigned::TransactionValidityError,
    > {
        #[allow(unreachable_patterns)]
        match call {
            RuntimeCall::System(inner_call) => System::pre_dispatch(inner_call),
            _ => Ok(()),
        }
    }
    fn validate_unsigned(
        #[allow(unused_variables)]
        source: self::sp_api_hidden_includes_construct_runtime::hidden_include::unsigned::TransactionSource,
        call: &Self::Call,
    ) -> self::sp_api_hidden_includes_construct_runtime::hidden_include::unsigned::TransactionValidity {
        #[allow(unreachable_patterns)]
        match call {
            RuntimeCall::System(inner_call) => {
                System::validate_unsigned(source, inner_call)
            }
            _ => {
                self::sp_api_hidden_includes_construct_runtime::hidden_include::unsigned::UnknownTransaction::NoUnsignedValidator
                    .into()
            }
        }
    }
}
/// A reason for placing a freeze on funds.
pub enum RuntimeFreezeReason {}
#[automatically_derived]
impl ::core::marker::Copy for RuntimeFreezeReason {}
#[automatically_derived]
impl ::core::clone::Clone for RuntimeFreezeReason {
    #[inline]
    fn clone(&self) -> RuntimeFreezeReason {
        *self
    }
}
#[automatically_derived]
impl ::core::cmp::Eq for RuntimeFreezeReason {
    #[inline]
    #[doc(hidden)]
    #[coverage(off)]
    fn assert_receiver_is_total_eq(&self) -> () {}
}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for RuntimeFreezeReason {}
#[automatically_derived]
impl ::core::cmp::PartialEq for RuntimeFreezeReason {
    #[inline]
    fn eq(&self, other: &RuntimeFreezeReason) -> bool {
        match *self {}
    }
}
#[allow(deprecated)]
const _: () = {
    #[automatically_derived]
    impl ::codec::Encode for RuntimeFreezeReason {}
    #[automatically_derived]
    impl ::codec::EncodeLike for RuntimeFreezeReason {}
};
#[allow(deprecated)]
const _: () = {
    #[automatically_derived]
    impl ::codec::Decode for RuntimeFreezeReason {
        fn decode<__CodecInputEdqy: ::codec::Input>(
            __codec_input_edqy: &mut __CodecInputEdqy,
        ) -> ::core::result::Result<Self, ::codec::Error> {
            match __codec_input_edqy
                .read_byte()
                .map_err(|e| {
                    e
                        .chain(
                            "Could not decode `RuntimeFreezeReason`, failed to read variant byte",
                        )
                })?
            {
                _ => {
                    #[allow(clippy::redundant_closure_call)]
                    return (move || {
                        ::core::result::Result::Err(
                            <_ as ::core::convert::Into<
                                _,
                            >>::into(
                                "Could not decode `RuntimeFreezeReason`, variant doesn't exist",
                            ),
                        )
                    })();
                }
            }
        }
    }
};
#[allow(deprecated)]
const _: () = {
    fn check_struct() {
        fn check_field<T: ::codec::DecodeWithMemTracking>() {}
    }
    #[automatically_derived]
    impl ::codec::DecodeWithMemTracking for RuntimeFreezeReason {}
};
const _: () = {
    #[automatically_derived]
    impl ::codec::MaxEncodedLen for RuntimeFreezeReason {
        fn max_encoded_len() -> ::core::primitive::usize {
            0_usize.saturating_add(1)
        }
    }
};
#[allow(non_upper_case_globals, deprecated, unused_attributes, unused_qualifications)]
const _: () = {
    impl ::scale_info::TypeInfo for RuntimeFreezeReason {
        type Identity = Self;
        fn type_info() -> ::scale_info::Type {
            ::scale_info::Type::builder()
                .path(
                    ::scale_info::Path::new_with_replace(
                        "RuntimeFreezeReason",
                        "polkadot_test",
                        &[],
                    ),
                )
                .type_params(::alloc::vec::Vec::new())
                .docs(&["A reason for placing a freeze on funds."])
                .variant(::scale_info::build::Variants::new())
        }
    }
};
impl core::fmt::Debug for RuntimeFreezeReason {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            _ => Ok(()),
        }
    }
}
impl self::sp_api_hidden_includes_construct_runtime::hidden_include::traits::VariantCount
for RuntimeFreezeReason {
    const VARIANT_COUNT: u32 = 0;
}
/// A reason for placing a hold on funds.
pub enum RuntimeHoldReason {}
#[automatically_derived]
impl ::core::marker::Copy for RuntimeHoldReason {}
#[automatically_derived]
impl ::core::clone::Clone for RuntimeHoldReason {
    #[inline]
    fn clone(&self) -> RuntimeHoldReason {
        *self
    }
}
#[automatically_derived]
impl ::core::cmp::Eq for RuntimeHoldReason {
    #[inline]
    #[doc(hidden)]
    #[coverage(off)]
    fn assert_receiver_is_total_eq(&self) -> () {}
}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for RuntimeHoldReason {}
#[automatically_derived]
impl ::core::cmp::PartialEq for RuntimeHoldReason {
    #[inline]
    fn eq(&self, other: &RuntimeHoldReason) -> bool {
        match *self {}
    }
}
#[allow(deprecated)]
const _: () = {
    #[automatically_derived]
    impl ::codec::Encode for RuntimeHoldReason {}
    #[automatically_derived]
    impl ::codec::EncodeLike for RuntimeHoldReason {}
};
#[allow(deprecated)]
const _: () = {
    #[automatically_derived]
    impl ::codec::Decode for RuntimeHoldReason {
        fn decode<__CodecInputEdqy: ::codec::Input>(
            __codec_input_edqy: &mut __CodecInputEdqy,
        ) -> ::core::result::Result<Self, ::codec::Error> {
            match __codec_input_edqy
                .read_byte()
                .map_err(|e| {
                    e
                        .chain(
                            "Could not decode `RuntimeHoldReason`, failed to read variant byte",
                        )
                })?
            {
                _ => {
                    #[allow(clippy::redundant_closure_call)]
                    return (move || {
                        ::core::result::Result::Err(
                            <_ as ::core::convert::Into<
                                _,
                            >>::into(
                                "Could not decode `RuntimeHoldReason`, variant doesn't exist",
                            ),
                        )
                    })();
                }
            }
        }
    }
};
#[allow(deprecated)]
const _: () = {
    fn check_struct() {
        fn check_field<T: ::codec::DecodeWithMemTracking>() {}
    }
    #[automatically_derived]
    impl ::codec::DecodeWithMemTracking for RuntimeHoldReason {}
};
const _: () = {
    #[automatically_derived]
    impl ::codec::MaxEncodedLen for RuntimeHoldReason {
        fn max_encoded_len() -> ::core::primitive::usize {
            0_usize.saturating_add(1)
        }
    }
};
#[allow(non_upper_case_globals, deprecated, unused_attributes, unused_qualifications)]
const _: () = {
    impl ::scale_info::TypeInfo for RuntimeHoldReason {
        type Identity = Self;
        fn type_info() -> ::scale_info::Type {
            ::scale_info::Type::builder()
                .path(
                    ::scale_info::Path::new_with_replace(
                        "RuntimeHoldReason",
                        "polkadot_test",
                        &[],
                    ),
                )
                .type_params(::alloc::vec::Vec::new())
                .docs(&["A reason for placing a hold on funds."])
                .variant(::scale_info::build::Variants::new())
        }
    }
};
impl core::fmt::Debug for RuntimeHoldReason {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            _ => Ok(()),
        }
    }
}
impl self::sp_api_hidden_includes_construct_runtime::hidden_include::traits::VariantCount
for RuntimeHoldReason {
    const VARIANT_COUNT: u32 = 0;
}
/// An identifier for each lock placed on funds.
pub enum RuntimeLockId {}
#[automatically_derived]
impl ::core::marker::Copy for RuntimeLockId {}
#[automatically_derived]
impl ::core::clone::Clone for RuntimeLockId {
    #[inline]
    fn clone(&self) -> RuntimeLockId {
        *self
    }
}
#[automatically_derived]
impl ::core::cmp::Eq for RuntimeLockId {
    #[inline]
    #[doc(hidden)]
    #[coverage(off)]
    fn assert_receiver_is_total_eq(&self) -> () {}
}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for RuntimeLockId {}
#[automatically_derived]
impl ::core::cmp::PartialEq for RuntimeLockId {
    #[inline]
    fn eq(&self, other: &RuntimeLockId) -> bool {
        match *self {}
    }
}
#[allow(deprecated)]
const _: () = {
    #[automatically_derived]
    impl ::codec::Encode for RuntimeLockId {}
    #[automatically_derived]
    impl ::codec::EncodeLike for RuntimeLockId {}
};
#[allow(deprecated)]
const _: () = {
    #[automatically_derived]
    impl ::codec::Decode for RuntimeLockId {
        fn decode<__CodecInputEdqy: ::codec::Input>(
            __codec_input_edqy: &mut __CodecInputEdqy,
        ) -> ::core::result::Result<Self, ::codec::Error> {
            match __codec_input_edqy
                .read_byte()
                .map_err(|e| {
                    e
                        .chain(
                            "Could not decode `RuntimeLockId`, failed to read variant byte",
                        )
                })?
            {
                _ => {
                    #[allow(clippy::redundant_closure_call)]
                    return (move || {
                        ::core::result::Result::Err(
                            <_ as ::core::convert::Into<
                                _,
                            >>::into(
                                "Could not decode `RuntimeLockId`, variant doesn't exist",
                            ),
                        )
                    })();
                }
            }
        }
    }
};
#[allow(deprecated)]
const _: () = {
    fn check_struct() {
        fn check_field<T: ::codec::DecodeWithMemTracking>() {}
    }
    #[automatically_derived]
    impl ::codec::DecodeWithMemTracking for RuntimeLockId {}
};
const _: () = {
    #[automatically_derived]
    impl ::codec::MaxEncodedLen for RuntimeLockId {
        fn max_encoded_len() -> ::core::primitive::usize {
            0_usize.saturating_add(1)
        }
    }
};
#[allow(non_upper_case_globals, deprecated, unused_attributes, unused_qualifications)]
const _: () = {
    impl ::scale_info::TypeInfo for RuntimeLockId {
        type Identity = Self;
        fn type_info() -> ::scale_info::Type {
            ::scale_info::Type::builder()
                .path(
                    ::scale_info::Path::new_with_replace(
                        "RuntimeLockId",
                        "polkadot_test",
                        &[],
                    ),
                )
                .type_params(::alloc::vec::Vec::new())
                .docs(&["An identifier for each lock placed on funds."])
                .variant(::scale_info::build::Variants::new())
        }
    }
};
impl core::fmt::Debug for RuntimeLockId {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            _ => Ok(()),
        }
    }
}
/// A reason for slashing funds.
pub enum RuntimeSlashReason {}
#[automatically_derived]
impl ::core::marker::Copy for RuntimeSlashReason {}
#[automatically_derived]
impl ::core::clone::Clone for RuntimeSlashReason {
    #[inline]
    fn clone(&self) -> RuntimeSlashReason {
        *self
    }
}
#[automatically_derived]
impl ::core::cmp::Eq for RuntimeSlashReason {
    #[inline]
    #[doc(hidden)]
    #[coverage(off)]
    fn assert_receiver_is_total_eq(&self) -> () {}
}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for RuntimeSlashReason {}
#[automatically_derived]
impl ::core::cmp::PartialEq for RuntimeSlashReason {
    #[inline]
    fn eq(&self, other: &RuntimeSlashReason) -> bool {
        match *self {}
    }
}
#[allow(deprecated)]
const _: () = {
    #[automatically_derived]
    impl ::codec::Encode for RuntimeSlashReason {}
    #[automatically_derived]
    impl ::codec::EncodeLike for RuntimeSlashReason {}
};
#[allow(deprecated)]
const _: () = {
    #[automatically_derived]
    impl ::codec::Decode for RuntimeSlashReason {
        fn decode<__CodecInputEdqy: ::codec::Input>(
            __codec_input_edqy: &mut __CodecInputEdqy,
        ) -> ::core::result::Result<Self, ::codec::Error> {
            match __codec_input_edqy
                .read_byte()
                .map_err(|e| {
                    e
                        .chain(
                            "Could not decode `RuntimeSlashReason`, failed to read variant byte",
                        )
                })?
            {
                _ => {
                    #[allow(clippy::redundant_closure_call)]
                    return (move || {
                        ::core::result::Result::Err(
                            <_ as ::core::convert::Into<
                                _,
                            >>::into(
                                "Could not decode `RuntimeSlashReason`, variant doesn't exist",
                            ),
                        )
                    })();
                }
            }
        }
    }
};
#[allow(deprecated)]
const _: () = {
    fn check_struct() {
        fn check_field<T: ::codec::DecodeWithMemTracking>() {}
    }
    #[automatically_derived]
    impl ::codec::DecodeWithMemTracking for RuntimeSlashReason {}
};
const _: () = {
    #[automatically_derived]
    impl ::codec::MaxEncodedLen for RuntimeSlashReason {
        fn max_encoded_len() -> ::core::primitive::usize {
            0_usize.saturating_add(1)
        }
    }
};
#[allow(non_upper_case_globals, deprecated, unused_attributes, unused_qualifications)]
const _: () = {
    impl ::scale_info::TypeInfo for RuntimeSlashReason {
        type Identity = Self;
        fn type_info() -> ::scale_info::Type {
            ::scale_info::Type::builder()
                .path(
                    ::scale_info::Path::new_with_replace(
                        "RuntimeSlashReason",
                        "polkadot_test",
                        &[],
                    ),
                )
                .type_params(::alloc::vec::Vec::new())
                .docs(&["A reason for slashing funds."])
                .variant(::scale_info::build::Variants::new())
        }
    }
};
impl core::fmt::Debug for RuntimeSlashReason {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            _ => Ok(()),
        }
    }
}
#[allow(deprecated)]
const _: () = if !(<frame_system::Error<
    Runtime,
> as ::frame_support::traits::PalletError>::MAX_ENCODED_SIZE
    <= ::frame_support::MAX_MODULE_ERROR_ENCODED_SIZE)
{
    {
        ::core::panicking::panic_fmt(
            format_args!(
                "The maximum encoded size of the error type in the `System` pallet exceeds `MAX_MODULE_ERROR_ENCODED_SIZE`",
            ),
        );
    }
};
#[allow(deprecated)]
const _: () = if !(<pallet_balances::Error<
    Runtime,
> as ::frame_support::traits::PalletError>::MAX_ENCODED_SIZE
    <= ::frame_support::MAX_MODULE_ERROR_ENCODED_SIZE)
{
    {
        ::core::panicking::panic_fmt(
            format_args!(
                "The maximum encoded size of the error type in the `Balances` pallet exceeds `MAX_MODULE_ERROR_ENCODED_SIZE`",
            ),
        );
    }
};
impl frame_system::Config for Runtime {
    type Block = Block;
    type Version = Version;
    type AccountData = pallet_balances::AccountData<
        <Runtime as pallet_balances::Config>::Balance,
    >;
    type Nonce = <frame_system::config_preludes::SolochainDefaultConfig as frame_system::DefaultConfig>::Nonce;
    type Hash = <frame_system::config_preludes::SolochainDefaultConfig as frame_system::DefaultConfig>::Hash;
    type Hashing = <frame_system::config_preludes::SolochainDefaultConfig as frame_system::DefaultConfig>::Hashing;
    type AccountId = <frame_system::config_preludes::SolochainDefaultConfig as frame_system::DefaultConfig>::AccountId;
    type Lookup = <frame_system::config_preludes::SolochainDefaultConfig as frame_system::DefaultConfig>::Lookup;
    type MaxConsumers = <frame_system::config_preludes::SolochainDefaultConfig as frame_system::DefaultConfig>::MaxConsumers;
    type OnNewAccount = <frame_system::config_preludes::SolochainDefaultConfig as frame_system::DefaultConfig>::OnNewAccount;
    type OnKilledAccount = <frame_system::config_preludes::SolochainDefaultConfig as frame_system::DefaultConfig>::OnKilledAccount;
    type SystemWeightInfo = <frame_system::config_preludes::SolochainDefaultConfig as frame_system::DefaultConfig>::SystemWeightInfo;
    type ExtensionsWeightInfo = <frame_system::config_preludes::SolochainDefaultConfig as frame_system::DefaultConfig>::ExtensionsWeightInfo;
    type SS58Prefix = <frame_system::config_preludes::SolochainDefaultConfig as frame_system::DefaultConfig>::SS58Prefix;
    type BlockWeights = <frame_system::config_preludes::SolochainDefaultConfig as frame_system::DefaultConfig>::BlockWeights;
    type BlockLength = <frame_system::config_preludes::SolochainDefaultConfig as frame_system::DefaultConfig>::BlockLength;
    type DbWeight = <frame_system::config_preludes::SolochainDefaultConfig as frame_system::DefaultConfig>::DbWeight;
    type RuntimeEvent = RuntimeEvent;
    type RuntimeOrigin = RuntimeOrigin;
    type RuntimeCall = RuntimeCall;
    type RuntimeTask = RuntimeTask;
    type PalletInfo = PalletInfo;
    type BaseCallFilter = <frame_system::config_preludes::SolochainDefaultConfig as frame_system::DefaultConfig>::BaseCallFilter;
    type BlockHashCount = <frame_system::config_preludes::SolochainDefaultConfig as frame_system::DefaultConfig>::BlockHashCount;
    type OnSetCode = <frame_system::config_preludes::SolochainDefaultConfig as frame_system::DefaultConfig>::OnSetCode;
    type SingleBlockMigrations = <frame_system::config_preludes::SolochainDefaultConfig as frame_system::DefaultConfig>::SingleBlockMigrations;
    type MultiBlockMigrator = <frame_system::config_preludes::SolochainDefaultConfig as frame_system::DefaultConfig>::MultiBlockMigrator;
    type PreInherents = <frame_system::config_preludes::SolochainDefaultConfig as frame_system::DefaultConfig>::PreInherents;
    type PostInherents = <frame_system::config_preludes::SolochainDefaultConfig as frame_system::DefaultConfig>::PostInherents;
    type PostTransactions = <frame_system::config_preludes::SolochainDefaultConfig as frame_system::DefaultConfig>::PostTransactions;
}
impl pallet_balances::Config for Runtime {
    type AccountStore = System;
    type RuntimeHoldReason = ();
    type RuntimeEvent = RuntimeEvent;
    type RuntimeFreezeReason = RuntimeFreezeReason;
    type Balance = <pallet_balances::config_preludes::TestDefaultConfig as pallet_balances::DefaultConfig>::Balance;
    type ExistentialDeposit = <pallet_balances::config_preludes::TestDefaultConfig as pallet_balances::DefaultConfig>::ExistentialDeposit;
    type ReserveIdentifier = <pallet_balances::config_preludes::TestDefaultConfig as pallet_balances::DefaultConfig>::ReserveIdentifier;
    type FreezeIdentifier = <pallet_balances::config_preludes::TestDefaultConfig as pallet_balances::DefaultConfig>::FreezeIdentifier;
    type DustRemoval = <pallet_balances::config_preludes::TestDefaultConfig as pallet_balances::DefaultConfig>::DustRemoval;
    type MaxLocks = <pallet_balances::config_preludes::TestDefaultConfig as pallet_balances::DefaultConfig>::MaxLocks;
    type MaxReserves = <pallet_balances::config_preludes::TestDefaultConfig as pallet_balances::DefaultConfig>::MaxReserves;
    type MaxFreezes = <pallet_balances::config_preludes::TestDefaultConfig as pallet_balances::DefaultConfig>::MaxFreezes;
    type WeightInfo = <pallet_balances::config_preludes::TestDefaultConfig as pallet_balances::DefaultConfig>::WeightInfo;
    type DoneSlashHandler = <pallet_balances::config_preludes::TestDefaultConfig as pallet_balances::DefaultConfig>::DoneSlashHandler;
}
impl pallet_parameters::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type RuntimeParameters = RuntimeParameters;
    type AdminOrigin = DynamicParameterOrigin;
    type WeightInfo = ();
}
/// Defines what origin can modify which dynamic parameters.
pub struct DynamicParameterOrigin;
impl EnsureOriginWithArg<RuntimeOrigin, RuntimeParametersKey>
for DynamicParameterOrigin {
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
pub mod dynamic_params {
    use super::*;
    pub mod inflationAV {
        use super::*;
        #[doc(hidden)]
        pub enum Parameters {
            /// Minimum inflation rate used to calculate era payouts.
            #[codec(index = 0)]
            MinInflation(MinInflation, Option<u32>),
        }
        #[automatically_derived]
        impl ::core::clone::Clone for Parameters {
            #[inline]
            fn clone(&self) -> Parameters {
                match self {
                    Parameters::MinInflation(__self_0, __self_1) => {
                        Parameters::MinInflation(
                            ::core::clone::Clone::clone(__self_0),
                            ::core::clone::Clone::clone(__self_1),
                        )
                    }
                }
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for Parameters {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for Parameters {
            #[inline]
            fn eq(&self, other: &Parameters) -> bool {
                match (self, other) {
                    (
                        Parameters::MinInflation(__self_0, __self_1),
                        Parameters::MinInflation(__arg1_0, __arg1_1),
                    ) => __self_0 == __arg1_0 && __self_1 == __arg1_1,
                }
            }
        }
        #[automatically_derived]
        impl ::core::cmp::Eq for Parameters {
            #[inline]
            #[doc(hidden)]
            #[coverage(off)]
            fn assert_receiver_is_total_eq(&self) -> () {
                let _: ::core::cmp::AssertParamIsEq<MinInflation>;
                let _: ::core::cmp::AssertParamIsEq<Option<u32>>;
            }
        }
        #[allow(deprecated)]
        const _: () = {
            #[automatically_derived]
            impl ::codec::Encode for Parameters {
                fn size_hint(&self) -> usize {
                    1_usize
                        + match *self {
                            Parameters::MinInflation(ref aa, ref ba) => {
                                0_usize
                                    .saturating_add(::codec::Encode::size_hint(aa))
                                    .saturating_add(::codec::Encode::size_hint(ba))
                            }
                            _ => 0_usize,
                        }
                }
                fn encode_to<
                    __CodecOutputEdqy: ::codec::Output + ?::core::marker::Sized,
                >(&self, __codec_dest_edqy: &mut __CodecOutputEdqy) {
                    #[automatically_derived]
                    const _: () = {
                        #[allow(clippy::unnecessary_cast)]
                        #[allow(clippy::cast_possible_truncation)]
                        const indices: [(usize, &'static str); 1usize] = [
                            ((0usize) as ::core::primitive::usize, "MinInflation"),
                        ];
                        const fn search_for_invalid_index(
                            array: &[(usize, &'static str); 1usize],
                        ) -> (bool, usize) {
                            let mut i = 0;
                            while i < 1usize {
                                if array[i].0 > 255 {
                                    return (true, i);
                                }
                                i += 1;
                            }
                            (false, 0)
                        }
                        const INVALID_INDEX: (bool, usize) = search_for_invalid_index(
                            &indices,
                        );
                        if INVALID_INDEX.0 {
                            let msg = ::const_format::pmr::__AssertStr {
                                x: {
                                    use ::const_format::__cf_osRcTFl4A;
                                    ({
                                        #[doc(hidden)]
                                        #[allow(unused_mut, non_snake_case)]
                                        const CONCATP_NHPMWYD3NJA: &[__cf_osRcTFl4A::pmr::PArgument] = {
                                            let fmt = __cf_osRcTFl4A::pmr::FormattingFlags::NEW;
                                            &[
                                                __cf_osRcTFl4A::pmr::PConvWrapper("Found variant `")
                                                    .to_pargument_display(fmt),
                                                __cf_osRcTFl4A::pmr::PConvWrapper(
                                                        indices[INVALID_INDEX.1].1,
                                                    )
                                                    .to_pargument_display(fmt),
                                                __cf_osRcTFl4A::pmr::PConvWrapper("` with invalid index: `")
                                                    .to_pargument_display(fmt),
                                                __cf_osRcTFl4A::pmr::PConvWrapper(
                                                        indices[INVALID_INDEX.1].0,
                                                    )
                                                    .to_pargument_display(fmt),
                                                __cf_osRcTFl4A::pmr::PConvWrapper(
                                                        "`. Max supported index is 255.",
                                                    )
                                                    .to_pargument_display(fmt),
                                            ]
                                        };
                                        {
                                            #[doc(hidden)]
                                            const ARR_LEN: usize = ::const_format::pmr::PArgument::calc_len(
                                                CONCATP_NHPMWYD3NJA,
                                            );
                                            #[doc(hidden)]
                                            const CONCAT_ARR: &::const_format::pmr::LenAndArray<
                                                [u8; ARR_LEN],
                                            > = &::const_format::pmr::__priv_concatenate(
                                                CONCATP_NHPMWYD3NJA,
                                            );
                                            #[doc(hidden)]
                                            #[allow(clippy::transmute_ptr_to_ptr)]
                                            const CONCAT_STR: &str = unsafe {
                                                let slice = ::const_format::pmr::transmute::<
                                                    &[u8; ARR_LEN],
                                                    &[u8; CONCAT_ARR.len],
                                                >(&CONCAT_ARR.array);
                                                {
                                                    let bytes: &'static [::const_format::pmr::u8] = slice;
                                                    let string: &'static ::const_format::pmr::str = {
                                                        ::const_format::__hidden_utils::PtrToRef {
                                                            ptr: bytes as *const [::const_format::pmr::u8] as *const str,
                                                        }
                                                            .reff
                                                    };
                                                    string
                                                }
                                            };
                                            CONCAT_STR
                                        }
                                    })
                                },
                            }
                                .x;
                            {
                                ::core::panicking::panic_display(&msg);
                            };
                        }
                        const fn duplicate_info(
                            array: &[(usize, &'static str); 1usize],
                        ) -> (bool, usize, usize) {
                            let len = 1usize;
                            let mut i = 0usize;
                            while i < len {
                                let mut j = i + 1;
                                while j < len {
                                    if array[i].0 == array[j].0 {
                                        return (true, i, j);
                                    }
                                    j += 1;
                                }
                                i += 1;
                            }
                            (false, 0, 0)
                        }
                        const DUP_INFO: (bool, usize, usize) = duplicate_info(&indices);
                        if DUP_INFO.0 {
                            let msg = ::const_format::pmr::__AssertStr {
                                x: {
                                    use ::const_format::__cf_osRcTFl4A;
                                    ({
                                        #[doc(hidden)]
                                        #[allow(unused_mut, non_snake_case)]
                                        const CONCATP_NHPMWYD3NJA: &[__cf_osRcTFl4A::pmr::PArgument] = {
                                            let fmt = __cf_osRcTFl4A::pmr::FormattingFlags::NEW;
                                            &[
                                                __cf_osRcTFl4A::pmr::PConvWrapper(
                                                        "Found variants that have duplicate indexes. Both `",
                                                    )
                                                    .to_pargument_display(fmt),
                                                __cf_osRcTFl4A::pmr::PConvWrapper(indices[DUP_INFO.1].1)
                                                    .to_pargument_display(fmt),
                                                __cf_osRcTFl4A::pmr::PConvWrapper("` and `")
                                                    .to_pargument_display(fmt),
                                                __cf_osRcTFl4A::pmr::PConvWrapper(indices[DUP_INFO.2].1)
                                                    .to_pargument_display(fmt),
                                                __cf_osRcTFl4A::pmr::PConvWrapper("` have the index `")
                                                    .to_pargument_display(fmt),
                                                __cf_osRcTFl4A::pmr::PConvWrapper(indices[DUP_INFO.1].0)
                                                    .to_pargument_display(fmt),
                                                __cf_osRcTFl4A::pmr::PConvWrapper(
                                                        "`. Use different indexes for each variant.",
                                                    )
                                                    .to_pargument_display(fmt),
                                            ]
                                        };
                                        {
                                            #[doc(hidden)]
                                            const ARR_LEN: usize = ::const_format::pmr::PArgument::calc_len(
                                                CONCATP_NHPMWYD3NJA,
                                            );
                                            #[doc(hidden)]
                                            const CONCAT_ARR: &::const_format::pmr::LenAndArray<
                                                [u8; ARR_LEN],
                                            > = &::const_format::pmr::__priv_concatenate(
                                                CONCATP_NHPMWYD3NJA,
                                            );
                                            #[doc(hidden)]
                                            #[allow(clippy::transmute_ptr_to_ptr)]
                                            const CONCAT_STR: &str = unsafe {
                                                let slice = ::const_format::pmr::transmute::<
                                                    &[u8; ARR_LEN],
                                                    &[u8; CONCAT_ARR.len],
                                                >(&CONCAT_ARR.array);
                                                {
                                                    let bytes: &'static [::const_format::pmr::u8] = slice;
                                                    let string: &'static ::const_format::pmr::str = {
                                                        ::const_format::__hidden_utils::PtrToRef {
                                                            ptr: bytes as *const [::const_format::pmr::u8] as *const str,
                                                        }
                                                            .reff
                                                    };
                                                    string
                                                }
                                            };
                                            CONCAT_STR
                                        }
                                    })
                                },
                            }
                                .x;
                            {
                                ::core::panicking::panic_display(&msg);
                            };
                        }
                    };
                    match *self {
                        Parameters::MinInflation(ref aa, ref ba) => {
                            #[allow(clippy::unnecessary_cast)]
                            __codec_dest_edqy
                                .push_byte((0usize) as ::core::primitive::u8);
                            ::codec::Encode::encode_to(aa, __codec_dest_edqy);
                            ::codec::Encode::encode_to(ba, __codec_dest_edqy);
                        }
                        _ => {}
                    }
                }
            }
            #[automatically_derived]
            impl ::codec::EncodeLike for Parameters {}
        };
        #[allow(deprecated)]
        const _: () = {
            #[automatically_derived]
            impl ::codec::Decode for Parameters {
                fn decode<__CodecInputEdqy: ::codec::Input>(
                    __codec_input_edqy: &mut __CodecInputEdqy,
                ) -> ::core::result::Result<Self, ::codec::Error> {
                    #[automatically_derived]
                    const _: () = {
                        #[allow(clippy::unnecessary_cast)]
                        #[allow(clippy::cast_possible_truncation)]
                        const indices: [(usize, &'static str); 1usize] = [
                            ((0usize) as ::core::primitive::usize, "MinInflation"),
                        ];
                        const fn search_for_invalid_index(
                            array: &[(usize, &'static str); 1usize],
                        ) -> (bool, usize) {
                            let mut i = 0;
                            while i < 1usize {
                                if array[i].0 > 255 {
                                    return (true, i);
                                }
                                i += 1;
                            }
                            (false, 0)
                        }
                        const INVALID_INDEX: (bool, usize) = search_for_invalid_index(
                            &indices,
                        );
                        if INVALID_INDEX.0 {
                            let msg = ::const_format::pmr::__AssertStr {
                                x: {
                                    use ::const_format::__cf_osRcTFl4A;
                                    ({
                                        #[doc(hidden)]
                                        #[allow(unused_mut, non_snake_case)]
                                        const CONCATP_NHPMWYD3NJA: &[__cf_osRcTFl4A::pmr::PArgument] = {
                                            let fmt = __cf_osRcTFl4A::pmr::FormattingFlags::NEW;
                                            &[
                                                __cf_osRcTFl4A::pmr::PConvWrapper("Found variant `")
                                                    .to_pargument_display(fmt),
                                                __cf_osRcTFl4A::pmr::PConvWrapper(
                                                        indices[INVALID_INDEX.1].1,
                                                    )
                                                    .to_pargument_display(fmt),
                                                __cf_osRcTFl4A::pmr::PConvWrapper("` with invalid index: `")
                                                    .to_pargument_display(fmt),
                                                __cf_osRcTFl4A::pmr::PConvWrapper(
                                                        indices[INVALID_INDEX.1].0,
                                                    )
                                                    .to_pargument_display(fmt),
                                                __cf_osRcTFl4A::pmr::PConvWrapper(
                                                        "`. Max supported index is 255.",
                                                    )
                                                    .to_pargument_display(fmt),
                                            ]
                                        };
                                        {
                                            #[doc(hidden)]
                                            const ARR_LEN: usize = ::const_format::pmr::PArgument::calc_len(
                                                CONCATP_NHPMWYD3NJA,
                                            );
                                            #[doc(hidden)]
                                            const CONCAT_ARR: &::const_format::pmr::LenAndArray<
                                                [u8; ARR_LEN],
                                            > = &::const_format::pmr::__priv_concatenate(
                                                CONCATP_NHPMWYD3NJA,
                                            );
                                            #[doc(hidden)]
                                            #[allow(clippy::transmute_ptr_to_ptr)]
                                            const CONCAT_STR: &str = unsafe {
                                                let slice = ::const_format::pmr::transmute::<
                                                    &[u8; ARR_LEN],
                                                    &[u8; CONCAT_ARR.len],
                                                >(&CONCAT_ARR.array);
                                                {
                                                    let bytes: &'static [::const_format::pmr::u8] = slice;
                                                    let string: &'static ::const_format::pmr::str = {
                                                        ::const_format::__hidden_utils::PtrToRef {
                                                            ptr: bytes as *const [::const_format::pmr::u8] as *const str,
                                                        }
                                                            .reff
                                                    };
                                                    string
                                                }
                                            };
                                            CONCAT_STR
                                        }
                                    })
                                },
                            }
                                .x;
                            {
                                ::core::panicking::panic_display(&msg);
                            };
                        }
                        const fn duplicate_info(
                            array: &[(usize, &'static str); 1usize],
                        ) -> (bool, usize, usize) {
                            let len = 1usize;
                            let mut i = 0usize;
                            while i < len {
                                let mut j = i + 1;
                                while j < len {
                                    if array[i].0 == array[j].0 {
                                        return (true, i, j);
                                    }
                                    j += 1;
                                }
                                i += 1;
                            }
                            (false, 0, 0)
                        }
                        const DUP_INFO: (bool, usize, usize) = duplicate_info(&indices);
                        if DUP_INFO.0 {
                            let msg = ::const_format::pmr::__AssertStr {
                                x: {
                                    use ::const_format::__cf_osRcTFl4A;
                                    ({
                                        #[doc(hidden)]
                                        #[allow(unused_mut, non_snake_case)]
                                        const CONCATP_NHPMWYD3NJA: &[__cf_osRcTFl4A::pmr::PArgument] = {
                                            let fmt = __cf_osRcTFl4A::pmr::FormattingFlags::NEW;
                                            &[
                                                __cf_osRcTFl4A::pmr::PConvWrapper(
                                                        "Found variants that have duplicate indexes. Both `",
                                                    )
                                                    .to_pargument_display(fmt),
                                                __cf_osRcTFl4A::pmr::PConvWrapper(indices[DUP_INFO.1].1)
                                                    .to_pargument_display(fmt),
                                                __cf_osRcTFl4A::pmr::PConvWrapper("` and `")
                                                    .to_pargument_display(fmt),
                                                __cf_osRcTFl4A::pmr::PConvWrapper(indices[DUP_INFO.2].1)
                                                    .to_pargument_display(fmt),
                                                __cf_osRcTFl4A::pmr::PConvWrapper("` have the index `")
                                                    .to_pargument_display(fmt),
                                                __cf_osRcTFl4A::pmr::PConvWrapper(indices[DUP_INFO.1].0)
                                                    .to_pargument_display(fmt),
                                                __cf_osRcTFl4A::pmr::PConvWrapper(
                                                        "`. Use different indexes for each variant.",
                                                    )
                                                    .to_pargument_display(fmt),
                                            ]
                                        };
                                        {
                                            #[doc(hidden)]
                                            const ARR_LEN: usize = ::const_format::pmr::PArgument::calc_len(
                                                CONCATP_NHPMWYD3NJA,
                                            );
                                            #[doc(hidden)]
                                            const CONCAT_ARR: &::const_format::pmr::LenAndArray<
                                                [u8; ARR_LEN],
                                            > = &::const_format::pmr::__priv_concatenate(
                                                CONCATP_NHPMWYD3NJA,
                                            );
                                            #[doc(hidden)]
                                            #[allow(clippy::transmute_ptr_to_ptr)]
                                            const CONCAT_STR: &str = unsafe {
                                                let slice = ::const_format::pmr::transmute::<
                                                    &[u8; ARR_LEN],
                                                    &[u8; CONCAT_ARR.len],
                                                >(&CONCAT_ARR.array);
                                                {
                                                    let bytes: &'static [::const_format::pmr::u8] = slice;
                                                    let string: &'static ::const_format::pmr::str = {
                                                        ::const_format::__hidden_utils::PtrToRef {
                                                            ptr: bytes as *const [::const_format::pmr::u8] as *const str,
                                                        }
                                                            .reff
                                                    };
                                                    string
                                                }
                                            };
                                            CONCAT_STR
                                        }
                                    })
                                },
                            }
                                .x;
                            {
                                ::core::panicking::panic_display(&msg);
                            };
                        }
                    };
                    match __codec_input_edqy
                        .read_byte()
                        .map_err(|e| {
                            e
                                .chain(
                                    "Could not decode `Parameters`, failed to read variant byte",
                                )
                        })?
                    {
                        #[allow(clippy::unnecessary_cast)]
                        #[allow(clippy::cast_possible_truncation)]
                        __codec_x_edqy if __codec_x_edqy
                            == (0usize) as ::core::primitive::u8 => {
                            #[allow(clippy::redundant_closure_call)]
                            return (move || {
                                ::core::result::Result::Ok(
                                    Parameters::MinInflation(
                                        {
                                            let __codec_res_edqy = <MinInflation as ::codec::Decode>::decode(
                                                __codec_input_edqy,
                                            );
                                            match __codec_res_edqy {
                                                ::core::result::Result::Err(e) => {
                                                    return ::core::result::Result::Err(
                                                        e.chain("Could not decode `Parameters::MinInflation.0`"),
                                                    );
                                                }
                                                ::core::result::Result::Ok(__codec_res_edqy) => {
                                                    __codec_res_edqy
                                                }
                                            }
                                        },
                                        {
                                            let __codec_res_edqy = <Option<
                                                u32,
                                            > as ::codec::Decode>::decode(__codec_input_edqy);
                                            match __codec_res_edqy {
                                                ::core::result::Result::Err(e) => {
                                                    return ::core::result::Result::Err(
                                                        e.chain("Could not decode `Parameters::MinInflation.1`"),
                                                    );
                                                }
                                                ::core::result::Result::Ok(__codec_res_edqy) => {
                                                    __codec_res_edqy
                                                }
                                            }
                                        },
                                    ),
                                )
                            })();
                        }
                        _ => {
                            #[allow(clippy::redundant_closure_call)]
                            return (move || {
                                ::core::result::Result::Err(
                                    <_ as ::core::convert::Into<
                                        _,
                                    >>::into(
                                        "Could not decode `Parameters`, variant doesn't exist",
                                    ),
                                )
                            })();
                        }
                    }
                }
            }
        };
        #[allow(deprecated)]
        const _: () = {
            fn check_struct() {
                fn check_field<T: ::codec::DecodeWithMemTracking>() {}
                check_field::<MinInflation>();
                check_field::<Option<u32>>();
            }
            #[automatically_derived]
            impl ::codec::DecodeWithMemTracking for Parameters {}
        };
        const _: () = {
            #[automatically_derived]
            impl ::codec::MaxEncodedLen for Parameters {
                fn max_encoded_len() -> ::core::primitive::usize {
                    0_usize
                        .max(
                            0_usize
                                .saturating_add(
                                    <MinInflation as ::codec::MaxEncodedLen>::max_encoded_len(),
                                )
                                .saturating_add(
                                    <Option<u32> as ::codec::MaxEncodedLen>::max_encoded_len(),
                                ),
                        )
                        .saturating_add(1)
                }
            }
        };
        impl core::fmt::Debug for Parameters {
            fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
                match self {
                    Self::MinInflation(ref a0, ref a1) => {
                        fmt.debug_tuple("Parameters::MinInflation")
                            .field(a0)
                            .field(a1)
                            .finish()
                    }
                    _ => Ok(()),
                }
            }
        }
        #[allow(
            non_upper_case_globals,
            deprecated,
            unused_attributes,
            unused_qualifications
        )]
        const _: () = {
            impl ::scale_info::TypeInfo for Parameters {
                type Identity = Self;
                fn type_info() -> ::scale_info::Type {
                    ::scale_info::Type::builder()
                        .path(
                            ::scale_info::Path::new_with_replace(
                                "Parameters",
                                "polkadot_test::dynamic_params::inflationAV",
                                &[],
                            ),
                        )
                        .type_params(::alloc::vec::Vec::new())
                        .variant(
                            ::scale_info::build::Variants::new()
                                .variant(
                                    "MinInflation",
                                    |v| {
                                        v
                                            .index(0u8 as ::core::primitive::u8)
                                            .fields(
                                                ::scale_info::build::Fields::unnamed()
                                                    .field(|f| f.ty::<MinInflation>().type_name("MinInflation"))
                                                    .field(|f| f.ty::<Option<u32>>().type_name("Option<u32>")),
                                            )
                                            .docs(
                                                &["Minimum inflation rate used to calculate era payouts."],
                                            )
                                    },
                                ),
                        )
                }
            }
        };
        #[doc(hidden)]
        pub enum ParametersKey {
            /// Minimum inflation rate used to calculate era payouts.
            #[codec(index = 0)]
            MinInflation(MinInflation),
        }
        #[automatically_derived]
        impl ::core::clone::Clone for ParametersKey {
            #[inline]
            fn clone(&self) -> ParametersKey {
                match self {
                    ParametersKey::MinInflation(__self_0) => {
                        ParametersKey::MinInflation(
                            ::core::clone::Clone::clone(__self_0),
                        )
                    }
                }
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for ParametersKey {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for ParametersKey {
            #[inline]
            fn eq(&self, other: &ParametersKey) -> bool {
                match (self, other) {
                    (
                        ParametersKey::MinInflation(__self_0),
                        ParametersKey::MinInflation(__arg1_0),
                    ) => __self_0 == __arg1_0,
                }
            }
        }
        #[automatically_derived]
        impl ::core::cmp::Eq for ParametersKey {
            #[inline]
            #[doc(hidden)]
            #[coverage(off)]
            fn assert_receiver_is_total_eq(&self) -> () {
                let _: ::core::cmp::AssertParamIsEq<MinInflation>;
            }
        }
        #[allow(deprecated)]
        const _: () = {
            #[automatically_derived]
            impl ::codec::Encode for ParametersKey {
                fn size_hint(&self) -> usize {
                    1_usize
                        + match *self {
                            ParametersKey::MinInflation(ref aa) => {
                                0_usize.saturating_add(::codec::Encode::size_hint(aa))
                            }
                            _ => 0_usize,
                        }
                }
                fn encode_to<
                    __CodecOutputEdqy: ::codec::Output + ?::core::marker::Sized,
                >(&self, __codec_dest_edqy: &mut __CodecOutputEdqy) {
                    #[automatically_derived]
                    const _: () = {
                        #[allow(clippy::unnecessary_cast)]
                        #[allow(clippy::cast_possible_truncation)]
                        const indices: [(usize, &'static str); 1usize] = [
                            ((0usize) as ::core::primitive::usize, "MinInflation"),
                        ];
                        const fn search_for_invalid_index(
                            array: &[(usize, &'static str); 1usize],
                        ) -> (bool, usize) {
                            let mut i = 0;
                            while i < 1usize {
                                if array[i].0 > 255 {
                                    return (true, i);
                                }
                                i += 1;
                            }
                            (false, 0)
                        }
                        const INVALID_INDEX: (bool, usize) = search_for_invalid_index(
                            &indices,
                        );
                        if INVALID_INDEX.0 {
                            let msg = ::const_format::pmr::__AssertStr {
                                x: {
                                    use ::const_format::__cf_osRcTFl4A;
                                    ({
                                        #[doc(hidden)]
                                        #[allow(unused_mut, non_snake_case)]
                                        const CONCATP_NHPMWYD3NJA: &[__cf_osRcTFl4A::pmr::PArgument] = {
                                            let fmt = __cf_osRcTFl4A::pmr::FormattingFlags::NEW;
                                            &[
                                                __cf_osRcTFl4A::pmr::PConvWrapper("Found variant `")
                                                    .to_pargument_display(fmt),
                                                __cf_osRcTFl4A::pmr::PConvWrapper(
                                                        indices[INVALID_INDEX.1].1,
                                                    )
                                                    .to_pargument_display(fmt),
                                                __cf_osRcTFl4A::pmr::PConvWrapper("` with invalid index: `")
                                                    .to_pargument_display(fmt),
                                                __cf_osRcTFl4A::pmr::PConvWrapper(
                                                        indices[INVALID_INDEX.1].0,
                                                    )
                                                    .to_pargument_display(fmt),
                                                __cf_osRcTFl4A::pmr::PConvWrapper(
                                                        "`. Max supported index is 255.",
                                                    )
                                                    .to_pargument_display(fmt),
                                            ]
                                        };
                                        {
                                            #[doc(hidden)]
                                            const ARR_LEN: usize = ::const_format::pmr::PArgument::calc_len(
                                                CONCATP_NHPMWYD3NJA,
                                            );
                                            #[doc(hidden)]
                                            const CONCAT_ARR: &::const_format::pmr::LenAndArray<
                                                [u8; ARR_LEN],
                                            > = &::const_format::pmr::__priv_concatenate(
                                                CONCATP_NHPMWYD3NJA,
                                            );
                                            #[doc(hidden)]
                                            #[allow(clippy::transmute_ptr_to_ptr)]
                                            const CONCAT_STR: &str = unsafe {
                                                let slice = ::const_format::pmr::transmute::<
                                                    &[u8; ARR_LEN],
                                                    &[u8; CONCAT_ARR.len],
                                                >(&CONCAT_ARR.array);
                                                {
                                                    let bytes: &'static [::const_format::pmr::u8] = slice;
                                                    let string: &'static ::const_format::pmr::str = {
                                                        ::const_format::__hidden_utils::PtrToRef {
                                                            ptr: bytes as *const [::const_format::pmr::u8] as *const str,
                                                        }
                                                            .reff
                                                    };
                                                    string
                                                }
                                            };
                                            CONCAT_STR
                                        }
                                    })
                                },
                            }
                                .x;
                            {
                                ::core::panicking::panic_display(&msg);
                            };
                        }
                        const fn duplicate_info(
                            array: &[(usize, &'static str); 1usize],
                        ) -> (bool, usize, usize) {
                            let len = 1usize;
                            let mut i = 0usize;
                            while i < len {
                                let mut j = i + 1;
                                while j < len {
                                    if array[i].0 == array[j].0 {
                                        return (true, i, j);
                                    }
                                    j += 1;
                                }
                                i += 1;
                            }
                            (false, 0, 0)
                        }
                        const DUP_INFO: (bool, usize, usize) = duplicate_info(&indices);
                        if DUP_INFO.0 {
                            let msg = ::const_format::pmr::__AssertStr {
                                x: {
                                    use ::const_format::__cf_osRcTFl4A;
                                    ({
                                        #[doc(hidden)]
                                        #[allow(unused_mut, non_snake_case)]
                                        const CONCATP_NHPMWYD3NJA: &[__cf_osRcTFl4A::pmr::PArgument] = {
                                            let fmt = __cf_osRcTFl4A::pmr::FormattingFlags::NEW;
                                            &[
                                                __cf_osRcTFl4A::pmr::PConvWrapper(
                                                        "Found variants that have duplicate indexes. Both `",
                                                    )
                                                    .to_pargument_display(fmt),
                                                __cf_osRcTFl4A::pmr::PConvWrapper(indices[DUP_INFO.1].1)
                                                    .to_pargument_display(fmt),
                                                __cf_osRcTFl4A::pmr::PConvWrapper("` and `")
                                                    .to_pargument_display(fmt),
                                                __cf_osRcTFl4A::pmr::PConvWrapper(indices[DUP_INFO.2].1)
                                                    .to_pargument_display(fmt),
                                                __cf_osRcTFl4A::pmr::PConvWrapper("` have the index `")
                                                    .to_pargument_display(fmt),
                                                __cf_osRcTFl4A::pmr::PConvWrapper(indices[DUP_INFO.1].0)
                                                    .to_pargument_display(fmt),
                                                __cf_osRcTFl4A::pmr::PConvWrapper(
                                                        "`. Use different indexes for each variant.",
                                                    )
                                                    .to_pargument_display(fmt),
                                            ]
                                        };
                                        {
                                            #[doc(hidden)]
                                            const ARR_LEN: usize = ::const_format::pmr::PArgument::calc_len(
                                                CONCATP_NHPMWYD3NJA,
                                            );
                                            #[doc(hidden)]
                                            const CONCAT_ARR: &::const_format::pmr::LenAndArray<
                                                [u8; ARR_LEN],
                                            > = &::const_format::pmr::__priv_concatenate(
                                                CONCATP_NHPMWYD3NJA,
                                            );
                                            #[doc(hidden)]
                                            #[allow(clippy::transmute_ptr_to_ptr)]
                                            const CONCAT_STR: &str = unsafe {
                                                let slice = ::const_format::pmr::transmute::<
                                                    &[u8; ARR_LEN],
                                                    &[u8; CONCAT_ARR.len],
                                                >(&CONCAT_ARR.array);
                                                {
                                                    let bytes: &'static [::const_format::pmr::u8] = slice;
                                                    let string: &'static ::const_format::pmr::str = {
                                                        ::const_format::__hidden_utils::PtrToRef {
                                                            ptr: bytes as *const [::const_format::pmr::u8] as *const str,
                                                        }
                                                            .reff
                                                    };
                                                    string
                                                }
                                            };
                                            CONCAT_STR
                                        }
                                    })
                                },
                            }
                                .x;
                            {
                                ::core::panicking::panic_display(&msg);
                            };
                        }
                    };
                    match *self {
                        ParametersKey::MinInflation(ref aa) => {
                            #[allow(clippy::unnecessary_cast)]
                            __codec_dest_edqy
                                .push_byte((0usize) as ::core::primitive::u8);
                            ::codec::Encode::encode_to(aa, __codec_dest_edqy);
                        }
                        _ => {}
                    }
                }
            }
            #[automatically_derived]
            impl ::codec::EncodeLike for ParametersKey {}
        };
        #[allow(deprecated)]
        const _: () = {
            #[automatically_derived]
            impl ::codec::Decode for ParametersKey {
                fn decode<__CodecInputEdqy: ::codec::Input>(
                    __codec_input_edqy: &mut __CodecInputEdqy,
                ) -> ::core::result::Result<Self, ::codec::Error> {
                    #[automatically_derived]
                    const _: () = {
                        #[allow(clippy::unnecessary_cast)]
                        #[allow(clippy::cast_possible_truncation)]
                        const indices: [(usize, &'static str); 1usize] = [
                            ((0usize) as ::core::primitive::usize, "MinInflation"),
                        ];
                        const fn search_for_invalid_index(
                            array: &[(usize, &'static str); 1usize],
                        ) -> (bool, usize) {
                            let mut i = 0;
                            while i < 1usize {
                                if array[i].0 > 255 {
                                    return (true, i);
                                }
                                i += 1;
                            }
                            (false, 0)
                        }
                        const INVALID_INDEX: (bool, usize) = search_for_invalid_index(
                            &indices,
                        );
                        if INVALID_INDEX.0 {
                            let msg = ::const_format::pmr::__AssertStr {
                                x: {
                                    use ::const_format::__cf_osRcTFl4A;
                                    ({
                                        #[doc(hidden)]
                                        #[allow(unused_mut, non_snake_case)]
                                        const CONCATP_NHPMWYD3NJA: &[__cf_osRcTFl4A::pmr::PArgument] = {
                                            let fmt = __cf_osRcTFl4A::pmr::FormattingFlags::NEW;
                                            &[
                                                __cf_osRcTFl4A::pmr::PConvWrapper("Found variant `")
                                                    .to_pargument_display(fmt),
                                                __cf_osRcTFl4A::pmr::PConvWrapper(
                                                        indices[INVALID_INDEX.1].1,
                                                    )
                                                    .to_pargument_display(fmt),
                                                __cf_osRcTFl4A::pmr::PConvWrapper("` with invalid index: `")
                                                    .to_pargument_display(fmt),
                                                __cf_osRcTFl4A::pmr::PConvWrapper(
                                                        indices[INVALID_INDEX.1].0,
                                                    )
                                                    .to_pargument_display(fmt),
                                                __cf_osRcTFl4A::pmr::PConvWrapper(
                                                        "`. Max supported index is 255.",
                                                    )
                                                    .to_pargument_display(fmt),
                                            ]
                                        };
                                        {
                                            #[doc(hidden)]
                                            const ARR_LEN: usize = ::const_format::pmr::PArgument::calc_len(
                                                CONCATP_NHPMWYD3NJA,
                                            );
                                            #[doc(hidden)]
                                            const CONCAT_ARR: &::const_format::pmr::LenAndArray<
                                                [u8; ARR_LEN],
                                            > = &::const_format::pmr::__priv_concatenate(
                                                CONCATP_NHPMWYD3NJA,
                                            );
                                            #[doc(hidden)]
                                            #[allow(clippy::transmute_ptr_to_ptr)]
                                            const CONCAT_STR: &str = unsafe {
                                                let slice = ::const_format::pmr::transmute::<
                                                    &[u8; ARR_LEN],
                                                    &[u8; CONCAT_ARR.len],
                                                >(&CONCAT_ARR.array);
                                                {
                                                    let bytes: &'static [::const_format::pmr::u8] = slice;
                                                    let string: &'static ::const_format::pmr::str = {
                                                        ::const_format::__hidden_utils::PtrToRef {
                                                            ptr: bytes as *const [::const_format::pmr::u8] as *const str,
                                                        }
                                                            .reff
                                                    };
                                                    string
                                                }
                                            };
                                            CONCAT_STR
                                        }
                                    })
                                },
                            }
                                .x;
                            {
                                ::core::panicking::panic_display(&msg);
                            };
                        }
                        const fn duplicate_info(
                            array: &[(usize, &'static str); 1usize],
                        ) -> (bool, usize, usize) {
                            let len = 1usize;
                            let mut i = 0usize;
                            while i < len {
                                let mut j = i + 1;
                                while j < len {
                                    if array[i].0 == array[j].0 {
                                        return (true, i, j);
                                    }
                                    j += 1;
                                }
                                i += 1;
                            }
                            (false, 0, 0)
                        }
                        const DUP_INFO: (bool, usize, usize) = duplicate_info(&indices);
                        if DUP_INFO.0 {
                            let msg = ::const_format::pmr::__AssertStr {
                                x: {
                                    use ::const_format::__cf_osRcTFl4A;
                                    ({
                                        #[doc(hidden)]
                                        #[allow(unused_mut, non_snake_case)]
                                        const CONCATP_NHPMWYD3NJA: &[__cf_osRcTFl4A::pmr::PArgument] = {
                                            let fmt = __cf_osRcTFl4A::pmr::FormattingFlags::NEW;
                                            &[
                                                __cf_osRcTFl4A::pmr::PConvWrapper(
                                                        "Found variants that have duplicate indexes. Both `",
                                                    )
                                                    .to_pargument_display(fmt),
                                                __cf_osRcTFl4A::pmr::PConvWrapper(indices[DUP_INFO.1].1)
                                                    .to_pargument_display(fmt),
                                                __cf_osRcTFl4A::pmr::PConvWrapper("` and `")
                                                    .to_pargument_display(fmt),
                                                __cf_osRcTFl4A::pmr::PConvWrapper(indices[DUP_INFO.2].1)
                                                    .to_pargument_display(fmt),
                                                __cf_osRcTFl4A::pmr::PConvWrapper("` have the index `")
                                                    .to_pargument_display(fmt),
                                                __cf_osRcTFl4A::pmr::PConvWrapper(indices[DUP_INFO.1].0)
                                                    .to_pargument_display(fmt),
                                                __cf_osRcTFl4A::pmr::PConvWrapper(
                                                        "`. Use different indexes for each variant.",
                                                    )
                                                    .to_pargument_display(fmt),
                                            ]
                                        };
                                        {
                                            #[doc(hidden)]
                                            const ARR_LEN: usize = ::const_format::pmr::PArgument::calc_len(
                                                CONCATP_NHPMWYD3NJA,
                                            );
                                            #[doc(hidden)]
                                            const CONCAT_ARR: &::const_format::pmr::LenAndArray<
                                                [u8; ARR_LEN],
                                            > = &::const_format::pmr::__priv_concatenate(
                                                CONCATP_NHPMWYD3NJA,
                                            );
                                            #[doc(hidden)]
                                            #[allow(clippy::transmute_ptr_to_ptr)]
                                            const CONCAT_STR: &str = unsafe {
                                                let slice = ::const_format::pmr::transmute::<
                                                    &[u8; ARR_LEN],
                                                    &[u8; CONCAT_ARR.len],
                                                >(&CONCAT_ARR.array);
                                                {
                                                    let bytes: &'static [::const_format::pmr::u8] = slice;
                                                    let string: &'static ::const_format::pmr::str = {
                                                        ::const_format::__hidden_utils::PtrToRef {
                                                            ptr: bytes as *const [::const_format::pmr::u8] as *const str,
                                                        }
                                                            .reff
                                                    };
                                                    string
                                                }
                                            };
                                            CONCAT_STR
                                        }
                                    })
                                },
                            }
                                .x;
                            {
                                ::core::panicking::panic_display(&msg);
                            };
                        }
                    };
                    match __codec_input_edqy
                        .read_byte()
                        .map_err(|e| {
                            e
                                .chain(
                                    "Could not decode `ParametersKey`, failed to read variant byte",
                                )
                        })?
                    {
                        #[allow(clippy::unnecessary_cast)]
                        #[allow(clippy::cast_possible_truncation)]
                        __codec_x_edqy if __codec_x_edqy
                            == (0usize) as ::core::primitive::u8 => {
                            #[allow(clippy::redundant_closure_call)]
                            return (move || {
                                ::core::result::Result::Ok(
                                    ParametersKey::MinInflation({
                                        let __codec_res_edqy = <MinInflation as ::codec::Decode>::decode(
                                            __codec_input_edqy,
                                        );
                                        match __codec_res_edqy {
                                            ::core::result::Result::Err(e) => {
                                                return ::core::result::Result::Err(
                                                    e.chain("Could not decode `ParametersKey::MinInflation.0`"),
                                                );
                                            }
                                            ::core::result::Result::Ok(__codec_res_edqy) => {
                                                __codec_res_edqy
                                            }
                                        }
                                    }),
                                )
                            })();
                        }
                        _ => {
                            #[allow(clippy::redundant_closure_call)]
                            return (move || {
                                ::core::result::Result::Err(
                                    <_ as ::core::convert::Into<
                                        _,
                                    >>::into(
                                        "Could not decode `ParametersKey`, variant doesn't exist",
                                    ),
                                )
                            })();
                        }
                    }
                }
            }
        };
        #[allow(deprecated)]
        const _: () = {
            fn check_struct() {
                fn check_field<T: ::codec::DecodeWithMemTracking>() {}
                check_field::<MinInflation>();
            }
            #[automatically_derived]
            impl ::codec::DecodeWithMemTracking for ParametersKey {}
        };
        const _: () = {
            #[automatically_derived]
            impl ::codec::MaxEncodedLen for ParametersKey {
                fn max_encoded_len() -> ::core::primitive::usize {
                    0_usize
                        .max(
                            0_usize
                                .saturating_add(
                                    <MinInflation as ::codec::MaxEncodedLen>::max_encoded_len(),
                                ),
                        )
                        .saturating_add(1)
                }
            }
        };
        impl core::fmt::Debug for ParametersKey {
            fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
                match self {
                    Self::MinInflation(ref a0) => {
                        fmt.debug_tuple("ParametersKey::MinInflation").field(a0).finish()
                    }
                    _ => Ok(()),
                }
            }
        }
        #[allow(
            non_upper_case_globals,
            deprecated,
            unused_attributes,
            unused_qualifications
        )]
        const _: () = {
            impl ::scale_info::TypeInfo for ParametersKey {
                type Identity = Self;
                fn type_info() -> ::scale_info::Type {
                    ::scale_info::Type::builder()
                        .path(
                            ::scale_info::Path::new_with_replace(
                                "ParametersKey",
                                "polkadot_test::dynamic_params::inflationAV",
                                &[],
                            ),
                        )
                        .type_params(::alloc::vec::Vec::new())
                        .variant(
                            ::scale_info::build::Variants::new()
                                .variant(
                                    "MinInflation",
                                    |v| {
                                        v
                                            .index(0u8 as ::core::primitive::u8)
                                            .fields(
                                                ::scale_info::build::Fields::unnamed()
                                                    .field(|f| f.ty::<MinInflation>().type_name("MinInflation")),
                                            )
                                            .docs(
                                                &["Minimum inflation rate used to calculate era payouts."],
                                            )
                                    },
                                ),
                        )
                }
            }
        };
        #[doc(hidden)]
        pub enum ParametersValue {
            /// Minimum inflation rate used to calculate era payouts.
            #[codec(index = 0)]
            MinInflation(u32),
        }
        #[automatically_derived]
        impl ::core::clone::Clone for ParametersValue {
            #[inline]
            fn clone(&self) -> ParametersValue {
                match self {
                    ParametersValue::MinInflation(__self_0) => {
                        ParametersValue::MinInflation(
                            ::core::clone::Clone::clone(__self_0),
                        )
                    }
                }
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for ParametersValue {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for ParametersValue {
            #[inline]
            fn eq(&self, other: &ParametersValue) -> bool {
                match (self, other) {
                    (
                        ParametersValue::MinInflation(__self_0),
                        ParametersValue::MinInflation(__arg1_0),
                    ) => __self_0 == __arg1_0,
                }
            }
        }
        #[automatically_derived]
        impl ::core::cmp::Eq for ParametersValue {
            #[inline]
            #[doc(hidden)]
            #[coverage(off)]
            fn assert_receiver_is_total_eq(&self) -> () {
                let _: ::core::cmp::AssertParamIsEq<u32>;
            }
        }
        #[allow(deprecated)]
        const _: () = {
            #[automatically_derived]
            impl ::codec::Encode for ParametersValue {
                fn size_hint(&self) -> usize {
                    1_usize
                        + match *self {
                            ParametersValue::MinInflation(ref aa) => {
                                0_usize.saturating_add(::codec::Encode::size_hint(aa))
                            }
                            _ => 0_usize,
                        }
                }
                fn encode_to<
                    __CodecOutputEdqy: ::codec::Output + ?::core::marker::Sized,
                >(&self, __codec_dest_edqy: &mut __CodecOutputEdqy) {
                    #[automatically_derived]
                    const _: () = {
                        #[allow(clippy::unnecessary_cast)]
                        #[allow(clippy::cast_possible_truncation)]
                        const indices: [(usize, &'static str); 1usize] = [
                            ((0usize) as ::core::primitive::usize, "MinInflation"),
                        ];
                        const fn search_for_invalid_index(
                            array: &[(usize, &'static str); 1usize],
                        ) -> (bool, usize) {
                            let mut i = 0;
                            while i < 1usize {
                                if array[i].0 > 255 {
                                    return (true, i);
                                }
                                i += 1;
                            }
                            (false, 0)
                        }
                        const INVALID_INDEX: (bool, usize) = search_for_invalid_index(
                            &indices,
                        );
                        if INVALID_INDEX.0 {
                            let msg = ::const_format::pmr::__AssertStr {
                                x: {
                                    use ::const_format::__cf_osRcTFl4A;
                                    ({
                                        #[doc(hidden)]
                                        #[allow(unused_mut, non_snake_case)]
                                        const CONCATP_NHPMWYD3NJA: &[__cf_osRcTFl4A::pmr::PArgument] = {
                                            let fmt = __cf_osRcTFl4A::pmr::FormattingFlags::NEW;
                                            &[
                                                __cf_osRcTFl4A::pmr::PConvWrapper("Found variant `")
                                                    .to_pargument_display(fmt),
                                                __cf_osRcTFl4A::pmr::PConvWrapper(
                                                        indices[INVALID_INDEX.1].1,
                                                    )
                                                    .to_pargument_display(fmt),
                                                __cf_osRcTFl4A::pmr::PConvWrapper("` with invalid index: `")
                                                    .to_pargument_display(fmt),
                                                __cf_osRcTFl4A::pmr::PConvWrapper(
                                                        indices[INVALID_INDEX.1].0,
                                                    )
                                                    .to_pargument_display(fmt),
                                                __cf_osRcTFl4A::pmr::PConvWrapper(
                                                        "`. Max supported index is 255.",
                                                    )
                                                    .to_pargument_display(fmt),
                                            ]
                                        };
                                        {
                                            #[doc(hidden)]
                                            const ARR_LEN: usize = ::const_format::pmr::PArgument::calc_len(
                                                CONCATP_NHPMWYD3NJA,
                                            );
                                            #[doc(hidden)]
                                            const CONCAT_ARR: &::const_format::pmr::LenAndArray<
                                                [u8; ARR_LEN],
                                            > = &::const_format::pmr::__priv_concatenate(
                                                CONCATP_NHPMWYD3NJA,
                                            );
                                            #[doc(hidden)]
                                            #[allow(clippy::transmute_ptr_to_ptr)]
                                            const CONCAT_STR: &str = unsafe {
                                                let slice = ::const_format::pmr::transmute::<
                                                    &[u8; ARR_LEN],
                                                    &[u8; CONCAT_ARR.len],
                                                >(&CONCAT_ARR.array);
                                                {
                                                    let bytes: &'static [::const_format::pmr::u8] = slice;
                                                    let string: &'static ::const_format::pmr::str = {
                                                        ::const_format::__hidden_utils::PtrToRef {
                                                            ptr: bytes as *const [::const_format::pmr::u8] as *const str,
                                                        }
                                                            .reff
                                                    };
                                                    string
                                                }
                                            };
                                            CONCAT_STR
                                        }
                                    })
                                },
                            }
                                .x;
                            {
                                ::core::panicking::panic_display(&msg);
                            };
                        }
                        const fn duplicate_info(
                            array: &[(usize, &'static str); 1usize],
                        ) -> (bool, usize, usize) {
                            let len = 1usize;
                            let mut i = 0usize;
                            while i < len {
                                let mut j = i + 1;
                                while j < len {
                                    if array[i].0 == array[j].0 {
                                        return (true, i, j);
                                    }
                                    j += 1;
                                }
                                i += 1;
                            }
                            (false, 0, 0)
                        }
                        const DUP_INFO: (bool, usize, usize) = duplicate_info(&indices);
                        if DUP_INFO.0 {
                            let msg = ::const_format::pmr::__AssertStr {
                                x: {
                                    use ::const_format::__cf_osRcTFl4A;
                                    ({
                                        #[doc(hidden)]
                                        #[allow(unused_mut, non_snake_case)]
                                        const CONCATP_NHPMWYD3NJA: &[__cf_osRcTFl4A::pmr::PArgument] = {
                                            let fmt = __cf_osRcTFl4A::pmr::FormattingFlags::NEW;
                                            &[
                                                __cf_osRcTFl4A::pmr::PConvWrapper(
                                                        "Found variants that have duplicate indexes. Both `",
                                                    )
                                                    .to_pargument_display(fmt),
                                                __cf_osRcTFl4A::pmr::PConvWrapper(indices[DUP_INFO.1].1)
                                                    .to_pargument_display(fmt),
                                                __cf_osRcTFl4A::pmr::PConvWrapper("` and `")
                                                    .to_pargument_display(fmt),
                                                __cf_osRcTFl4A::pmr::PConvWrapper(indices[DUP_INFO.2].1)
                                                    .to_pargument_display(fmt),
                                                __cf_osRcTFl4A::pmr::PConvWrapper("` have the index `")
                                                    .to_pargument_display(fmt),
                                                __cf_osRcTFl4A::pmr::PConvWrapper(indices[DUP_INFO.1].0)
                                                    .to_pargument_display(fmt),
                                                __cf_osRcTFl4A::pmr::PConvWrapper(
                                                        "`. Use different indexes for each variant.",
                                                    )
                                                    .to_pargument_display(fmt),
                                            ]
                                        };
                                        {
                                            #[doc(hidden)]
                                            const ARR_LEN: usize = ::const_format::pmr::PArgument::calc_len(
                                                CONCATP_NHPMWYD3NJA,
                                            );
                                            #[doc(hidden)]
                                            const CONCAT_ARR: &::const_format::pmr::LenAndArray<
                                                [u8; ARR_LEN],
                                            > = &::const_format::pmr::__priv_concatenate(
                                                CONCATP_NHPMWYD3NJA,
                                            );
                                            #[doc(hidden)]
                                            #[allow(clippy::transmute_ptr_to_ptr)]
                                            const CONCAT_STR: &str = unsafe {
                                                let slice = ::const_format::pmr::transmute::<
                                                    &[u8; ARR_LEN],
                                                    &[u8; CONCAT_ARR.len],
                                                >(&CONCAT_ARR.array);
                                                {
                                                    let bytes: &'static [::const_format::pmr::u8] = slice;
                                                    let string: &'static ::const_format::pmr::str = {
                                                        ::const_format::__hidden_utils::PtrToRef {
                                                            ptr: bytes as *const [::const_format::pmr::u8] as *const str,
                                                        }
                                                            .reff
                                                    };
                                                    string
                                                }
                                            };
                                            CONCAT_STR
                                        }
                                    })
                                },
                            }
                                .x;
                            {
                                ::core::panicking::panic_display(&msg);
                            };
                        }
                    };
                    match *self {
                        ParametersValue::MinInflation(ref aa) => {
                            #[allow(clippy::unnecessary_cast)]
                            __codec_dest_edqy
                                .push_byte((0usize) as ::core::primitive::u8);
                            ::codec::Encode::encode_to(aa, __codec_dest_edqy);
                        }
                        _ => {}
                    }
                }
            }
            #[automatically_derived]
            impl ::codec::EncodeLike for ParametersValue {}
        };
        #[allow(deprecated)]
        const _: () = {
            #[automatically_derived]
            impl ::codec::Decode for ParametersValue {
                fn decode<__CodecInputEdqy: ::codec::Input>(
                    __codec_input_edqy: &mut __CodecInputEdqy,
                ) -> ::core::result::Result<Self, ::codec::Error> {
                    #[automatically_derived]
                    const _: () = {
                        #[allow(clippy::unnecessary_cast)]
                        #[allow(clippy::cast_possible_truncation)]
                        const indices: [(usize, &'static str); 1usize] = [
                            ((0usize) as ::core::primitive::usize, "MinInflation"),
                        ];
                        const fn search_for_invalid_index(
                            array: &[(usize, &'static str); 1usize],
                        ) -> (bool, usize) {
                            let mut i = 0;
                            while i < 1usize {
                                if array[i].0 > 255 {
                                    return (true, i);
                                }
                                i += 1;
                            }
                            (false, 0)
                        }
                        const INVALID_INDEX: (bool, usize) = search_for_invalid_index(
                            &indices,
                        );
                        if INVALID_INDEX.0 {
                            let msg = ::const_format::pmr::__AssertStr {
                                x: {
                                    use ::const_format::__cf_osRcTFl4A;
                                    ({
                                        #[doc(hidden)]
                                        #[allow(unused_mut, non_snake_case)]
                                        const CONCATP_NHPMWYD3NJA: &[__cf_osRcTFl4A::pmr::PArgument] = {
                                            let fmt = __cf_osRcTFl4A::pmr::FormattingFlags::NEW;
                                            &[
                                                __cf_osRcTFl4A::pmr::PConvWrapper("Found variant `")
                                                    .to_pargument_display(fmt),
                                                __cf_osRcTFl4A::pmr::PConvWrapper(
                                                        indices[INVALID_INDEX.1].1,
                                                    )
                                                    .to_pargument_display(fmt),
                                                __cf_osRcTFl4A::pmr::PConvWrapper("` with invalid index: `")
                                                    .to_pargument_display(fmt),
                                                __cf_osRcTFl4A::pmr::PConvWrapper(
                                                        indices[INVALID_INDEX.1].0,
                                                    )
                                                    .to_pargument_display(fmt),
                                                __cf_osRcTFl4A::pmr::PConvWrapper(
                                                        "`. Max supported index is 255.",
                                                    )
                                                    .to_pargument_display(fmt),
                                            ]
                                        };
                                        {
                                            #[doc(hidden)]
                                            const ARR_LEN: usize = ::const_format::pmr::PArgument::calc_len(
                                                CONCATP_NHPMWYD3NJA,
                                            );
                                            #[doc(hidden)]
                                            const CONCAT_ARR: &::const_format::pmr::LenAndArray<
                                                [u8; ARR_LEN],
                                            > = &::const_format::pmr::__priv_concatenate(
                                                CONCATP_NHPMWYD3NJA,
                                            );
                                            #[doc(hidden)]
                                            #[allow(clippy::transmute_ptr_to_ptr)]
                                            const CONCAT_STR: &str = unsafe {
                                                let slice = ::const_format::pmr::transmute::<
                                                    &[u8; ARR_LEN],
                                                    &[u8; CONCAT_ARR.len],
                                                >(&CONCAT_ARR.array);
                                                {
                                                    let bytes: &'static [::const_format::pmr::u8] = slice;
                                                    let string: &'static ::const_format::pmr::str = {
                                                        ::const_format::__hidden_utils::PtrToRef {
                                                            ptr: bytes as *const [::const_format::pmr::u8] as *const str,
                                                        }
                                                            .reff
                                                    };
                                                    string
                                                }
                                            };
                                            CONCAT_STR
                                        }
                                    })
                                },
                            }
                                .x;
                            {
                                ::core::panicking::panic_display(&msg);
                            };
                        }
                        const fn duplicate_info(
                            array: &[(usize, &'static str); 1usize],
                        ) -> (bool, usize, usize) {
                            let len = 1usize;
                            let mut i = 0usize;
                            while i < len {
                                let mut j = i + 1;
                                while j < len {
                                    if array[i].0 == array[j].0 {
                                        return (true, i, j);
                                    }
                                    j += 1;
                                }
                                i += 1;
                            }
                            (false, 0, 0)
                        }
                        const DUP_INFO: (bool, usize, usize) = duplicate_info(&indices);
                        if DUP_INFO.0 {
                            let msg = ::const_format::pmr::__AssertStr {
                                x: {
                                    use ::const_format::__cf_osRcTFl4A;
                                    ({
                                        #[doc(hidden)]
                                        #[allow(unused_mut, non_snake_case)]
                                        const CONCATP_NHPMWYD3NJA: &[__cf_osRcTFl4A::pmr::PArgument] = {
                                            let fmt = __cf_osRcTFl4A::pmr::FormattingFlags::NEW;
                                            &[
                                                __cf_osRcTFl4A::pmr::PConvWrapper(
                                                        "Found variants that have duplicate indexes. Both `",
                                                    )
                                                    .to_pargument_display(fmt),
                                                __cf_osRcTFl4A::pmr::PConvWrapper(indices[DUP_INFO.1].1)
                                                    .to_pargument_display(fmt),
                                                __cf_osRcTFl4A::pmr::PConvWrapper("` and `")
                                                    .to_pargument_display(fmt),
                                                __cf_osRcTFl4A::pmr::PConvWrapper(indices[DUP_INFO.2].1)
                                                    .to_pargument_display(fmt),
                                                __cf_osRcTFl4A::pmr::PConvWrapper("` have the index `")
                                                    .to_pargument_display(fmt),
                                                __cf_osRcTFl4A::pmr::PConvWrapper(indices[DUP_INFO.1].0)
                                                    .to_pargument_display(fmt),
                                                __cf_osRcTFl4A::pmr::PConvWrapper(
                                                        "`. Use different indexes for each variant.",
                                                    )
                                                    .to_pargument_display(fmt),
                                            ]
                                        };
                                        {
                                            #[doc(hidden)]
                                            const ARR_LEN: usize = ::const_format::pmr::PArgument::calc_len(
                                                CONCATP_NHPMWYD3NJA,
                                            );
                                            #[doc(hidden)]
                                            const CONCAT_ARR: &::const_format::pmr::LenAndArray<
                                                [u8; ARR_LEN],
                                            > = &::const_format::pmr::__priv_concatenate(
                                                CONCATP_NHPMWYD3NJA,
                                            );
                                            #[doc(hidden)]
                                            #[allow(clippy::transmute_ptr_to_ptr)]
                                            const CONCAT_STR: &str = unsafe {
                                                let slice = ::const_format::pmr::transmute::<
                                                    &[u8; ARR_LEN],
                                                    &[u8; CONCAT_ARR.len],
                                                >(&CONCAT_ARR.array);
                                                {
                                                    let bytes: &'static [::const_format::pmr::u8] = slice;
                                                    let string: &'static ::const_format::pmr::str = {
                                                        ::const_format::__hidden_utils::PtrToRef {
                                                            ptr: bytes as *const [::const_format::pmr::u8] as *const str,
                                                        }
                                                            .reff
                                                    };
                                                    string
                                                }
                                            };
                                            CONCAT_STR
                                        }
                                    })
                                },
                            }
                                .x;
                            {
                                ::core::panicking::panic_display(&msg);
                            };
                        }
                    };
                    match __codec_input_edqy
                        .read_byte()
                        .map_err(|e| {
                            e
                                .chain(
                                    "Could not decode `ParametersValue`, failed to read variant byte",
                                )
                        })?
                    {
                        #[allow(clippy::unnecessary_cast)]
                        #[allow(clippy::cast_possible_truncation)]
                        __codec_x_edqy if __codec_x_edqy
                            == (0usize) as ::core::primitive::u8 => {
                            #[allow(clippy::redundant_closure_call)]
                            return (move || {
                                ::core::result::Result::Ok(
                                    ParametersValue::MinInflation({
                                        let __codec_res_edqy = <u32 as ::codec::Decode>::decode(
                                            __codec_input_edqy,
                                        );
                                        match __codec_res_edqy {
                                            ::core::result::Result::Err(e) => {
                                                return ::core::result::Result::Err(
                                                    e
                                                        .chain("Could not decode `ParametersValue::MinInflation.0`"),
                                                );
                                            }
                                            ::core::result::Result::Ok(__codec_res_edqy) => {
                                                __codec_res_edqy
                                            }
                                        }
                                    }),
                                )
                            })();
                        }
                        _ => {
                            #[allow(clippy::redundant_closure_call)]
                            return (move || {
                                ::core::result::Result::Err(
                                    <_ as ::core::convert::Into<
                                        _,
                                    >>::into(
                                        "Could not decode `ParametersValue`, variant doesn't exist",
                                    ),
                                )
                            })();
                        }
                    }
                }
            }
        };
        #[allow(deprecated)]
        const _: () = {
            fn check_struct() {
                fn check_field<T: ::codec::DecodeWithMemTracking>() {}
                check_field::<u32>();
            }
            #[automatically_derived]
            impl ::codec::DecodeWithMemTracking for ParametersValue {}
        };
        const _: () = {
            #[automatically_derived]
            impl ::codec::MaxEncodedLen for ParametersValue {
                fn max_encoded_len() -> ::core::primitive::usize {
                    0_usize
                        .max(
                            0_usize
                                .saturating_add(
                                    <u32 as ::codec::MaxEncodedLen>::max_encoded_len(),
                                ),
                        )
                        .saturating_add(1)
                }
            }
        };
        impl core::fmt::Debug for ParametersValue {
            fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
                match self {
                    Self::MinInflation(ref a0) => {
                        fmt.debug_tuple("ParametersValue::MinInflation")
                            .field(a0)
                            .finish()
                    }
                    _ => Ok(()),
                }
            }
        }
        #[allow(
            non_upper_case_globals,
            deprecated,
            unused_attributes,
            unused_qualifications
        )]
        const _: () = {
            impl ::scale_info::TypeInfo for ParametersValue {
                type Identity = Self;
                fn type_info() -> ::scale_info::Type {
                    ::scale_info::Type::builder()
                        .path(
                            ::scale_info::Path::new_with_replace(
                                "ParametersValue",
                                "polkadot_test::dynamic_params::inflationAV",
                                &[],
                            ),
                        )
                        .type_params(::alloc::vec::Vec::new())
                        .variant(
                            ::scale_info::build::Variants::new()
                                .variant(
                                    "MinInflation",
                                    |v| {
                                        v
                                            .index(0u8 as ::core::primitive::u8)
                                            .fields(
                                                ::scale_info::build::Fields::unnamed()
                                                    .field(|f| f.ty::<u32>().type_name("u32")),
                                            )
                                            .docs(
                                                &["Minimum inflation rate used to calculate era payouts."],
                                            )
                                    },
                                ),
                        )
                }
            }
        };
        impl polkadot_sdk::frame_support::traits::dynamic_params::AggregatedKeyValue
        for Parameters {
            type Key = ParametersKey;
            type Value = ParametersValue;
            fn into_parts(self) -> (Self::Key, Option<Self::Value>) {
                match self {
                    Parameters::MinInflation(key, value) => {
                        (
                            ParametersKey::MinInflation(key),
                            value.map(ParametersValue::MinInflation),
                        )
                    }
                }
            }
        }
        #[doc(hidden)]
        pub struct MinInflation;
        #[automatically_derived]
        impl ::core::clone::Clone for MinInflation {
            #[inline]
            fn clone(&self) -> MinInflation {
                MinInflation
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for MinInflation {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for MinInflation {
            #[inline]
            fn eq(&self, other: &MinInflation) -> bool {
                true
            }
        }
        #[automatically_derived]
        impl ::core::cmp::Eq for MinInflation {
            #[inline]
            #[doc(hidden)]
            #[coverage(off)]
            fn assert_receiver_is_total_eq(&self) -> () {}
        }
        #[allow(deprecated)]
        const _: () = {
            #[automatically_derived]
            impl ::codec::Encode for MinInflation {
                fn size_hint(&self) -> usize {
                    0_usize
                }
                fn encode_to<
                    __CodecOutputEdqy: ::codec::Output + ?::core::marker::Sized,
                >(&self, __codec_dest_edqy: &mut __CodecOutputEdqy) {}
            }
            #[automatically_derived]
            impl ::codec::EncodeLike for MinInflation {}
        };
        #[allow(deprecated)]
        const _: () = {
            #[automatically_derived]
            impl ::codec::Decode for MinInflation {
                fn decode<__CodecInputEdqy: ::codec::Input>(
                    __codec_input_edqy: &mut __CodecInputEdqy,
                ) -> ::core::result::Result<Self, ::codec::Error> {
                    ::core::result::Result::Ok(MinInflation)
                }
            }
        };
        #[allow(deprecated)]
        const _: () = {
            fn check_struct() {
                fn check_field<T: ::codec::DecodeWithMemTracking>() {}
            }
            #[automatically_derived]
            impl ::codec::DecodeWithMemTracking for MinInflation {}
        };
        const _: () = {
            #[automatically_derived]
            impl ::codec::MaxEncodedLen for MinInflation {
                fn max_encoded_len() -> ::core::primitive::usize {
                    0_usize
                }
            }
        };
        impl core::fmt::Debug for MinInflation {
            fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
                fmt.debug_tuple("MinInflation").finish()
            }
        }
        #[allow(
            non_upper_case_globals,
            deprecated,
            unused_attributes,
            unused_qualifications
        )]
        const _: () = {
            impl ::scale_info::TypeInfo for MinInflation {
                type Identity = Self;
                fn type_info() -> ::scale_info::Type {
                    ::scale_info::Type::builder()
                        .path(
                            ::scale_info::Path::new_with_replace(
                                "MinInflation",
                                "polkadot_test::dynamic_params::inflationAV",
                                &[],
                            ),
                        )
                        .type_params(::alloc::vec::Vec::new())
                        .composite(::scale_info::build::Fields::unit())
                }
            }
        };
        impl polkadot_sdk::frame_support::__private::Get<u32> for MinInflation {
            fn get() -> u32 {
                match <pallet_parameters::Parameters<
                    Runtime,
                > as polkadot_sdk::frame_support::storage::StorageMap<
                    RuntimeParametersKey,
                    RuntimeParametersValue,
                >>::get(
                    RuntimeParametersKey::InflationAV(
                        ParametersKey::MinInflation(MinInflation),
                    ),
                ) {
                    Some(
                        RuntimeParametersValue::InflationAV(
                            ParametersValue::MinInflation(inner),
                        ),
                    ) => inner,
                    Some(_) => {
                        {
                            {
                                {
                                    let lvl = ::log::Level::Error;
                                    if lvl <= ::log::STATIC_MAX_LEVEL
                                        && lvl <= ::log::max_level()
                                    {
                                        ::log::__private_api::log(
                                            { ::log::__private_api::GlobalLogger },
                                            format_args!(
                                                "{0}: {1:?}",
                                                ::frame_support::traits::DEFENSIVE_OP_PUBLIC_ERROR,
                                                "Unexpected value type at key - returning default",
                                            ),
                                            lvl,
                                            &(
                                                "runtime::defensive",
                                                "polkadot_test::dynamic_params::inflationAV",
                                                ::log::__private_api::loc(),
                                            ),
                                            (),
                                        );
                                    }
                                }
                            }
                        };
                        if true {
                            if !false {
                                {
                                    ::core::panicking::panic_fmt(
                                        format_args!(
                                            "{0}: {1:?}",
                                            ::frame_support::traits::DEFENSIVE_OP_INTERNAL_ERROR,
                                            "Unexpected value type at key - returning default",
                                        ),
                                    );
                                }
                            }
                        }
                        0
                    }
                    None => 0,
                }
            }
        }
        impl polkadot_sdk::frame_support::traits::dynamic_params::Key for MinInflation {
            type Value = u32;
            type WrappedValue = MinInflationValue;
        }
        impl From<MinInflation> for ParametersKey {
            fn from(key: MinInflation) -> Self {
                ParametersKey::MinInflation(key)
            }
        }
        impl TryFrom<ParametersKey> for MinInflation {
            type Error = ();
            fn try_from(key: ParametersKey) -> Result<Self, Self::Error> {
                match key {
                    ParametersKey::MinInflation(key) => Ok(key),
                    _ => Err(()),
                }
            }
        }
        #[doc(hidden)]
        pub struct MinInflationValue(pub u32);
        #[automatically_derived]
        impl ::core::clone::Clone for MinInflationValue {
            #[inline]
            fn clone(&self) -> MinInflationValue {
                MinInflationValue(::core::clone::Clone::clone(&self.0))
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for MinInflationValue {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for MinInflationValue {
            #[inline]
            fn eq(&self, other: &MinInflationValue) -> bool {
                self.0 == other.0
            }
        }
        #[automatically_derived]
        impl ::core::cmp::Eq for MinInflationValue {
            #[inline]
            #[doc(hidden)]
            #[coverage(off)]
            fn assert_receiver_is_total_eq(&self) -> () {
                let _: ::core::cmp::AssertParamIsEq<u32>;
            }
        }
        impl core::fmt::Debug for MinInflationValue {
            fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
                fmt.debug_tuple("MinInflationValue").field(&self.0).finish()
            }
        }
        impl From<MinInflationValue> for ParametersValue {
            fn from(value: MinInflationValue) -> Self {
                ParametersValue::MinInflation(value.0)
            }
        }
        impl From<(MinInflation, u32)> for Parameters {
            fn from((key, value): (MinInflation, u32)) -> Self {
                Parameters::MinInflation(key, Some(value))
            }
        }
        impl From<MinInflation> for Parameters {
            fn from(key: MinInflation) -> Self {
                Parameters::MinInflation(key, None)
            }
        }
        impl TryFrom<ParametersValue> for MinInflationValue {
            type Error = ();
            fn try_from(value: ParametersValue) -> Result<Self, Self::Error> {
                match value {
                    ParametersValue::MinInflation(value) => Ok(MinInflationValue(value)),
                    _ => Err(()),
                }
            }
        }
        impl From<MinInflationValue> for u32 {
            fn from(value: MinInflationValue) -> Self {
                value.0
            }
        }
    }
    pub mod inflation2FG {
        use super::*;
        #[doc(hidden)]
        pub enum Parameters {
            /// Minimum inflation rate used to calculate era payouts.
            #[codec(index = 0)]
            MinInflation2(MinInflation2, Option<u32>),
        }
        #[automatically_derived]
        impl ::core::clone::Clone for Parameters {
            #[inline]
            fn clone(&self) -> Parameters {
                match self {
                    Parameters::MinInflation2(__self_0, __self_1) => {
                        Parameters::MinInflation2(
                            ::core::clone::Clone::clone(__self_0),
                            ::core::clone::Clone::clone(__self_1),
                        )
                    }
                }
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for Parameters {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for Parameters {
            #[inline]
            fn eq(&self, other: &Parameters) -> bool {
                match (self, other) {
                    (
                        Parameters::MinInflation2(__self_0, __self_1),
                        Parameters::MinInflation2(__arg1_0, __arg1_1),
                    ) => __self_0 == __arg1_0 && __self_1 == __arg1_1,
                }
            }
        }
        #[automatically_derived]
        impl ::core::cmp::Eq for Parameters {
            #[inline]
            #[doc(hidden)]
            #[coverage(off)]
            fn assert_receiver_is_total_eq(&self) -> () {
                let _: ::core::cmp::AssertParamIsEq<MinInflation2>;
                let _: ::core::cmp::AssertParamIsEq<Option<u32>>;
            }
        }
        #[allow(deprecated)]
        const _: () = {
            #[automatically_derived]
            impl ::codec::Encode for Parameters {
                fn size_hint(&self) -> usize {
                    1_usize
                        + match *self {
                            Parameters::MinInflation2(ref aa, ref ba) => {
                                0_usize
                                    .saturating_add(::codec::Encode::size_hint(aa))
                                    .saturating_add(::codec::Encode::size_hint(ba))
                            }
                            _ => 0_usize,
                        }
                }
                fn encode_to<
                    __CodecOutputEdqy: ::codec::Output + ?::core::marker::Sized,
                >(&self, __codec_dest_edqy: &mut __CodecOutputEdqy) {
                    #[automatically_derived]
                    const _: () = {
                        #[allow(clippy::unnecessary_cast)]
                        #[allow(clippy::cast_possible_truncation)]
                        const indices: [(usize, &'static str); 1usize] = [
                            ((0usize) as ::core::primitive::usize, "MinInflation2"),
                        ];
                        const fn search_for_invalid_index(
                            array: &[(usize, &'static str); 1usize],
                        ) -> (bool, usize) {
                            let mut i = 0;
                            while i < 1usize {
                                if array[i].0 > 255 {
                                    return (true, i);
                                }
                                i += 1;
                            }
                            (false, 0)
                        }
                        const INVALID_INDEX: (bool, usize) = search_for_invalid_index(
                            &indices,
                        );
                        if INVALID_INDEX.0 {
                            let msg = ::const_format::pmr::__AssertStr {
                                x: {
                                    use ::const_format::__cf_osRcTFl4A;
                                    ({
                                        #[doc(hidden)]
                                        #[allow(unused_mut, non_snake_case)]
                                        const CONCATP_NHPMWYD3NJA: &[__cf_osRcTFl4A::pmr::PArgument] = {
                                            let fmt = __cf_osRcTFl4A::pmr::FormattingFlags::NEW;
                                            &[
                                                __cf_osRcTFl4A::pmr::PConvWrapper("Found variant `")
                                                    .to_pargument_display(fmt),
                                                __cf_osRcTFl4A::pmr::PConvWrapper(
                                                        indices[INVALID_INDEX.1].1,
                                                    )
                                                    .to_pargument_display(fmt),
                                                __cf_osRcTFl4A::pmr::PConvWrapper("` with invalid index: `")
                                                    .to_pargument_display(fmt),
                                                __cf_osRcTFl4A::pmr::PConvWrapper(
                                                        indices[INVALID_INDEX.1].0,
                                                    )
                                                    .to_pargument_display(fmt),
                                                __cf_osRcTFl4A::pmr::PConvWrapper(
                                                        "`. Max supported index is 255.",
                                                    )
                                                    .to_pargument_display(fmt),
                                            ]
                                        };
                                        {
                                            #[doc(hidden)]
                                            const ARR_LEN: usize = ::const_format::pmr::PArgument::calc_len(
                                                CONCATP_NHPMWYD3NJA,
                                            );
                                            #[doc(hidden)]
                                            const CONCAT_ARR: &::const_format::pmr::LenAndArray<
                                                [u8; ARR_LEN],
                                            > = &::const_format::pmr::__priv_concatenate(
                                                CONCATP_NHPMWYD3NJA,
                                            );
                                            #[doc(hidden)]
                                            #[allow(clippy::transmute_ptr_to_ptr)]
                                            const CONCAT_STR: &str = unsafe {
                                                let slice = ::const_format::pmr::transmute::<
                                                    &[u8; ARR_LEN],
                                                    &[u8; CONCAT_ARR.len],
                                                >(&CONCAT_ARR.array);
                                                {
                                                    let bytes: &'static [::const_format::pmr::u8] = slice;
                                                    let string: &'static ::const_format::pmr::str = {
                                                        ::const_format::__hidden_utils::PtrToRef {
                                                            ptr: bytes as *const [::const_format::pmr::u8] as *const str,
                                                        }
                                                            .reff
                                                    };
                                                    string
                                                }
                                            };
                                            CONCAT_STR
                                        }
                                    })
                                },
                            }
                                .x;
                            {
                                ::core::panicking::panic_display(&msg);
                            };
                        }
                        const fn duplicate_info(
                            array: &[(usize, &'static str); 1usize],
                        ) -> (bool, usize, usize) {
                            let len = 1usize;
                            let mut i = 0usize;
                            while i < len {
                                let mut j = i + 1;
                                while j < len {
                                    if array[i].0 == array[j].0 {
                                        return (true, i, j);
                                    }
                                    j += 1;
                                }
                                i += 1;
                            }
                            (false, 0, 0)
                        }
                        const DUP_INFO: (bool, usize, usize) = duplicate_info(&indices);
                        if DUP_INFO.0 {
                            let msg = ::const_format::pmr::__AssertStr {
                                x: {
                                    use ::const_format::__cf_osRcTFl4A;
                                    ({
                                        #[doc(hidden)]
                                        #[allow(unused_mut, non_snake_case)]
                                        const CONCATP_NHPMWYD3NJA: &[__cf_osRcTFl4A::pmr::PArgument] = {
                                            let fmt = __cf_osRcTFl4A::pmr::FormattingFlags::NEW;
                                            &[
                                                __cf_osRcTFl4A::pmr::PConvWrapper(
                                                        "Found variants that have duplicate indexes. Both `",
                                                    )
                                                    .to_pargument_display(fmt),
                                                __cf_osRcTFl4A::pmr::PConvWrapper(indices[DUP_INFO.1].1)
                                                    .to_pargument_display(fmt),
                                                __cf_osRcTFl4A::pmr::PConvWrapper("` and `")
                                                    .to_pargument_display(fmt),
                                                __cf_osRcTFl4A::pmr::PConvWrapper(indices[DUP_INFO.2].1)
                                                    .to_pargument_display(fmt),
                                                __cf_osRcTFl4A::pmr::PConvWrapper("` have the index `")
                                                    .to_pargument_display(fmt),
                                                __cf_osRcTFl4A::pmr::PConvWrapper(indices[DUP_INFO.1].0)
                                                    .to_pargument_display(fmt),
                                                __cf_osRcTFl4A::pmr::PConvWrapper(
                                                        "`. Use different indexes for each variant.",
                                                    )
                                                    .to_pargument_display(fmt),
                                            ]
                                        };
                                        {
                                            #[doc(hidden)]
                                            const ARR_LEN: usize = ::const_format::pmr::PArgument::calc_len(
                                                CONCATP_NHPMWYD3NJA,
                                            );
                                            #[doc(hidden)]
                                            const CONCAT_ARR: &::const_format::pmr::LenAndArray<
                                                [u8; ARR_LEN],
                                            > = &::const_format::pmr::__priv_concatenate(
                                                CONCATP_NHPMWYD3NJA,
                                            );
                                            #[doc(hidden)]
                                            #[allow(clippy::transmute_ptr_to_ptr)]
                                            const CONCAT_STR: &str = unsafe {
                                                let slice = ::const_format::pmr::transmute::<
                                                    &[u8; ARR_LEN],
                                                    &[u8; CONCAT_ARR.len],
                                                >(&CONCAT_ARR.array);
                                                {
                                                    let bytes: &'static [::const_format::pmr::u8] = slice;
                                                    let string: &'static ::const_format::pmr::str = {
                                                        ::const_format::__hidden_utils::PtrToRef {
                                                            ptr: bytes as *const [::const_format::pmr::u8] as *const str,
                                                        }
                                                            .reff
                                                    };
                                                    string
                                                }
                                            };
                                            CONCAT_STR
                                        }
                                    })
                                },
                            }
                                .x;
                            {
                                ::core::panicking::panic_display(&msg);
                            };
                        }
                    };
                    match *self {
                        Parameters::MinInflation2(ref aa, ref ba) => {
                            #[allow(clippy::unnecessary_cast)]
                            __codec_dest_edqy
                                .push_byte((0usize) as ::core::primitive::u8);
                            ::codec::Encode::encode_to(aa, __codec_dest_edqy);
                            ::codec::Encode::encode_to(ba, __codec_dest_edqy);
                        }
                        _ => {}
                    }
                }
            }
            #[automatically_derived]
            impl ::codec::EncodeLike for Parameters {}
        };
        #[allow(deprecated)]
        const _: () = {
            #[automatically_derived]
            impl ::codec::Decode for Parameters {
                fn decode<__CodecInputEdqy: ::codec::Input>(
                    __codec_input_edqy: &mut __CodecInputEdqy,
                ) -> ::core::result::Result<Self, ::codec::Error> {
                    #[automatically_derived]
                    const _: () = {
                        #[allow(clippy::unnecessary_cast)]
                        #[allow(clippy::cast_possible_truncation)]
                        const indices: [(usize, &'static str); 1usize] = [
                            ((0usize) as ::core::primitive::usize, "MinInflation2"),
                        ];
                        const fn search_for_invalid_index(
                            array: &[(usize, &'static str); 1usize],
                        ) -> (bool, usize) {
                            let mut i = 0;
                            while i < 1usize {
                                if array[i].0 > 255 {
                                    return (true, i);
                                }
                                i += 1;
                            }
                            (false, 0)
                        }
                        const INVALID_INDEX: (bool, usize) = search_for_invalid_index(
                            &indices,
                        );
                        if INVALID_INDEX.0 {
                            let msg = ::const_format::pmr::__AssertStr {
                                x: {
                                    use ::const_format::__cf_osRcTFl4A;
                                    ({
                                        #[doc(hidden)]
                                        #[allow(unused_mut, non_snake_case)]
                                        const CONCATP_NHPMWYD3NJA: &[__cf_osRcTFl4A::pmr::PArgument] = {
                                            let fmt = __cf_osRcTFl4A::pmr::FormattingFlags::NEW;
                                            &[
                                                __cf_osRcTFl4A::pmr::PConvWrapper("Found variant `")
                                                    .to_pargument_display(fmt),
                                                __cf_osRcTFl4A::pmr::PConvWrapper(
                                                        indices[INVALID_INDEX.1].1,
                                                    )
                                                    .to_pargument_display(fmt),
                                                __cf_osRcTFl4A::pmr::PConvWrapper("` with invalid index: `")
                                                    .to_pargument_display(fmt),
                                                __cf_osRcTFl4A::pmr::PConvWrapper(
                                                        indices[INVALID_INDEX.1].0,
                                                    )
                                                    .to_pargument_display(fmt),
                                                __cf_osRcTFl4A::pmr::PConvWrapper(
                                                        "`. Max supported index is 255.",
                                                    )
                                                    .to_pargument_display(fmt),
                                            ]
                                        };
                                        {
                                            #[doc(hidden)]
                                            const ARR_LEN: usize = ::const_format::pmr::PArgument::calc_len(
                                                CONCATP_NHPMWYD3NJA,
                                            );
                                            #[doc(hidden)]
                                            const CONCAT_ARR: &::const_format::pmr::LenAndArray<
                                                [u8; ARR_LEN],
                                            > = &::const_format::pmr::__priv_concatenate(
                                                CONCATP_NHPMWYD3NJA,
                                            );
                                            #[doc(hidden)]
                                            #[allow(clippy::transmute_ptr_to_ptr)]
                                            const CONCAT_STR: &str = unsafe {
                                                let slice = ::const_format::pmr::transmute::<
                                                    &[u8; ARR_LEN],
                                                    &[u8; CONCAT_ARR.len],
                                                >(&CONCAT_ARR.array);
                                                {
                                                    let bytes: &'static [::const_format::pmr::u8] = slice;
                                                    let string: &'static ::const_format::pmr::str = {
                                                        ::const_format::__hidden_utils::PtrToRef {
                                                            ptr: bytes as *const [::const_format::pmr::u8] as *const str,
                                                        }
                                                            .reff
                                                    };
                                                    string
                                                }
                                            };
                                            CONCAT_STR
                                        }
                                    })
                                },
                            }
                                .x;
                            {
                                ::core::panicking::panic_display(&msg);
                            };
                        }
                        const fn duplicate_info(
                            array: &[(usize, &'static str); 1usize],
                        ) -> (bool, usize, usize) {
                            let len = 1usize;
                            let mut i = 0usize;
                            while i < len {
                                let mut j = i + 1;
                                while j < len {
                                    if array[i].0 == array[j].0 {
                                        return (true, i, j);
                                    }
                                    j += 1;
                                }
                                i += 1;
                            }
                            (false, 0, 0)
                        }
                        const DUP_INFO: (bool, usize, usize) = duplicate_info(&indices);
                        if DUP_INFO.0 {
                            let msg = ::const_format::pmr::__AssertStr {
                                x: {
                                    use ::const_format::__cf_osRcTFl4A;
                                    ({
                                        #[doc(hidden)]
                                        #[allow(unused_mut, non_snake_case)]
                                        const CONCATP_NHPMWYD3NJA: &[__cf_osRcTFl4A::pmr::PArgument] = {
                                            let fmt = __cf_osRcTFl4A::pmr::FormattingFlags::NEW;
                                            &[
                                                __cf_osRcTFl4A::pmr::PConvWrapper(
                                                        "Found variants that have duplicate indexes. Both `",
                                                    )
                                                    .to_pargument_display(fmt),
                                                __cf_osRcTFl4A::pmr::PConvWrapper(indices[DUP_INFO.1].1)
                                                    .to_pargument_display(fmt),
                                                __cf_osRcTFl4A::pmr::PConvWrapper("` and `")
                                                    .to_pargument_display(fmt),
                                                __cf_osRcTFl4A::pmr::PConvWrapper(indices[DUP_INFO.2].1)
                                                    .to_pargument_display(fmt),
                                                __cf_osRcTFl4A::pmr::PConvWrapper("` have the index `")
                                                    .to_pargument_display(fmt),
                                                __cf_osRcTFl4A::pmr::PConvWrapper(indices[DUP_INFO.1].0)
                                                    .to_pargument_display(fmt),
                                                __cf_osRcTFl4A::pmr::PConvWrapper(
                                                        "`. Use different indexes for each variant.",
                                                    )
                                                    .to_pargument_display(fmt),
                                            ]
                                        };
                                        {
                                            #[doc(hidden)]
                                            const ARR_LEN: usize = ::const_format::pmr::PArgument::calc_len(
                                                CONCATP_NHPMWYD3NJA,
                                            );
                                            #[doc(hidden)]
                                            const CONCAT_ARR: &::const_format::pmr::LenAndArray<
                                                [u8; ARR_LEN],
                                            > = &::const_format::pmr::__priv_concatenate(
                                                CONCATP_NHPMWYD3NJA,
                                            );
                                            #[doc(hidden)]
                                            #[allow(clippy::transmute_ptr_to_ptr)]
                                            const CONCAT_STR: &str = unsafe {
                                                let slice = ::const_format::pmr::transmute::<
                                                    &[u8; ARR_LEN],
                                                    &[u8; CONCAT_ARR.len],
                                                >(&CONCAT_ARR.array);
                                                {
                                                    let bytes: &'static [::const_format::pmr::u8] = slice;
                                                    let string: &'static ::const_format::pmr::str = {
                                                        ::const_format::__hidden_utils::PtrToRef {
                                                            ptr: bytes as *const [::const_format::pmr::u8] as *const str,
                                                        }
                                                            .reff
                                                    };
                                                    string
                                                }
                                            };
                                            CONCAT_STR
                                        }
                                    })
                                },
                            }
                                .x;
                            {
                                ::core::panicking::panic_display(&msg);
                            };
                        }
                    };
                    match __codec_input_edqy
                        .read_byte()
                        .map_err(|e| {
                            e
                                .chain(
                                    "Could not decode `Parameters`, failed to read variant byte",
                                )
                        })?
                    {
                        #[allow(clippy::unnecessary_cast)]
                        #[allow(clippy::cast_possible_truncation)]
                        __codec_x_edqy if __codec_x_edqy
                            == (0usize) as ::core::primitive::u8 => {
                            #[allow(clippy::redundant_closure_call)]
                            return (move || {
                                ::core::result::Result::Ok(
                                    Parameters::MinInflation2(
                                        {
                                            let __codec_res_edqy = <MinInflation2 as ::codec::Decode>::decode(
                                                __codec_input_edqy,
                                            );
                                            match __codec_res_edqy {
                                                ::core::result::Result::Err(e) => {
                                                    return ::core::result::Result::Err(
                                                        e.chain("Could not decode `Parameters::MinInflation2.0`"),
                                                    );
                                                }
                                                ::core::result::Result::Ok(__codec_res_edqy) => {
                                                    __codec_res_edqy
                                                }
                                            }
                                        },
                                        {
                                            let __codec_res_edqy = <Option<
                                                u32,
                                            > as ::codec::Decode>::decode(__codec_input_edqy);
                                            match __codec_res_edqy {
                                                ::core::result::Result::Err(e) => {
                                                    return ::core::result::Result::Err(
                                                        e.chain("Could not decode `Parameters::MinInflation2.1`"),
                                                    );
                                                }
                                                ::core::result::Result::Ok(__codec_res_edqy) => {
                                                    __codec_res_edqy
                                                }
                                            }
                                        },
                                    ),
                                )
                            })();
                        }
                        _ => {
                            #[allow(clippy::redundant_closure_call)]
                            return (move || {
                                ::core::result::Result::Err(
                                    <_ as ::core::convert::Into<
                                        _,
                                    >>::into(
                                        "Could not decode `Parameters`, variant doesn't exist",
                                    ),
                                )
                            })();
                        }
                    }
                }
            }
        };
        #[allow(deprecated)]
        const _: () = {
            fn check_struct() {
                fn check_field<T: ::codec::DecodeWithMemTracking>() {}
                check_field::<MinInflation2>();
                check_field::<Option<u32>>();
            }
            #[automatically_derived]
            impl ::codec::DecodeWithMemTracking for Parameters {}
        };
        const _: () = {
            #[automatically_derived]
            impl ::codec::MaxEncodedLen for Parameters {
                fn max_encoded_len() -> ::core::primitive::usize {
                    0_usize
                        .max(
                            0_usize
                                .saturating_add(
                                    <MinInflation2 as ::codec::MaxEncodedLen>::max_encoded_len(),
                                )
                                .saturating_add(
                                    <Option<u32> as ::codec::MaxEncodedLen>::max_encoded_len(),
                                ),
                        )
                        .saturating_add(1)
                }
            }
        };
        impl core::fmt::Debug for Parameters {
            fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
                match self {
                    Self::MinInflation2(ref a0, ref a1) => {
                        fmt.debug_tuple("Parameters::MinInflation2")
                            .field(a0)
                            .field(a1)
                            .finish()
                    }
                    _ => Ok(()),
                }
            }
        }
        #[allow(
            non_upper_case_globals,
            deprecated,
            unused_attributes,
            unused_qualifications
        )]
        const _: () = {
            impl ::scale_info::TypeInfo for Parameters {
                type Identity = Self;
                fn type_info() -> ::scale_info::Type {
                    ::scale_info::Type::builder()
                        .path(
                            ::scale_info::Path::new_with_replace(
                                "Parameters",
                                "polkadot_test::dynamic_params::inflation2FG",
                                &[],
                            ),
                        )
                        .type_params(::alloc::vec::Vec::new())
                        .variant(
                            ::scale_info::build::Variants::new()
                                .variant(
                                    "MinInflation2",
                                    |v| {
                                        v
                                            .index(0u8 as ::core::primitive::u8)
                                            .fields(
                                                ::scale_info::build::Fields::unnamed()
                                                    .field(|f| {
                                                        f.ty::<MinInflation2>().type_name("MinInflation2")
                                                    })
                                                    .field(|f| f.ty::<Option<u32>>().type_name("Option<u32>")),
                                            )
                                            .docs(
                                                &["Minimum inflation rate used to calculate era payouts."],
                                            )
                                    },
                                ),
                        )
                }
            }
        };
        #[doc(hidden)]
        pub enum ParametersKey {
            /// Minimum inflation rate used to calculate era payouts.
            #[codec(index = 0)]
            MinInflation2(MinInflation2),
        }
        #[automatically_derived]
        impl ::core::clone::Clone for ParametersKey {
            #[inline]
            fn clone(&self) -> ParametersKey {
                match self {
                    ParametersKey::MinInflation2(__self_0) => {
                        ParametersKey::MinInflation2(
                            ::core::clone::Clone::clone(__self_0),
                        )
                    }
                }
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for ParametersKey {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for ParametersKey {
            #[inline]
            fn eq(&self, other: &ParametersKey) -> bool {
                match (self, other) {
                    (
                        ParametersKey::MinInflation2(__self_0),
                        ParametersKey::MinInflation2(__arg1_0),
                    ) => __self_0 == __arg1_0,
                }
            }
        }
        #[automatically_derived]
        impl ::core::cmp::Eq for ParametersKey {
            #[inline]
            #[doc(hidden)]
            #[coverage(off)]
            fn assert_receiver_is_total_eq(&self) -> () {
                let _: ::core::cmp::AssertParamIsEq<MinInflation2>;
            }
        }
        #[allow(deprecated)]
        const _: () = {
            #[automatically_derived]
            impl ::codec::Encode for ParametersKey {
                fn size_hint(&self) -> usize {
                    1_usize
                        + match *self {
                            ParametersKey::MinInflation2(ref aa) => {
                                0_usize.saturating_add(::codec::Encode::size_hint(aa))
                            }
                            _ => 0_usize,
                        }
                }
                fn encode_to<
                    __CodecOutputEdqy: ::codec::Output + ?::core::marker::Sized,
                >(&self, __codec_dest_edqy: &mut __CodecOutputEdqy) {
                    #[automatically_derived]
                    const _: () = {
                        #[allow(clippy::unnecessary_cast)]
                        #[allow(clippy::cast_possible_truncation)]
                        const indices: [(usize, &'static str); 1usize] = [
                            ((0usize) as ::core::primitive::usize, "MinInflation2"),
                        ];
                        const fn search_for_invalid_index(
                            array: &[(usize, &'static str); 1usize],
                        ) -> (bool, usize) {
                            let mut i = 0;
                            while i < 1usize {
                                if array[i].0 > 255 {
                                    return (true, i);
                                }
                                i += 1;
                            }
                            (false, 0)
                        }
                        const INVALID_INDEX: (bool, usize) = search_for_invalid_index(
                            &indices,
                        );
                        if INVALID_INDEX.0 {
                            let msg = ::const_format::pmr::__AssertStr {
                                x: {
                                    use ::const_format::__cf_osRcTFl4A;
                                    ({
                                        #[doc(hidden)]
                                        #[allow(unused_mut, non_snake_case)]
                                        const CONCATP_NHPMWYD3NJA: &[__cf_osRcTFl4A::pmr::PArgument] = {
                                            let fmt = __cf_osRcTFl4A::pmr::FormattingFlags::NEW;
                                            &[
                                                __cf_osRcTFl4A::pmr::PConvWrapper("Found variant `")
                                                    .to_pargument_display(fmt),
                                                __cf_osRcTFl4A::pmr::PConvWrapper(
                                                        indices[INVALID_INDEX.1].1,
                                                    )
                                                    .to_pargument_display(fmt),
                                                __cf_osRcTFl4A::pmr::PConvWrapper("` with invalid index: `")
                                                    .to_pargument_display(fmt),
                                                __cf_osRcTFl4A::pmr::PConvWrapper(
                                                        indices[INVALID_INDEX.1].0,
                                                    )
                                                    .to_pargument_display(fmt),
                                                __cf_osRcTFl4A::pmr::PConvWrapper(
                                                        "`. Max supported index is 255.",
                                                    )
                                                    .to_pargument_display(fmt),
                                            ]
                                        };
                                        {
                                            #[doc(hidden)]
                                            const ARR_LEN: usize = ::const_format::pmr::PArgument::calc_len(
                                                CONCATP_NHPMWYD3NJA,
                                            );
                                            #[doc(hidden)]
                                            const CONCAT_ARR: &::const_format::pmr::LenAndArray<
                                                [u8; ARR_LEN],
                                            > = &::const_format::pmr::__priv_concatenate(
                                                CONCATP_NHPMWYD3NJA,
                                            );
                                            #[doc(hidden)]
                                            #[allow(clippy::transmute_ptr_to_ptr)]
                                            const CONCAT_STR: &str = unsafe {
                                                let slice = ::const_format::pmr::transmute::<
                                                    &[u8; ARR_LEN],
                                                    &[u8; CONCAT_ARR.len],
                                                >(&CONCAT_ARR.array);
                                                {
                                                    let bytes: &'static [::const_format::pmr::u8] = slice;
                                                    let string: &'static ::const_format::pmr::str = {
                                                        ::const_format::__hidden_utils::PtrToRef {
                                                            ptr: bytes as *const [::const_format::pmr::u8] as *const str,
                                                        }
                                                            .reff
                                                    };
                                                    string
                                                }
                                            };
                                            CONCAT_STR
                                        }
                                    })
                                },
                            }
                                .x;
                            {
                                ::core::panicking::panic_display(&msg);
                            };
                        }
                        const fn duplicate_info(
                            array: &[(usize, &'static str); 1usize],
                        ) -> (bool, usize, usize) {
                            let len = 1usize;
                            let mut i = 0usize;
                            while i < len {
                                let mut j = i + 1;
                                while j < len {
                                    if array[i].0 == array[j].0 {
                                        return (true, i, j);
                                    }
                                    j += 1;
                                }
                                i += 1;
                            }
                            (false, 0, 0)
                        }
                        const DUP_INFO: (bool, usize, usize) = duplicate_info(&indices);
                        if DUP_INFO.0 {
                            let msg = ::const_format::pmr::__AssertStr {
                                x: {
                                    use ::const_format::__cf_osRcTFl4A;
                                    ({
                                        #[doc(hidden)]
                                        #[allow(unused_mut, non_snake_case)]
                                        const CONCATP_NHPMWYD3NJA: &[__cf_osRcTFl4A::pmr::PArgument] = {
                                            let fmt = __cf_osRcTFl4A::pmr::FormattingFlags::NEW;
                                            &[
                                                __cf_osRcTFl4A::pmr::PConvWrapper(
                                                        "Found variants that have duplicate indexes. Both `",
                                                    )
                                                    .to_pargument_display(fmt),
                                                __cf_osRcTFl4A::pmr::PConvWrapper(indices[DUP_INFO.1].1)
                                                    .to_pargument_display(fmt),
                                                __cf_osRcTFl4A::pmr::PConvWrapper("` and `")
                                                    .to_pargument_display(fmt),
                                                __cf_osRcTFl4A::pmr::PConvWrapper(indices[DUP_INFO.2].1)
                                                    .to_pargument_display(fmt),
                                                __cf_osRcTFl4A::pmr::PConvWrapper("` have the index `")
                                                    .to_pargument_display(fmt),
                                                __cf_osRcTFl4A::pmr::PConvWrapper(indices[DUP_INFO.1].0)
                                                    .to_pargument_display(fmt),
                                                __cf_osRcTFl4A::pmr::PConvWrapper(
                                                        "`. Use different indexes for each variant.",
                                                    )
                                                    .to_pargument_display(fmt),
                                            ]
                                        };
                                        {
                                            #[doc(hidden)]
                                            const ARR_LEN: usize = ::const_format::pmr::PArgument::calc_len(
                                                CONCATP_NHPMWYD3NJA,
                                            );
                                            #[doc(hidden)]
                                            const CONCAT_ARR: &::const_format::pmr::LenAndArray<
                                                [u8; ARR_LEN],
                                            > = &::const_format::pmr::__priv_concatenate(
                                                CONCATP_NHPMWYD3NJA,
                                            );
                                            #[doc(hidden)]
                                            #[allow(clippy::transmute_ptr_to_ptr)]
                                            const CONCAT_STR: &str = unsafe {
                                                let slice = ::const_format::pmr::transmute::<
                                                    &[u8; ARR_LEN],
                                                    &[u8; CONCAT_ARR.len],
                                                >(&CONCAT_ARR.array);
                                                {
                                                    let bytes: &'static [::const_format::pmr::u8] = slice;
                                                    let string: &'static ::const_format::pmr::str = {
                                                        ::const_format::__hidden_utils::PtrToRef {
                                                            ptr: bytes as *const [::const_format::pmr::u8] as *const str,
                                                        }
                                                            .reff
                                                    };
                                                    string
                                                }
                                            };
                                            CONCAT_STR
                                        }
                                    })
                                },
                            }
                                .x;
                            {
                                ::core::panicking::panic_display(&msg);
                            };
                        }
                    };
                    match *self {
                        ParametersKey::MinInflation2(ref aa) => {
                            #[allow(clippy::unnecessary_cast)]
                            __codec_dest_edqy
                                .push_byte((0usize) as ::core::primitive::u8);
                            ::codec::Encode::encode_to(aa, __codec_dest_edqy);
                        }
                        _ => {}
                    }
                }
            }
            #[automatically_derived]
            impl ::codec::EncodeLike for ParametersKey {}
        };
        #[allow(deprecated)]
        const _: () = {
            #[automatically_derived]
            impl ::codec::Decode for ParametersKey {
                fn decode<__CodecInputEdqy: ::codec::Input>(
                    __codec_input_edqy: &mut __CodecInputEdqy,
                ) -> ::core::result::Result<Self, ::codec::Error> {
                    #[automatically_derived]
                    const _: () = {
                        #[allow(clippy::unnecessary_cast)]
                        #[allow(clippy::cast_possible_truncation)]
                        const indices: [(usize, &'static str); 1usize] = [
                            ((0usize) as ::core::primitive::usize, "MinInflation2"),
                        ];
                        const fn search_for_invalid_index(
                            array: &[(usize, &'static str); 1usize],
                        ) -> (bool, usize) {
                            let mut i = 0;
                            while i < 1usize {
                                if array[i].0 > 255 {
                                    return (true, i);
                                }
                                i += 1;
                            }
                            (false, 0)
                        }
                        const INVALID_INDEX: (bool, usize) = search_for_invalid_index(
                            &indices,
                        );
                        if INVALID_INDEX.0 {
                            let msg = ::const_format::pmr::__AssertStr {
                                x: {
                                    use ::const_format::__cf_osRcTFl4A;
                                    ({
                                        #[doc(hidden)]
                                        #[allow(unused_mut, non_snake_case)]
                                        const CONCATP_NHPMWYD3NJA: &[__cf_osRcTFl4A::pmr::PArgument] = {
                                            let fmt = __cf_osRcTFl4A::pmr::FormattingFlags::NEW;
                                            &[
                                                __cf_osRcTFl4A::pmr::PConvWrapper("Found variant `")
                                                    .to_pargument_display(fmt),
                                                __cf_osRcTFl4A::pmr::PConvWrapper(
                                                        indices[INVALID_INDEX.1].1,
                                                    )
                                                    .to_pargument_display(fmt),
                                                __cf_osRcTFl4A::pmr::PConvWrapper("` with invalid index: `")
                                                    .to_pargument_display(fmt),
                                                __cf_osRcTFl4A::pmr::PConvWrapper(
                                                        indices[INVALID_INDEX.1].0,
                                                    )
                                                    .to_pargument_display(fmt),
                                                __cf_osRcTFl4A::pmr::PConvWrapper(
                                                        "`. Max supported index is 255.",
                                                    )
                                                    .to_pargument_display(fmt),
                                            ]
                                        };
                                        {
                                            #[doc(hidden)]
                                            const ARR_LEN: usize = ::const_format::pmr::PArgument::calc_len(
                                                CONCATP_NHPMWYD3NJA,
                                            );
                                            #[doc(hidden)]
                                            const CONCAT_ARR: &::const_format::pmr::LenAndArray<
                                                [u8; ARR_LEN],
                                            > = &::const_format::pmr::__priv_concatenate(
                                                CONCATP_NHPMWYD3NJA,
                                            );
                                            #[doc(hidden)]
                                            #[allow(clippy::transmute_ptr_to_ptr)]
                                            const CONCAT_STR: &str = unsafe {
                                                let slice = ::const_format::pmr::transmute::<
                                                    &[u8; ARR_LEN],
                                                    &[u8; CONCAT_ARR.len],
                                                >(&CONCAT_ARR.array);
                                                {
                                                    let bytes: &'static [::const_format::pmr::u8] = slice;
                                                    let string: &'static ::const_format::pmr::str = {
                                                        ::const_format::__hidden_utils::PtrToRef {
                                                            ptr: bytes as *const [::const_format::pmr::u8] as *const str,
                                                        }
                                                            .reff
                                                    };
                                                    string
                                                }
                                            };
                                            CONCAT_STR
                                        }
                                    })
                                },
                            }
                                .x;
                            {
                                ::core::panicking::panic_display(&msg);
                            };
                        }
                        const fn duplicate_info(
                            array: &[(usize, &'static str); 1usize],
                        ) -> (bool, usize, usize) {
                            let len = 1usize;
                            let mut i = 0usize;
                            while i < len {
                                let mut j = i + 1;
                                while j < len {
                                    if array[i].0 == array[j].0 {
                                        return (true, i, j);
                                    }
                                    j += 1;
                                }
                                i += 1;
                            }
                            (false, 0, 0)
                        }
                        const DUP_INFO: (bool, usize, usize) = duplicate_info(&indices);
                        if DUP_INFO.0 {
                            let msg = ::const_format::pmr::__AssertStr {
                                x: {
                                    use ::const_format::__cf_osRcTFl4A;
                                    ({
                                        #[doc(hidden)]
                                        #[allow(unused_mut, non_snake_case)]
                                        const CONCATP_NHPMWYD3NJA: &[__cf_osRcTFl4A::pmr::PArgument] = {
                                            let fmt = __cf_osRcTFl4A::pmr::FormattingFlags::NEW;
                                            &[
                                                __cf_osRcTFl4A::pmr::PConvWrapper(
                                                        "Found variants that have duplicate indexes. Both `",
                                                    )
                                                    .to_pargument_display(fmt),
                                                __cf_osRcTFl4A::pmr::PConvWrapper(indices[DUP_INFO.1].1)
                                                    .to_pargument_display(fmt),
                                                __cf_osRcTFl4A::pmr::PConvWrapper("` and `")
                                                    .to_pargument_display(fmt),
                                                __cf_osRcTFl4A::pmr::PConvWrapper(indices[DUP_INFO.2].1)
                                                    .to_pargument_display(fmt),
                                                __cf_osRcTFl4A::pmr::PConvWrapper("` have the index `")
                                                    .to_pargument_display(fmt),
                                                __cf_osRcTFl4A::pmr::PConvWrapper(indices[DUP_INFO.1].0)
                                                    .to_pargument_display(fmt),
                                                __cf_osRcTFl4A::pmr::PConvWrapper(
                                                        "`. Use different indexes for each variant.",
                                                    )
                                                    .to_pargument_display(fmt),
                                            ]
                                        };
                                        {
                                            #[doc(hidden)]
                                            const ARR_LEN: usize = ::const_format::pmr::PArgument::calc_len(
                                                CONCATP_NHPMWYD3NJA,
                                            );
                                            #[doc(hidden)]
                                            const CONCAT_ARR: &::const_format::pmr::LenAndArray<
                                                [u8; ARR_LEN],
                                            > = &::const_format::pmr::__priv_concatenate(
                                                CONCATP_NHPMWYD3NJA,
                                            );
                                            #[doc(hidden)]
                                            #[allow(clippy::transmute_ptr_to_ptr)]
                                            const CONCAT_STR: &str = unsafe {
                                                let slice = ::const_format::pmr::transmute::<
                                                    &[u8; ARR_LEN],
                                                    &[u8; CONCAT_ARR.len],
                                                >(&CONCAT_ARR.array);
                                                {
                                                    let bytes: &'static [::const_format::pmr::u8] = slice;
                                                    let string: &'static ::const_format::pmr::str = {
                                                        ::const_format::__hidden_utils::PtrToRef {
                                                            ptr: bytes as *const [::const_format::pmr::u8] as *const str,
                                                        }
                                                            .reff
                                                    };
                                                    string
                                                }
                                            };
                                            CONCAT_STR
                                        }
                                    })
                                },
                            }
                                .x;
                            {
                                ::core::panicking::panic_display(&msg);
                            };
                        }
                    };
                    match __codec_input_edqy
                        .read_byte()
                        .map_err(|e| {
                            e
                                .chain(
                                    "Could not decode `ParametersKey`, failed to read variant byte",
                                )
                        })?
                    {
                        #[allow(clippy::unnecessary_cast)]
                        #[allow(clippy::cast_possible_truncation)]
                        __codec_x_edqy if __codec_x_edqy
                            == (0usize) as ::core::primitive::u8 => {
                            #[allow(clippy::redundant_closure_call)]
                            return (move || {
                                ::core::result::Result::Ok(
                                    ParametersKey::MinInflation2({
                                        let __codec_res_edqy = <MinInflation2 as ::codec::Decode>::decode(
                                            __codec_input_edqy,
                                        );
                                        match __codec_res_edqy {
                                            ::core::result::Result::Err(e) => {
                                                return ::core::result::Result::Err(
                                                    e.chain("Could not decode `ParametersKey::MinInflation2.0`"),
                                                );
                                            }
                                            ::core::result::Result::Ok(__codec_res_edqy) => {
                                                __codec_res_edqy
                                            }
                                        }
                                    }),
                                )
                            })();
                        }
                        _ => {
                            #[allow(clippy::redundant_closure_call)]
                            return (move || {
                                ::core::result::Result::Err(
                                    <_ as ::core::convert::Into<
                                        _,
                                    >>::into(
                                        "Could not decode `ParametersKey`, variant doesn't exist",
                                    ),
                                )
                            })();
                        }
                    }
                }
            }
        };
        #[allow(deprecated)]
        const _: () = {
            fn check_struct() {
                fn check_field<T: ::codec::DecodeWithMemTracking>() {}
                check_field::<MinInflation2>();
            }
            #[automatically_derived]
            impl ::codec::DecodeWithMemTracking for ParametersKey {}
        };
        const _: () = {
            #[automatically_derived]
            impl ::codec::MaxEncodedLen for ParametersKey {
                fn max_encoded_len() -> ::core::primitive::usize {
                    0_usize
                        .max(
                            0_usize
                                .saturating_add(
                                    <MinInflation2 as ::codec::MaxEncodedLen>::max_encoded_len(),
                                ),
                        )
                        .saturating_add(1)
                }
            }
        };
        impl core::fmt::Debug for ParametersKey {
            fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
                match self {
                    Self::MinInflation2(ref a0) => {
                        fmt.debug_tuple("ParametersKey::MinInflation2")
                            .field(a0)
                            .finish()
                    }
                    _ => Ok(()),
                }
            }
        }
        #[allow(
            non_upper_case_globals,
            deprecated,
            unused_attributes,
            unused_qualifications
        )]
        const _: () = {
            impl ::scale_info::TypeInfo for ParametersKey {
                type Identity = Self;
                fn type_info() -> ::scale_info::Type {
                    ::scale_info::Type::builder()
                        .path(
                            ::scale_info::Path::new_with_replace(
                                "ParametersKey",
                                "polkadot_test::dynamic_params::inflation2FG",
                                &[],
                            ),
                        )
                        .type_params(::alloc::vec::Vec::new())
                        .variant(
                            ::scale_info::build::Variants::new()
                                .variant(
                                    "MinInflation2",
                                    |v| {
                                        v
                                            .index(0u8 as ::core::primitive::u8)
                                            .fields(
                                                ::scale_info::build::Fields::unnamed()
                                                    .field(|f| {
                                                        f.ty::<MinInflation2>().type_name("MinInflation2")
                                                    }),
                                            )
                                            .docs(
                                                &["Minimum inflation rate used to calculate era payouts."],
                                            )
                                    },
                                ),
                        )
                }
            }
        };
        #[doc(hidden)]
        pub enum ParametersValue {
            /// Minimum inflation rate used to calculate era payouts.
            #[codec(index = 0)]
            MinInflation2(u32),
        }
        #[automatically_derived]
        impl ::core::clone::Clone for ParametersValue {
            #[inline]
            fn clone(&self) -> ParametersValue {
                match self {
                    ParametersValue::MinInflation2(__self_0) => {
                        ParametersValue::MinInflation2(
                            ::core::clone::Clone::clone(__self_0),
                        )
                    }
                }
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for ParametersValue {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for ParametersValue {
            #[inline]
            fn eq(&self, other: &ParametersValue) -> bool {
                match (self, other) {
                    (
                        ParametersValue::MinInflation2(__self_0),
                        ParametersValue::MinInflation2(__arg1_0),
                    ) => __self_0 == __arg1_0,
                }
            }
        }
        #[automatically_derived]
        impl ::core::cmp::Eq for ParametersValue {
            #[inline]
            #[doc(hidden)]
            #[coverage(off)]
            fn assert_receiver_is_total_eq(&self) -> () {
                let _: ::core::cmp::AssertParamIsEq<u32>;
            }
        }
        #[allow(deprecated)]
        const _: () = {
            #[automatically_derived]
            impl ::codec::Encode for ParametersValue {
                fn size_hint(&self) -> usize {
                    1_usize
                        + match *self {
                            ParametersValue::MinInflation2(ref aa) => {
                                0_usize.saturating_add(::codec::Encode::size_hint(aa))
                            }
                            _ => 0_usize,
                        }
                }
                fn encode_to<
                    __CodecOutputEdqy: ::codec::Output + ?::core::marker::Sized,
                >(&self, __codec_dest_edqy: &mut __CodecOutputEdqy) {
                    #[automatically_derived]
                    const _: () = {
                        #[allow(clippy::unnecessary_cast)]
                        #[allow(clippy::cast_possible_truncation)]
                        const indices: [(usize, &'static str); 1usize] = [
                            ((0usize) as ::core::primitive::usize, "MinInflation2"),
                        ];
                        const fn search_for_invalid_index(
                            array: &[(usize, &'static str); 1usize],
                        ) -> (bool, usize) {
                            let mut i = 0;
                            while i < 1usize {
                                if array[i].0 > 255 {
                                    return (true, i);
                                }
                                i += 1;
                            }
                            (false, 0)
                        }
                        const INVALID_INDEX: (bool, usize) = search_for_invalid_index(
                            &indices,
                        );
                        if INVALID_INDEX.0 {
                            let msg = ::const_format::pmr::__AssertStr {
                                x: {
                                    use ::const_format::__cf_osRcTFl4A;
                                    ({
                                        #[doc(hidden)]
                                        #[allow(unused_mut, non_snake_case)]
                                        const CONCATP_NHPMWYD3NJA: &[__cf_osRcTFl4A::pmr::PArgument] = {
                                            let fmt = __cf_osRcTFl4A::pmr::FormattingFlags::NEW;
                                            &[
                                                __cf_osRcTFl4A::pmr::PConvWrapper("Found variant `")
                                                    .to_pargument_display(fmt),
                                                __cf_osRcTFl4A::pmr::PConvWrapper(
                                                        indices[INVALID_INDEX.1].1,
                                                    )
                                                    .to_pargument_display(fmt),
                                                __cf_osRcTFl4A::pmr::PConvWrapper("` with invalid index: `")
                                                    .to_pargument_display(fmt),
                                                __cf_osRcTFl4A::pmr::PConvWrapper(
                                                        indices[INVALID_INDEX.1].0,
                                                    )
                                                    .to_pargument_display(fmt),
                                                __cf_osRcTFl4A::pmr::PConvWrapper(
                                                        "`. Max supported index is 255.",
                                                    )
                                                    .to_pargument_display(fmt),
                                            ]
                                        };
                                        {
                                            #[doc(hidden)]
                                            const ARR_LEN: usize = ::const_format::pmr::PArgument::calc_len(
                                                CONCATP_NHPMWYD3NJA,
                                            );
                                            #[doc(hidden)]
                                            const CONCAT_ARR: &::const_format::pmr::LenAndArray<
                                                [u8; ARR_LEN],
                                            > = &::const_format::pmr::__priv_concatenate(
                                                CONCATP_NHPMWYD3NJA,
                                            );
                                            #[doc(hidden)]
                                            #[allow(clippy::transmute_ptr_to_ptr)]
                                            const CONCAT_STR: &str = unsafe {
                                                let slice = ::const_format::pmr::transmute::<
                                                    &[u8; ARR_LEN],
                                                    &[u8; CONCAT_ARR.len],
                                                >(&CONCAT_ARR.array);
                                                {
                                                    let bytes: &'static [::const_format::pmr::u8] = slice;
                                                    let string: &'static ::const_format::pmr::str = {
                                                        ::const_format::__hidden_utils::PtrToRef {
                                                            ptr: bytes as *const [::const_format::pmr::u8] as *const str,
                                                        }
                                                            .reff
                                                    };
                                                    string
                                                }
                                            };
                                            CONCAT_STR
                                        }
                                    })
                                },
                            }
                                .x;
                            {
                                ::core::panicking::panic_display(&msg);
                            };
                        }
                        const fn duplicate_info(
                            array: &[(usize, &'static str); 1usize],
                        ) -> (bool, usize, usize) {
                            let len = 1usize;
                            let mut i = 0usize;
                            while i < len {
                                let mut j = i + 1;
                                while j < len {
                                    if array[i].0 == array[j].0 {
                                        return (true, i, j);
                                    }
                                    j += 1;
                                }
                                i += 1;
                            }
                            (false, 0, 0)
                        }
                        const DUP_INFO: (bool, usize, usize) = duplicate_info(&indices);
                        if DUP_INFO.0 {
                            let msg = ::const_format::pmr::__AssertStr {
                                x: {
                                    use ::const_format::__cf_osRcTFl4A;
                                    ({
                                        #[doc(hidden)]
                                        #[allow(unused_mut, non_snake_case)]
                                        const CONCATP_NHPMWYD3NJA: &[__cf_osRcTFl4A::pmr::PArgument] = {
                                            let fmt = __cf_osRcTFl4A::pmr::FormattingFlags::NEW;
                                            &[
                                                __cf_osRcTFl4A::pmr::PConvWrapper(
                                                        "Found variants that have duplicate indexes. Both `",
                                                    )
                                                    .to_pargument_display(fmt),
                                                __cf_osRcTFl4A::pmr::PConvWrapper(indices[DUP_INFO.1].1)
                                                    .to_pargument_display(fmt),
                                                __cf_osRcTFl4A::pmr::PConvWrapper("` and `")
                                                    .to_pargument_display(fmt),
                                                __cf_osRcTFl4A::pmr::PConvWrapper(indices[DUP_INFO.2].1)
                                                    .to_pargument_display(fmt),
                                                __cf_osRcTFl4A::pmr::PConvWrapper("` have the index `")
                                                    .to_pargument_display(fmt),
                                                __cf_osRcTFl4A::pmr::PConvWrapper(indices[DUP_INFO.1].0)
                                                    .to_pargument_display(fmt),
                                                __cf_osRcTFl4A::pmr::PConvWrapper(
                                                        "`. Use different indexes for each variant.",
                                                    )
                                                    .to_pargument_display(fmt),
                                            ]
                                        };
                                        {
                                            #[doc(hidden)]
                                            const ARR_LEN: usize = ::const_format::pmr::PArgument::calc_len(
                                                CONCATP_NHPMWYD3NJA,
                                            );
                                            #[doc(hidden)]
                                            const CONCAT_ARR: &::const_format::pmr::LenAndArray<
                                                [u8; ARR_LEN],
                                            > = &::const_format::pmr::__priv_concatenate(
                                                CONCATP_NHPMWYD3NJA,
                                            );
                                            #[doc(hidden)]
                                            #[allow(clippy::transmute_ptr_to_ptr)]
                                            const CONCAT_STR: &str = unsafe {
                                                let slice = ::const_format::pmr::transmute::<
                                                    &[u8; ARR_LEN],
                                                    &[u8; CONCAT_ARR.len],
                                                >(&CONCAT_ARR.array);
                                                {
                                                    let bytes: &'static [::const_format::pmr::u8] = slice;
                                                    let string: &'static ::const_format::pmr::str = {
                                                        ::const_format::__hidden_utils::PtrToRef {
                                                            ptr: bytes as *const [::const_format::pmr::u8] as *const str,
                                                        }
                                                            .reff
                                                    };
                                                    string
                                                }
                                            };
                                            CONCAT_STR
                                        }
                                    })
                                },
                            }
                                .x;
                            {
                                ::core::panicking::panic_display(&msg);
                            };
                        }
                    };
                    match *self {
                        ParametersValue::MinInflation2(ref aa) => {
                            #[allow(clippy::unnecessary_cast)]
                            __codec_dest_edqy
                                .push_byte((0usize) as ::core::primitive::u8);
                            ::codec::Encode::encode_to(aa, __codec_dest_edqy);
                        }
                        _ => {}
                    }
                }
            }
            #[automatically_derived]
            impl ::codec::EncodeLike for ParametersValue {}
        };
        #[allow(deprecated)]
        const _: () = {
            #[automatically_derived]
            impl ::codec::Decode for ParametersValue {
                fn decode<__CodecInputEdqy: ::codec::Input>(
                    __codec_input_edqy: &mut __CodecInputEdqy,
                ) -> ::core::result::Result<Self, ::codec::Error> {
                    #[automatically_derived]
                    const _: () = {
                        #[allow(clippy::unnecessary_cast)]
                        #[allow(clippy::cast_possible_truncation)]
                        const indices: [(usize, &'static str); 1usize] = [
                            ((0usize) as ::core::primitive::usize, "MinInflation2"),
                        ];
                        const fn search_for_invalid_index(
                            array: &[(usize, &'static str); 1usize],
                        ) -> (bool, usize) {
                            let mut i = 0;
                            while i < 1usize {
                                if array[i].0 > 255 {
                                    return (true, i);
                                }
                                i += 1;
                            }
                            (false, 0)
                        }
                        const INVALID_INDEX: (bool, usize) = search_for_invalid_index(
                            &indices,
                        );
                        if INVALID_INDEX.0 {
                            let msg = ::const_format::pmr::__AssertStr {
                                x: {
                                    use ::const_format::__cf_osRcTFl4A;
                                    ({
                                        #[doc(hidden)]
                                        #[allow(unused_mut, non_snake_case)]
                                        const CONCATP_NHPMWYD3NJA: &[__cf_osRcTFl4A::pmr::PArgument] = {
                                            let fmt = __cf_osRcTFl4A::pmr::FormattingFlags::NEW;
                                            &[
                                                __cf_osRcTFl4A::pmr::PConvWrapper("Found variant `")
                                                    .to_pargument_display(fmt),
                                                __cf_osRcTFl4A::pmr::PConvWrapper(
                                                        indices[INVALID_INDEX.1].1,
                                                    )
                                                    .to_pargument_display(fmt),
                                                __cf_osRcTFl4A::pmr::PConvWrapper("` with invalid index: `")
                                                    .to_pargument_display(fmt),
                                                __cf_osRcTFl4A::pmr::PConvWrapper(
                                                        indices[INVALID_INDEX.1].0,
                                                    )
                                                    .to_pargument_display(fmt),
                                                __cf_osRcTFl4A::pmr::PConvWrapper(
                                                        "`. Max supported index is 255.",
                                                    )
                                                    .to_pargument_display(fmt),
                                            ]
                                        };
                                        {
                                            #[doc(hidden)]
                                            const ARR_LEN: usize = ::const_format::pmr::PArgument::calc_len(
                                                CONCATP_NHPMWYD3NJA,
                                            );
                                            #[doc(hidden)]
                                            const CONCAT_ARR: &::const_format::pmr::LenAndArray<
                                                [u8; ARR_LEN],
                                            > = &::const_format::pmr::__priv_concatenate(
                                                CONCATP_NHPMWYD3NJA,
                                            );
                                            #[doc(hidden)]
                                            #[allow(clippy::transmute_ptr_to_ptr)]
                                            const CONCAT_STR: &str = unsafe {
                                                let slice = ::const_format::pmr::transmute::<
                                                    &[u8; ARR_LEN],
                                                    &[u8; CONCAT_ARR.len],
                                                >(&CONCAT_ARR.array);
                                                {
                                                    let bytes: &'static [::const_format::pmr::u8] = slice;
                                                    let string: &'static ::const_format::pmr::str = {
                                                        ::const_format::__hidden_utils::PtrToRef {
                                                            ptr: bytes as *const [::const_format::pmr::u8] as *const str,
                                                        }
                                                            .reff
                                                    };
                                                    string
                                                }
                                            };
                                            CONCAT_STR
                                        }
                                    })
                                },
                            }
                                .x;
                            {
                                ::core::panicking::panic_display(&msg);
                            };
                        }
                        const fn duplicate_info(
                            array: &[(usize, &'static str); 1usize],
                        ) -> (bool, usize, usize) {
                            let len = 1usize;
                            let mut i = 0usize;
                            while i < len {
                                let mut j = i + 1;
                                while j < len {
                                    if array[i].0 == array[j].0 {
                                        return (true, i, j);
                                    }
                                    j += 1;
                                }
                                i += 1;
                            }
                            (false, 0, 0)
                        }
                        const DUP_INFO: (bool, usize, usize) = duplicate_info(&indices);
                        if DUP_INFO.0 {
                            let msg = ::const_format::pmr::__AssertStr {
                                x: {
                                    use ::const_format::__cf_osRcTFl4A;
                                    ({
                                        #[doc(hidden)]
                                        #[allow(unused_mut, non_snake_case)]
                                        const CONCATP_NHPMWYD3NJA: &[__cf_osRcTFl4A::pmr::PArgument] = {
                                            let fmt = __cf_osRcTFl4A::pmr::FormattingFlags::NEW;
                                            &[
                                                __cf_osRcTFl4A::pmr::PConvWrapper(
                                                        "Found variants that have duplicate indexes. Both `",
                                                    )
                                                    .to_pargument_display(fmt),
                                                __cf_osRcTFl4A::pmr::PConvWrapper(indices[DUP_INFO.1].1)
                                                    .to_pargument_display(fmt),
                                                __cf_osRcTFl4A::pmr::PConvWrapper("` and `")
                                                    .to_pargument_display(fmt),
                                                __cf_osRcTFl4A::pmr::PConvWrapper(indices[DUP_INFO.2].1)
                                                    .to_pargument_display(fmt),
                                                __cf_osRcTFl4A::pmr::PConvWrapper("` have the index `")
                                                    .to_pargument_display(fmt),
                                                __cf_osRcTFl4A::pmr::PConvWrapper(indices[DUP_INFO.1].0)
                                                    .to_pargument_display(fmt),
                                                __cf_osRcTFl4A::pmr::PConvWrapper(
                                                        "`. Use different indexes for each variant.",
                                                    )
                                                    .to_pargument_display(fmt),
                                            ]
                                        };
                                        {
                                            #[doc(hidden)]
                                            const ARR_LEN: usize = ::const_format::pmr::PArgument::calc_len(
                                                CONCATP_NHPMWYD3NJA,
                                            );
                                            #[doc(hidden)]
                                            const CONCAT_ARR: &::const_format::pmr::LenAndArray<
                                                [u8; ARR_LEN],
                                            > = &::const_format::pmr::__priv_concatenate(
                                                CONCATP_NHPMWYD3NJA,
                                            );
                                            #[doc(hidden)]
                                            #[allow(clippy::transmute_ptr_to_ptr)]
                                            const CONCAT_STR: &str = unsafe {
                                                let slice = ::const_format::pmr::transmute::<
                                                    &[u8; ARR_LEN],
                                                    &[u8; CONCAT_ARR.len],
                                                >(&CONCAT_ARR.array);
                                                {
                                                    let bytes: &'static [::const_format::pmr::u8] = slice;
                                                    let string: &'static ::const_format::pmr::str = {
                                                        ::const_format::__hidden_utils::PtrToRef {
                                                            ptr: bytes as *const [::const_format::pmr::u8] as *const str,
                                                        }
                                                            .reff
                                                    };
                                                    string
                                                }
                                            };
                                            CONCAT_STR
                                        }
                                    })
                                },
                            }
                                .x;
                            {
                                ::core::panicking::panic_display(&msg);
                            };
                        }
                    };
                    match __codec_input_edqy
                        .read_byte()
                        .map_err(|e| {
                            e
                                .chain(
                                    "Could not decode `ParametersValue`, failed to read variant byte",
                                )
                        })?
                    {
                        #[allow(clippy::unnecessary_cast)]
                        #[allow(clippy::cast_possible_truncation)]
                        __codec_x_edqy if __codec_x_edqy
                            == (0usize) as ::core::primitive::u8 => {
                            #[allow(clippy::redundant_closure_call)]
                            return (move || {
                                ::core::result::Result::Ok(
                                    ParametersValue::MinInflation2({
                                        let __codec_res_edqy = <u32 as ::codec::Decode>::decode(
                                            __codec_input_edqy,
                                        );
                                        match __codec_res_edqy {
                                            ::core::result::Result::Err(e) => {
                                                return ::core::result::Result::Err(
                                                    e
                                                        .chain(
                                                            "Could not decode `ParametersValue::MinInflation2.0`",
                                                        ),
                                                );
                                            }
                                            ::core::result::Result::Ok(__codec_res_edqy) => {
                                                __codec_res_edqy
                                            }
                                        }
                                    }),
                                )
                            })();
                        }
                        _ => {
                            #[allow(clippy::redundant_closure_call)]
                            return (move || {
                                ::core::result::Result::Err(
                                    <_ as ::core::convert::Into<
                                        _,
                                    >>::into(
                                        "Could not decode `ParametersValue`, variant doesn't exist",
                                    ),
                                )
                            })();
                        }
                    }
                }
            }
        };
        #[allow(deprecated)]
        const _: () = {
            fn check_struct() {
                fn check_field<T: ::codec::DecodeWithMemTracking>() {}
                check_field::<u32>();
            }
            #[automatically_derived]
            impl ::codec::DecodeWithMemTracking for ParametersValue {}
        };
        const _: () = {
            #[automatically_derived]
            impl ::codec::MaxEncodedLen for ParametersValue {
                fn max_encoded_len() -> ::core::primitive::usize {
                    0_usize
                        .max(
                            0_usize
                                .saturating_add(
                                    <u32 as ::codec::MaxEncodedLen>::max_encoded_len(),
                                ),
                        )
                        .saturating_add(1)
                }
            }
        };
        impl core::fmt::Debug for ParametersValue {
            fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
                match self {
                    Self::MinInflation2(ref a0) => {
                        fmt.debug_tuple("ParametersValue::MinInflation2")
                            .field(a0)
                            .finish()
                    }
                    _ => Ok(()),
                }
            }
        }
        #[allow(
            non_upper_case_globals,
            deprecated,
            unused_attributes,
            unused_qualifications
        )]
        const _: () = {
            impl ::scale_info::TypeInfo for ParametersValue {
                type Identity = Self;
                fn type_info() -> ::scale_info::Type {
                    ::scale_info::Type::builder()
                        .path(
                            ::scale_info::Path::new_with_replace(
                                "ParametersValue",
                                "polkadot_test::dynamic_params::inflation2FG",
                                &[],
                            ),
                        )
                        .type_params(::alloc::vec::Vec::new())
                        .variant(
                            ::scale_info::build::Variants::new()
                                .variant(
                                    "MinInflation2",
                                    |v| {
                                        v
                                            .index(0u8 as ::core::primitive::u8)
                                            .fields(
                                                ::scale_info::build::Fields::unnamed()
                                                    .field(|f| f.ty::<u32>().type_name("u32")),
                                            )
                                            .docs(
                                                &["Minimum inflation rate used to calculate era payouts."],
                                            )
                                    },
                                ),
                        )
                }
            }
        };
        impl polkadot_sdk::frame_support::traits::dynamic_params::AggregatedKeyValue
        for Parameters {
            type Key = ParametersKey;
            type Value = ParametersValue;
            fn into_parts(self) -> (Self::Key, Option<Self::Value>) {
                match self {
                    Parameters::MinInflation2(key, value) => {
                        (
                            ParametersKey::MinInflation2(key),
                            value.map(ParametersValue::MinInflation2),
                        )
                    }
                }
            }
        }
        #[doc(hidden)]
        pub struct MinInflation2;
        #[automatically_derived]
        impl ::core::clone::Clone for MinInflation2 {
            #[inline]
            fn clone(&self) -> MinInflation2 {
                MinInflation2
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for MinInflation2 {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for MinInflation2 {
            #[inline]
            fn eq(&self, other: &MinInflation2) -> bool {
                true
            }
        }
        #[automatically_derived]
        impl ::core::cmp::Eq for MinInflation2 {
            #[inline]
            #[doc(hidden)]
            #[coverage(off)]
            fn assert_receiver_is_total_eq(&self) -> () {}
        }
        #[allow(deprecated)]
        const _: () = {
            #[automatically_derived]
            impl ::codec::Encode for MinInflation2 {
                fn size_hint(&self) -> usize {
                    0_usize
                }
                fn encode_to<
                    __CodecOutputEdqy: ::codec::Output + ?::core::marker::Sized,
                >(&self, __codec_dest_edqy: &mut __CodecOutputEdqy) {}
            }
            #[automatically_derived]
            impl ::codec::EncodeLike for MinInflation2 {}
        };
        #[allow(deprecated)]
        const _: () = {
            #[automatically_derived]
            impl ::codec::Decode for MinInflation2 {
                fn decode<__CodecInputEdqy: ::codec::Input>(
                    __codec_input_edqy: &mut __CodecInputEdqy,
                ) -> ::core::result::Result<Self, ::codec::Error> {
                    ::core::result::Result::Ok(MinInflation2)
                }
            }
        };
        #[allow(deprecated)]
        const _: () = {
            fn check_struct() {
                fn check_field<T: ::codec::DecodeWithMemTracking>() {}
            }
            #[automatically_derived]
            impl ::codec::DecodeWithMemTracking for MinInflation2 {}
        };
        const _: () = {
            #[automatically_derived]
            impl ::codec::MaxEncodedLen for MinInflation2 {
                fn max_encoded_len() -> ::core::primitive::usize {
                    0_usize
                }
            }
        };
        impl core::fmt::Debug for MinInflation2 {
            fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
                fmt.debug_tuple("MinInflation2").finish()
            }
        }
        #[allow(
            non_upper_case_globals,
            deprecated,
            unused_attributes,
            unused_qualifications
        )]
        const _: () = {
            impl ::scale_info::TypeInfo for MinInflation2 {
                type Identity = Self;
                fn type_info() -> ::scale_info::Type {
                    ::scale_info::Type::builder()
                        .path(
                            ::scale_info::Path::new_with_replace(
                                "MinInflation2",
                                "polkadot_test::dynamic_params::inflation2FG",
                                &[],
                            ),
                        )
                        .type_params(::alloc::vec::Vec::new())
                        .composite(::scale_info::build::Fields::unit())
                }
            }
        };
        impl polkadot_sdk::frame_support::__private::Get<u32> for MinInflation2 {
            fn get() -> u32 {
                match <pallet_parameters::Parameters<
                    Runtime,
                > as polkadot_sdk::frame_support::storage::StorageMap<
                    RuntimeParametersKey,
                    RuntimeParametersValue,
                >>::get(
                    RuntimeParametersKey::Inflation2FG(
                        ParametersKey::MinInflation2(MinInflation2),
                    ),
                ) {
                    Some(
                        RuntimeParametersValue::Inflation2FG(
                            ParametersValue::MinInflation2(inner),
                        ),
                    ) => inner,
                    Some(_) => {
                        {
                            {
                                {
                                    let lvl = ::log::Level::Error;
                                    if lvl <= ::log::STATIC_MAX_LEVEL
                                        && lvl <= ::log::max_level()
                                    {
                                        ::log::__private_api::log(
                                            { ::log::__private_api::GlobalLogger },
                                            format_args!(
                                                "{0}: {1:?}",
                                                ::frame_support::traits::DEFENSIVE_OP_PUBLIC_ERROR,
                                                "Unexpected value type at key - returning default",
                                            ),
                                            lvl,
                                            &(
                                                "runtime::defensive",
                                                "polkadot_test::dynamic_params::inflation2FG",
                                                ::log::__private_api::loc(),
                                            ),
                                            (),
                                        );
                                    }
                                }
                            }
                        };
                        if true {
                            if !false {
                                {
                                    ::core::panicking::panic_fmt(
                                        format_args!(
                                            "{0}: {1:?}",
                                            ::frame_support::traits::DEFENSIVE_OP_INTERNAL_ERROR,
                                            "Unexpected value type at key - returning default",
                                        ),
                                    );
                                }
                            }
                        }
                        0
                    }
                    None => 0,
                }
            }
        }
        impl polkadot_sdk::frame_support::traits::dynamic_params::Key for MinInflation2 {
            type Value = u32;
            type WrappedValue = MinInflation2Value;
        }
        impl From<MinInflation2> for ParametersKey {
            fn from(key: MinInflation2) -> Self {
                ParametersKey::MinInflation2(key)
            }
        }
        impl TryFrom<ParametersKey> for MinInflation2 {
            type Error = ();
            fn try_from(key: ParametersKey) -> Result<Self, Self::Error> {
                match key {
                    ParametersKey::MinInflation2(key) => Ok(key),
                    _ => Err(()),
                }
            }
        }
        #[doc(hidden)]
        pub struct MinInflation2Value(pub u32);
        #[automatically_derived]
        impl ::core::clone::Clone for MinInflation2Value {
            #[inline]
            fn clone(&self) -> MinInflation2Value {
                MinInflation2Value(::core::clone::Clone::clone(&self.0))
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for MinInflation2Value {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for MinInflation2Value {
            #[inline]
            fn eq(&self, other: &MinInflation2Value) -> bool {
                self.0 == other.0
            }
        }
        #[automatically_derived]
        impl ::core::cmp::Eq for MinInflation2Value {
            #[inline]
            #[doc(hidden)]
            #[coverage(off)]
            fn assert_receiver_is_total_eq(&self) -> () {
                let _: ::core::cmp::AssertParamIsEq<u32>;
            }
        }
        impl core::fmt::Debug for MinInflation2Value {
            fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
                fmt.debug_tuple("MinInflation2Value").field(&self.0).finish()
            }
        }
        impl From<MinInflation2Value> for ParametersValue {
            fn from(value: MinInflation2Value) -> Self {
                ParametersValue::MinInflation2(value.0)
            }
        }
        impl From<(MinInflation2, u32)> for Parameters {
            fn from((key, value): (MinInflation2, u32)) -> Self {
                Parameters::MinInflation2(key, Some(value))
            }
        }
        impl From<MinInflation2> for Parameters {
            fn from(key: MinInflation2) -> Self {
                Parameters::MinInflation2(key, None)
            }
        }
        impl TryFrom<ParametersValue> for MinInflation2Value {
            type Error = ();
            fn try_from(value: ParametersValue) -> Result<Self, Self::Error> {
                match value {
                    ParametersValue::MinInflation2(value) => {
                        Ok(MinInflation2Value(value))
                    }
                    _ => Err(()),
                }
            }
        }
        impl From<MinInflation2Value> for u32 {
            fn from(value: MinInflation2Value) -> Self {
                value.0
            }
        }
    }
}
#[doc(hidden)]
pub enum RuntimeParameters {
    /// Parameters used to calculate era payouts, see
    /// [`polkadot_runtime_common::impls::EraPayoutParams`].
    #[codec(index = 0)]
    InflationAV(dynamic_params::inflationAV::Parameters),
    #[codec(index = 1)]
    Inflation2FG(dynamic_params::inflation2FG::Parameters),
}
#[automatically_derived]
impl ::core::clone::Clone for RuntimeParameters {
    #[inline]
    fn clone(&self) -> RuntimeParameters {
        match self {
            RuntimeParameters::InflationAV(__self_0) => {
                RuntimeParameters::InflationAV(::core::clone::Clone::clone(__self_0))
            }
            RuntimeParameters::Inflation2FG(__self_0) => {
                RuntimeParameters::Inflation2FG(::core::clone::Clone::clone(__self_0))
            }
        }
    }
}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for RuntimeParameters {}
#[automatically_derived]
impl ::core::cmp::PartialEq for RuntimeParameters {
    #[inline]
    fn eq(&self, other: &RuntimeParameters) -> bool {
        let __self_discr = ::core::intrinsics::discriminant_value(self);
        let __arg1_discr = ::core::intrinsics::discriminant_value(other);
        __self_discr == __arg1_discr
            && match (self, other) {
                (
                    RuntimeParameters::InflationAV(__self_0),
                    RuntimeParameters::InflationAV(__arg1_0),
                ) => __self_0 == __arg1_0,
                (
                    RuntimeParameters::Inflation2FG(__self_0),
                    RuntimeParameters::Inflation2FG(__arg1_0),
                ) => __self_0 == __arg1_0,
                _ => unsafe { ::core::intrinsics::unreachable() }
            }
    }
}
#[automatically_derived]
impl ::core::cmp::Eq for RuntimeParameters {
    #[inline]
    #[doc(hidden)]
    #[coverage(off)]
    fn assert_receiver_is_total_eq(&self) -> () {
        let _: ::core::cmp::AssertParamIsEq<dynamic_params::inflationAV::Parameters>;
        let _: ::core::cmp::AssertParamIsEq<dynamic_params::inflation2FG::Parameters>;
    }
}
#[allow(deprecated)]
const _: () = {
    #[automatically_derived]
    impl ::codec::Encode for RuntimeParameters {
        fn size_hint(&self) -> usize {
            1_usize
                + match *self {
                    RuntimeParameters::InflationAV(ref aa) => {
                        0_usize.saturating_add(::codec::Encode::size_hint(aa))
                    }
                    RuntimeParameters::Inflation2FG(ref aa) => {
                        0_usize.saturating_add(::codec::Encode::size_hint(aa))
                    }
                    _ => 0_usize,
                }
        }
        fn encode_to<__CodecOutputEdqy: ::codec::Output + ?::core::marker::Sized>(
            &self,
            __codec_dest_edqy: &mut __CodecOutputEdqy,
        ) {
            #[automatically_derived]
            const _: () = {
                #[allow(clippy::unnecessary_cast)]
                #[allow(clippy::cast_possible_truncation)]
                const indices: [(usize, &'static str); 2usize] = [
                    ((0usize) as ::core::primitive::usize, "InflationAV"),
                    ((1usize) as ::core::primitive::usize, "Inflation2FG"),
                ];
                const fn search_for_invalid_index(
                    array: &[(usize, &'static str); 2usize],
                ) -> (bool, usize) {
                    let mut i = 0;
                    while i < 2usize {
                        if array[i].0 > 255 {
                            return (true, i);
                        }
                        i += 1;
                    }
                    (false, 0)
                }
                const INVALID_INDEX: (bool, usize) = search_for_invalid_index(&indices);
                if INVALID_INDEX.0 {
                    let msg = ::const_format::pmr::__AssertStr {
                        x: {
                            use ::const_format::__cf_osRcTFl4A;
                            ({
                                #[doc(hidden)]
                                #[allow(unused_mut, non_snake_case)]
                                const CONCATP_NHPMWYD3NJA: &[__cf_osRcTFl4A::pmr::PArgument] = {
                                    let fmt = __cf_osRcTFl4A::pmr::FormattingFlags::NEW;
                                    &[
                                        __cf_osRcTFl4A::pmr::PConvWrapper("Found variant `")
                                            .to_pargument_display(fmt),
                                        __cf_osRcTFl4A::pmr::PConvWrapper(
                                                indices[INVALID_INDEX.1].1,
                                            )
                                            .to_pargument_display(fmt),
                                        __cf_osRcTFl4A::pmr::PConvWrapper("` with invalid index: `")
                                            .to_pargument_display(fmt),
                                        __cf_osRcTFl4A::pmr::PConvWrapper(
                                                indices[INVALID_INDEX.1].0,
                                            )
                                            .to_pargument_display(fmt),
                                        __cf_osRcTFl4A::pmr::PConvWrapper(
                                                "`. Max supported index is 255.",
                                            )
                                            .to_pargument_display(fmt),
                                    ]
                                };
                                {
                                    #[doc(hidden)]
                                    const ARR_LEN: usize = ::const_format::pmr::PArgument::calc_len(
                                        CONCATP_NHPMWYD3NJA,
                                    );
                                    #[doc(hidden)]
                                    const CONCAT_ARR: &::const_format::pmr::LenAndArray<
                                        [u8; ARR_LEN],
                                    > = &::const_format::pmr::__priv_concatenate(
                                        CONCATP_NHPMWYD3NJA,
                                    );
                                    #[doc(hidden)]
                                    #[allow(clippy::transmute_ptr_to_ptr)]
                                    const CONCAT_STR: &str = unsafe {
                                        let slice = ::const_format::pmr::transmute::<
                                            &[u8; ARR_LEN],
                                            &[u8; CONCAT_ARR.len],
                                        >(&CONCAT_ARR.array);
                                        {
                                            let bytes: &'static [::const_format::pmr::u8] = slice;
                                            let string: &'static ::const_format::pmr::str = {
                                                ::const_format::__hidden_utils::PtrToRef {
                                                    ptr: bytes as *const [::const_format::pmr::u8] as *const str,
                                                }
                                                    .reff
                                            };
                                            string
                                        }
                                    };
                                    CONCAT_STR
                                }
                            })
                        },
                    }
                        .x;
                    {
                        ::core::panicking::panic_display(&msg);
                    };
                }
                const fn duplicate_info(
                    array: &[(usize, &'static str); 2usize],
                ) -> (bool, usize, usize) {
                    let len = 2usize;
                    let mut i = 0usize;
                    while i < len {
                        let mut j = i + 1;
                        while j < len {
                            if array[i].0 == array[j].0 {
                                return (true, i, j);
                            }
                            j += 1;
                        }
                        i += 1;
                    }
                    (false, 0, 0)
                }
                const DUP_INFO: (bool, usize, usize) = duplicate_info(&indices);
                if DUP_INFO.0 {
                    let msg = ::const_format::pmr::__AssertStr {
                        x: {
                            use ::const_format::__cf_osRcTFl4A;
                            ({
                                #[doc(hidden)]
                                #[allow(unused_mut, non_snake_case)]
                                const CONCATP_NHPMWYD3NJA: &[__cf_osRcTFl4A::pmr::PArgument] = {
                                    let fmt = __cf_osRcTFl4A::pmr::FormattingFlags::NEW;
                                    &[
                                        __cf_osRcTFl4A::pmr::PConvWrapper(
                                                "Found variants that have duplicate indexes. Both `",
                                            )
                                            .to_pargument_display(fmt),
                                        __cf_osRcTFl4A::pmr::PConvWrapper(indices[DUP_INFO.1].1)
                                            .to_pargument_display(fmt),
                                        __cf_osRcTFl4A::pmr::PConvWrapper("` and `")
                                            .to_pargument_display(fmt),
                                        __cf_osRcTFl4A::pmr::PConvWrapper(indices[DUP_INFO.2].1)
                                            .to_pargument_display(fmt),
                                        __cf_osRcTFl4A::pmr::PConvWrapper("` have the index `")
                                            .to_pargument_display(fmt),
                                        __cf_osRcTFl4A::pmr::PConvWrapper(indices[DUP_INFO.1].0)
                                            .to_pargument_display(fmt),
                                        __cf_osRcTFl4A::pmr::PConvWrapper(
                                                "`. Use different indexes for each variant.",
                                            )
                                            .to_pargument_display(fmt),
                                    ]
                                };
                                {
                                    #[doc(hidden)]
                                    const ARR_LEN: usize = ::const_format::pmr::PArgument::calc_len(
                                        CONCATP_NHPMWYD3NJA,
                                    );
                                    #[doc(hidden)]
                                    const CONCAT_ARR: &::const_format::pmr::LenAndArray<
                                        [u8; ARR_LEN],
                                    > = &::const_format::pmr::__priv_concatenate(
                                        CONCATP_NHPMWYD3NJA,
                                    );
                                    #[doc(hidden)]
                                    #[allow(clippy::transmute_ptr_to_ptr)]
                                    const CONCAT_STR: &str = unsafe {
                                        let slice = ::const_format::pmr::transmute::<
                                            &[u8; ARR_LEN],
                                            &[u8; CONCAT_ARR.len],
                                        >(&CONCAT_ARR.array);
                                        {
                                            let bytes: &'static [::const_format::pmr::u8] = slice;
                                            let string: &'static ::const_format::pmr::str = {
                                                ::const_format::__hidden_utils::PtrToRef {
                                                    ptr: bytes as *const [::const_format::pmr::u8] as *const str,
                                                }
                                                    .reff
                                            };
                                            string
                                        }
                                    };
                                    CONCAT_STR
                                }
                            })
                        },
                    }
                        .x;
                    {
                        ::core::panicking::panic_display(&msg);
                    };
                }
            };
            match *self {
                RuntimeParameters::InflationAV(ref aa) => {
                    #[allow(clippy::unnecessary_cast)]
                    __codec_dest_edqy.push_byte((0usize) as ::core::primitive::u8);
                    ::codec::Encode::encode_to(aa, __codec_dest_edqy);
                }
                RuntimeParameters::Inflation2FG(ref aa) => {
                    #[allow(clippy::unnecessary_cast)]
                    __codec_dest_edqy.push_byte((1usize) as ::core::primitive::u8);
                    ::codec::Encode::encode_to(aa, __codec_dest_edqy);
                }
                _ => {}
            }
        }
    }
    #[automatically_derived]
    impl ::codec::EncodeLike for RuntimeParameters {}
};
#[allow(deprecated)]
const _: () = {
    #[automatically_derived]
    impl ::codec::Decode for RuntimeParameters {
        fn decode<__CodecInputEdqy: ::codec::Input>(
            __codec_input_edqy: &mut __CodecInputEdqy,
        ) -> ::core::result::Result<Self, ::codec::Error> {
            #[automatically_derived]
            const _: () = {
                #[allow(clippy::unnecessary_cast)]
                #[allow(clippy::cast_possible_truncation)]
                const indices: [(usize, &'static str); 2usize] = [
                    ((0usize) as ::core::primitive::usize, "InflationAV"),
                    ((1usize) as ::core::primitive::usize, "Inflation2FG"),
                ];
                const fn search_for_invalid_index(
                    array: &[(usize, &'static str); 2usize],
                ) -> (bool, usize) {
                    let mut i = 0;
                    while i < 2usize {
                        if array[i].0 > 255 {
                            return (true, i);
                        }
                        i += 1;
                    }
                    (false, 0)
                }
                const INVALID_INDEX: (bool, usize) = search_for_invalid_index(&indices);
                if INVALID_INDEX.0 {
                    let msg = ::const_format::pmr::__AssertStr {
                        x: {
                            use ::const_format::__cf_osRcTFl4A;
                            ({
                                #[doc(hidden)]
                                #[allow(unused_mut, non_snake_case)]
                                const CONCATP_NHPMWYD3NJA: &[__cf_osRcTFl4A::pmr::PArgument] = {
                                    let fmt = __cf_osRcTFl4A::pmr::FormattingFlags::NEW;
                                    &[
                                        __cf_osRcTFl4A::pmr::PConvWrapper("Found variant `")
                                            .to_pargument_display(fmt),
                                        __cf_osRcTFl4A::pmr::PConvWrapper(
                                                indices[INVALID_INDEX.1].1,
                                            )
                                            .to_pargument_display(fmt),
                                        __cf_osRcTFl4A::pmr::PConvWrapper("` with invalid index: `")
                                            .to_pargument_display(fmt),
                                        __cf_osRcTFl4A::pmr::PConvWrapper(
                                                indices[INVALID_INDEX.1].0,
                                            )
                                            .to_pargument_display(fmt),
                                        __cf_osRcTFl4A::pmr::PConvWrapper(
                                                "`. Max supported index is 255.",
                                            )
                                            .to_pargument_display(fmt),
                                    ]
                                };
                                {
                                    #[doc(hidden)]
                                    const ARR_LEN: usize = ::const_format::pmr::PArgument::calc_len(
                                        CONCATP_NHPMWYD3NJA,
                                    );
                                    #[doc(hidden)]
                                    const CONCAT_ARR: &::const_format::pmr::LenAndArray<
                                        [u8; ARR_LEN],
                                    > = &::const_format::pmr::__priv_concatenate(
                                        CONCATP_NHPMWYD3NJA,
                                    );
                                    #[doc(hidden)]
                                    #[allow(clippy::transmute_ptr_to_ptr)]
                                    const CONCAT_STR: &str = unsafe {
                                        let slice = ::const_format::pmr::transmute::<
                                            &[u8; ARR_LEN],
                                            &[u8; CONCAT_ARR.len],
                                        >(&CONCAT_ARR.array);
                                        {
                                            let bytes: &'static [::const_format::pmr::u8] = slice;
                                            let string: &'static ::const_format::pmr::str = {
                                                ::const_format::__hidden_utils::PtrToRef {
                                                    ptr: bytes as *const [::const_format::pmr::u8] as *const str,
                                                }
                                                    .reff
                                            };
                                            string
                                        }
                                    };
                                    CONCAT_STR
                                }
                            })
                        },
                    }
                        .x;
                    {
                        ::core::panicking::panic_display(&msg);
                    };
                }
                const fn duplicate_info(
                    array: &[(usize, &'static str); 2usize],
                ) -> (bool, usize, usize) {
                    let len = 2usize;
                    let mut i = 0usize;
                    while i < len {
                        let mut j = i + 1;
                        while j < len {
                            if array[i].0 == array[j].0 {
                                return (true, i, j);
                            }
                            j += 1;
                        }
                        i += 1;
                    }
                    (false, 0, 0)
                }
                const DUP_INFO: (bool, usize, usize) = duplicate_info(&indices);
                if DUP_INFO.0 {
                    let msg = ::const_format::pmr::__AssertStr {
                        x: {
                            use ::const_format::__cf_osRcTFl4A;
                            ({
                                #[doc(hidden)]
                                #[allow(unused_mut, non_snake_case)]
                                const CONCATP_NHPMWYD3NJA: &[__cf_osRcTFl4A::pmr::PArgument] = {
                                    let fmt = __cf_osRcTFl4A::pmr::FormattingFlags::NEW;
                                    &[
                                        __cf_osRcTFl4A::pmr::PConvWrapper(
                                                "Found variants that have duplicate indexes. Both `",
                                            )
                                            .to_pargument_display(fmt),
                                        __cf_osRcTFl4A::pmr::PConvWrapper(indices[DUP_INFO.1].1)
                                            .to_pargument_display(fmt),
                                        __cf_osRcTFl4A::pmr::PConvWrapper("` and `")
                                            .to_pargument_display(fmt),
                                        __cf_osRcTFl4A::pmr::PConvWrapper(indices[DUP_INFO.2].1)
                                            .to_pargument_display(fmt),
                                        __cf_osRcTFl4A::pmr::PConvWrapper("` have the index `")
                                            .to_pargument_display(fmt),
                                        __cf_osRcTFl4A::pmr::PConvWrapper(indices[DUP_INFO.1].0)
                                            .to_pargument_display(fmt),
                                        __cf_osRcTFl4A::pmr::PConvWrapper(
                                                "`. Use different indexes for each variant.",
                                            )
                                            .to_pargument_display(fmt),
                                    ]
                                };
                                {
                                    #[doc(hidden)]
                                    const ARR_LEN: usize = ::const_format::pmr::PArgument::calc_len(
                                        CONCATP_NHPMWYD3NJA,
                                    );
                                    #[doc(hidden)]
                                    const CONCAT_ARR: &::const_format::pmr::LenAndArray<
                                        [u8; ARR_LEN],
                                    > = &::const_format::pmr::__priv_concatenate(
                                        CONCATP_NHPMWYD3NJA,
                                    );
                                    #[doc(hidden)]
                                    #[allow(clippy::transmute_ptr_to_ptr)]
                                    const CONCAT_STR: &str = unsafe {
                                        let slice = ::const_format::pmr::transmute::<
                                            &[u8; ARR_LEN],
                                            &[u8; CONCAT_ARR.len],
                                        >(&CONCAT_ARR.array);
                                        {
                                            let bytes: &'static [::const_format::pmr::u8] = slice;
                                            let string: &'static ::const_format::pmr::str = {
                                                ::const_format::__hidden_utils::PtrToRef {
                                                    ptr: bytes as *const [::const_format::pmr::u8] as *const str,
                                                }
                                                    .reff
                                            };
                                            string
                                        }
                                    };
                                    CONCAT_STR
                                }
                            })
                        },
                    }
                        .x;
                    {
                        ::core::panicking::panic_display(&msg);
                    };
                }
            };
            match __codec_input_edqy
                .read_byte()
                .map_err(|e| {
                    e
                        .chain(
                            "Could not decode `RuntimeParameters`, failed to read variant byte",
                        )
                })?
            {
                #[allow(clippy::unnecessary_cast)]
                #[allow(clippy::cast_possible_truncation)]
                __codec_x_edqy if __codec_x_edqy
                    == (0usize) as ::core::primitive::u8 => {
                    #[allow(clippy::redundant_closure_call)]
                    return (move || {
                        ::core::result::Result::Ok(
                            RuntimeParameters::InflationAV({
                                let __codec_res_edqy = <dynamic_params::inflationAV::Parameters as ::codec::Decode>::decode(
                                    __codec_input_edqy,
                                );
                                match __codec_res_edqy {
                                    ::core::result::Result::Err(e) => {
                                        return ::core::result::Result::Err(
                                            e
                                                .chain(
                                                    "Could not decode `RuntimeParameters::InflationAV.0`",
                                                ),
                                        );
                                    }
                                    ::core::result::Result::Ok(__codec_res_edqy) => {
                                        __codec_res_edqy
                                    }
                                }
                            }),
                        )
                    })();
                }
                #[allow(clippy::unnecessary_cast)]
                #[allow(clippy::cast_possible_truncation)]
                __codec_x_edqy if __codec_x_edqy
                    == (1usize) as ::core::primitive::u8 => {
                    #[allow(clippy::redundant_closure_call)]
                    return (move || {
                        ::core::result::Result::Ok(
                            RuntimeParameters::Inflation2FG({
                                let __codec_res_edqy = <dynamic_params::inflation2FG::Parameters as ::codec::Decode>::decode(
                                    __codec_input_edqy,
                                );
                                match __codec_res_edqy {
                                    ::core::result::Result::Err(e) => {
                                        return ::core::result::Result::Err(
                                            e
                                                .chain(
                                                    "Could not decode `RuntimeParameters::Inflation2FG.0`",
                                                ),
                                        );
                                    }
                                    ::core::result::Result::Ok(__codec_res_edqy) => {
                                        __codec_res_edqy
                                    }
                                }
                            }),
                        )
                    })();
                }
                _ => {
                    #[allow(clippy::redundant_closure_call)]
                    return (move || {
                        ::core::result::Result::Err(
                            <_ as ::core::convert::Into<
                                _,
                            >>::into(
                                "Could not decode `RuntimeParameters`, variant doesn't exist",
                            ),
                        )
                    })();
                }
            }
        }
    }
};
#[allow(deprecated)]
const _: () = {
    fn check_struct() {
        fn check_field<T: ::codec::DecodeWithMemTracking>() {}
        check_field::<dynamic_params::inflationAV::Parameters>();
        check_field::<dynamic_params::inflation2FG::Parameters>();
    }
    #[automatically_derived]
    impl ::codec::DecodeWithMemTracking for RuntimeParameters {}
};
const _: () = {
    #[automatically_derived]
    impl ::codec::MaxEncodedLen for RuntimeParameters {
        fn max_encoded_len() -> ::core::primitive::usize {
            0_usize
                .max(
                    0_usize
                        .saturating_add(
                            <dynamic_params::inflationAV::Parameters as ::codec::MaxEncodedLen>::max_encoded_len(),
                        ),
                )
                .max(
                    0_usize
                        .saturating_add(
                            <dynamic_params::inflation2FG::Parameters as ::codec::MaxEncodedLen>::max_encoded_len(),
                        ),
                )
                .saturating_add(1)
        }
    }
};
impl core::fmt::Debug for RuntimeParameters {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::InflationAV(ref a0) => {
                fmt.debug_tuple("RuntimeParameters::InflationAV").field(a0).finish()
            }
            Self::Inflation2FG(ref a0) => {
                fmt.debug_tuple("RuntimeParameters::Inflation2FG").field(a0).finish()
            }
            _ => Ok(()),
        }
    }
}
#[allow(non_upper_case_globals, deprecated, unused_attributes, unused_qualifications)]
const _: () = {
    impl ::scale_info::TypeInfo for RuntimeParameters {
        type Identity = Self;
        fn type_info() -> ::scale_info::Type {
            ::scale_info::Type::builder()
                .path(
                    ::scale_info::Path::new_with_replace(
                        "RuntimeParameters",
                        "polkadot_test",
                        &[],
                    ),
                )
                .type_params(::alloc::vec::Vec::new())
                .variant(
                    ::scale_info::build::Variants::new()
                        .variant(
                            "InflationAV",
                            |v| {
                                v
                                    .index(0u8 as ::core::primitive::u8)
                                    .fields(
                                        ::scale_info::build::Fields::unnamed()
                                            .field(|f| {
                                                f
                                                    .ty::<dynamic_params::inflationAV::Parameters>()
                                                    .type_name("dynamic_params::inflationAV::Parameters")
                                            }),
                                    )
                                    .docs(
                                        &[
                                            "Parameters used to calculate era payouts, see",
                                            "[`polkadot_runtime_common::impls::EraPayoutParams`].",
                                        ],
                                    )
                            },
                        )
                        .variant(
                            "Inflation2FG",
                            |v| {
                                v
                                    .index(1u8 as ::core::primitive::u8)
                                    .fields(
                                        ::scale_info::build::Fields::unnamed()
                                            .field(|f| {
                                                f
                                                    .ty::<dynamic_params::inflation2FG::Parameters>()
                                                    .type_name("dynamic_params::inflation2FG::Parameters")
                                            }),
                                    )
                            },
                        ),
                )
        }
    }
};
#[doc(hidden)]
pub enum RuntimeParametersKey {
    /// Parameters used to calculate era payouts, see
    /// [`polkadot_runtime_common::impls::EraPayoutParams`].
    #[codec(index = 0)]
    InflationAV(
        <dynamic_params::inflationAV::Parameters as polkadot_sdk::frame_support::traits::dynamic_params::AggregatedKeyValue>::Key,
    ),
    #[codec(index = 1)]
    Inflation2FG(
        <dynamic_params::inflation2FG::Parameters as polkadot_sdk::frame_support::traits::dynamic_params::AggregatedKeyValue>::Key,
    ),
}
#[automatically_derived]
impl ::core::clone::Clone for RuntimeParametersKey {
    #[inline]
    fn clone(&self) -> RuntimeParametersKey {
        match self {
            RuntimeParametersKey::InflationAV(__self_0) => {
                RuntimeParametersKey::InflationAV(::core::clone::Clone::clone(__self_0))
            }
            RuntimeParametersKey::Inflation2FG(__self_0) => {
                RuntimeParametersKey::Inflation2FG(::core::clone::Clone::clone(__self_0))
            }
        }
    }
}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for RuntimeParametersKey {}
#[automatically_derived]
impl ::core::cmp::PartialEq for RuntimeParametersKey {
    #[inline]
    fn eq(&self, other: &RuntimeParametersKey) -> bool {
        let __self_discr = ::core::intrinsics::discriminant_value(self);
        let __arg1_discr = ::core::intrinsics::discriminant_value(other);
        __self_discr == __arg1_discr
            && match (self, other) {
                (
                    RuntimeParametersKey::InflationAV(__self_0),
                    RuntimeParametersKey::InflationAV(__arg1_0),
                ) => __self_0 == __arg1_0,
                (
                    RuntimeParametersKey::Inflation2FG(__self_0),
                    RuntimeParametersKey::Inflation2FG(__arg1_0),
                ) => __self_0 == __arg1_0,
                _ => unsafe { ::core::intrinsics::unreachable() }
            }
    }
}
#[automatically_derived]
impl ::core::cmp::Eq for RuntimeParametersKey {
    #[inline]
    #[doc(hidden)]
    #[coverage(off)]
    fn assert_receiver_is_total_eq(&self) -> () {
        let _: ::core::cmp::AssertParamIsEq<
            <dynamic_params::inflationAV::Parameters as polkadot_sdk::frame_support::traits::dynamic_params::AggregatedKeyValue>::Key,
        >;
        let _: ::core::cmp::AssertParamIsEq<
            <dynamic_params::inflation2FG::Parameters as polkadot_sdk::frame_support::traits::dynamic_params::AggregatedKeyValue>::Key,
        >;
    }
}
#[allow(deprecated)]
const _: () = {
    #[automatically_derived]
    impl ::codec::Encode for RuntimeParametersKey {
        fn size_hint(&self) -> usize {
            1_usize
                + match *self {
                    RuntimeParametersKey::InflationAV(ref aa) => {
                        0_usize.saturating_add(::codec::Encode::size_hint(aa))
                    }
                    RuntimeParametersKey::Inflation2FG(ref aa) => {
                        0_usize.saturating_add(::codec::Encode::size_hint(aa))
                    }
                    _ => 0_usize,
                }
        }
        fn encode_to<__CodecOutputEdqy: ::codec::Output + ?::core::marker::Sized>(
            &self,
            __codec_dest_edqy: &mut __CodecOutputEdqy,
        ) {
            #[automatically_derived]
            const _: () = {
                #[allow(clippy::unnecessary_cast)]
                #[allow(clippy::cast_possible_truncation)]
                const indices: [(usize, &'static str); 2usize] = [
                    ((0usize) as ::core::primitive::usize, "InflationAV"),
                    ((1usize) as ::core::primitive::usize, "Inflation2FG"),
                ];
                const fn search_for_invalid_index(
                    array: &[(usize, &'static str); 2usize],
                ) -> (bool, usize) {
                    let mut i = 0;
                    while i < 2usize {
                        if array[i].0 > 255 {
                            return (true, i);
                        }
                        i += 1;
                    }
                    (false, 0)
                }
                const INVALID_INDEX: (bool, usize) = search_for_invalid_index(&indices);
                if INVALID_INDEX.0 {
                    let msg = ::const_format::pmr::__AssertStr {
                        x: {
                            use ::const_format::__cf_osRcTFl4A;
                            ({
                                #[doc(hidden)]
                                #[allow(unused_mut, non_snake_case)]
                                const CONCATP_NHPMWYD3NJA: &[__cf_osRcTFl4A::pmr::PArgument] = {
                                    let fmt = __cf_osRcTFl4A::pmr::FormattingFlags::NEW;
                                    &[
                                        __cf_osRcTFl4A::pmr::PConvWrapper("Found variant `")
                                            .to_pargument_display(fmt),
                                        __cf_osRcTFl4A::pmr::PConvWrapper(
                                                indices[INVALID_INDEX.1].1,
                                            )
                                            .to_pargument_display(fmt),
                                        __cf_osRcTFl4A::pmr::PConvWrapper("` with invalid index: `")
                                            .to_pargument_display(fmt),
                                        __cf_osRcTFl4A::pmr::PConvWrapper(
                                                indices[INVALID_INDEX.1].0,
                                            )
                                            .to_pargument_display(fmt),
                                        __cf_osRcTFl4A::pmr::PConvWrapper(
                                                "`. Max supported index is 255.",
                                            )
                                            .to_pargument_display(fmt),
                                    ]
                                };
                                {
                                    #[doc(hidden)]
                                    const ARR_LEN: usize = ::const_format::pmr::PArgument::calc_len(
                                        CONCATP_NHPMWYD3NJA,
                                    );
                                    #[doc(hidden)]
                                    const CONCAT_ARR: &::const_format::pmr::LenAndArray<
                                        [u8; ARR_LEN],
                                    > = &::const_format::pmr::__priv_concatenate(
                                        CONCATP_NHPMWYD3NJA,
                                    );
                                    #[doc(hidden)]
                                    #[allow(clippy::transmute_ptr_to_ptr)]
                                    const CONCAT_STR: &str = unsafe {
                                        let slice = ::const_format::pmr::transmute::<
                                            &[u8; ARR_LEN],
                                            &[u8; CONCAT_ARR.len],
                                        >(&CONCAT_ARR.array);
                                        {
                                            let bytes: &'static [::const_format::pmr::u8] = slice;
                                            let string: &'static ::const_format::pmr::str = {
                                                ::const_format::__hidden_utils::PtrToRef {
                                                    ptr: bytes as *const [::const_format::pmr::u8] as *const str,
                                                }
                                                    .reff
                                            };
                                            string
                                        }
                                    };
                                    CONCAT_STR
                                }
                            })
                        },
                    }
                        .x;
                    {
                        ::core::panicking::panic_display(&msg);
                    };
                }
                const fn duplicate_info(
                    array: &[(usize, &'static str); 2usize],
                ) -> (bool, usize, usize) {
                    let len = 2usize;
                    let mut i = 0usize;
                    while i < len {
                        let mut j = i + 1;
                        while j < len {
                            if array[i].0 == array[j].0 {
                                return (true, i, j);
                            }
                            j += 1;
                        }
                        i += 1;
                    }
                    (false, 0, 0)
                }
                const DUP_INFO: (bool, usize, usize) = duplicate_info(&indices);
                if DUP_INFO.0 {
                    let msg = ::const_format::pmr::__AssertStr {
                        x: {
                            use ::const_format::__cf_osRcTFl4A;
                            ({
                                #[doc(hidden)]
                                #[allow(unused_mut, non_snake_case)]
                                const CONCATP_NHPMWYD3NJA: &[__cf_osRcTFl4A::pmr::PArgument] = {
                                    let fmt = __cf_osRcTFl4A::pmr::FormattingFlags::NEW;
                                    &[
                                        __cf_osRcTFl4A::pmr::PConvWrapper(
                                                "Found variants that have duplicate indexes. Both `",
                                            )
                                            .to_pargument_display(fmt),
                                        __cf_osRcTFl4A::pmr::PConvWrapper(indices[DUP_INFO.1].1)
                                            .to_pargument_display(fmt),
                                        __cf_osRcTFl4A::pmr::PConvWrapper("` and `")
                                            .to_pargument_display(fmt),
                                        __cf_osRcTFl4A::pmr::PConvWrapper(indices[DUP_INFO.2].1)
                                            .to_pargument_display(fmt),
                                        __cf_osRcTFl4A::pmr::PConvWrapper("` have the index `")
                                            .to_pargument_display(fmt),
                                        __cf_osRcTFl4A::pmr::PConvWrapper(indices[DUP_INFO.1].0)
                                            .to_pargument_display(fmt),
                                        __cf_osRcTFl4A::pmr::PConvWrapper(
                                                "`. Use different indexes for each variant.",
                                            )
                                            .to_pargument_display(fmt),
                                    ]
                                };
                                {
                                    #[doc(hidden)]
                                    const ARR_LEN: usize = ::const_format::pmr::PArgument::calc_len(
                                        CONCATP_NHPMWYD3NJA,
                                    );
                                    #[doc(hidden)]
                                    const CONCAT_ARR: &::const_format::pmr::LenAndArray<
                                        [u8; ARR_LEN],
                                    > = &::const_format::pmr::__priv_concatenate(
                                        CONCATP_NHPMWYD3NJA,
                                    );
                                    #[doc(hidden)]
                                    #[allow(clippy::transmute_ptr_to_ptr)]
                                    const CONCAT_STR: &str = unsafe {
                                        let slice = ::const_format::pmr::transmute::<
                                            &[u8; ARR_LEN],
                                            &[u8; CONCAT_ARR.len],
                                        >(&CONCAT_ARR.array);
                                        {
                                            let bytes: &'static [::const_format::pmr::u8] = slice;
                                            let string: &'static ::const_format::pmr::str = {
                                                ::const_format::__hidden_utils::PtrToRef {
                                                    ptr: bytes as *const [::const_format::pmr::u8] as *const str,
                                                }
                                                    .reff
                                            };
                                            string
                                        }
                                    };
                                    CONCAT_STR
                                }
                            })
                        },
                    }
                        .x;
                    {
                        ::core::panicking::panic_display(&msg);
                    };
                }
            };
            match *self {
                RuntimeParametersKey::InflationAV(ref aa) => {
                    #[allow(clippy::unnecessary_cast)]
                    __codec_dest_edqy.push_byte((0usize) as ::core::primitive::u8);
                    ::codec::Encode::encode_to(aa, __codec_dest_edqy);
                }
                RuntimeParametersKey::Inflation2FG(ref aa) => {
                    #[allow(clippy::unnecessary_cast)]
                    __codec_dest_edqy.push_byte((1usize) as ::core::primitive::u8);
                    ::codec::Encode::encode_to(aa, __codec_dest_edqy);
                }
                _ => {}
            }
        }
    }
    #[automatically_derived]
    impl ::codec::EncodeLike for RuntimeParametersKey {}
};
#[allow(deprecated)]
const _: () = {
    #[automatically_derived]
    impl ::codec::Decode for RuntimeParametersKey {
        fn decode<__CodecInputEdqy: ::codec::Input>(
            __codec_input_edqy: &mut __CodecInputEdqy,
        ) -> ::core::result::Result<Self, ::codec::Error> {
            #[automatically_derived]
            const _: () = {
                #[allow(clippy::unnecessary_cast)]
                #[allow(clippy::cast_possible_truncation)]
                const indices: [(usize, &'static str); 2usize] = [
                    ((0usize) as ::core::primitive::usize, "InflationAV"),
                    ((1usize) as ::core::primitive::usize, "Inflation2FG"),
                ];
                const fn search_for_invalid_index(
                    array: &[(usize, &'static str); 2usize],
                ) -> (bool, usize) {
                    let mut i = 0;
                    while i < 2usize {
                        if array[i].0 > 255 {
                            return (true, i);
                        }
                        i += 1;
                    }
                    (false, 0)
                }
                const INVALID_INDEX: (bool, usize) = search_for_invalid_index(&indices);
                if INVALID_INDEX.0 {
                    let msg = ::const_format::pmr::__AssertStr {
                        x: {
                            use ::const_format::__cf_osRcTFl4A;
                            ({
                                #[doc(hidden)]
                                #[allow(unused_mut, non_snake_case)]
                                const CONCATP_NHPMWYD3NJA: &[__cf_osRcTFl4A::pmr::PArgument] = {
                                    let fmt = __cf_osRcTFl4A::pmr::FormattingFlags::NEW;
                                    &[
                                        __cf_osRcTFl4A::pmr::PConvWrapper("Found variant `")
                                            .to_pargument_display(fmt),
                                        __cf_osRcTFl4A::pmr::PConvWrapper(
                                                indices[INVALID_INDEX.1].1,
                                            )
                                            .to_pargument_display(fmt),
                                        __cf_osRcTFl4A::pmr::PConvWrapper("` with invalid index: `")
                                            .to_pargument_display(fmt),
                                        __cf_osRcTFl4A::pmr::PConvWrapper(
                                                indices[INVALID_INDEX.1].0,
                                            )
                                            .to_pargument_display(fmt),
                                        __cf_osRcTFl4A::pmr::PConvWrapper(
                                                "`. Max supported index is 255.",
                                            )
                                            .to_pargument_display(fmt),
                                    ]
                                };
                                {
                                    #[doc(hidden)]
                                    const ARR_LEN: usize = ::const_format::pmr::PArgument::calc_len(
                                        CONCATP_NHPMWYD3NJA,
                                    );
                                    #[doc(hidden)]
                                    const CONCAT_ARR: &::const_format::pmr::LenAndArray<
                                        [u8; ARR_LEN],
                                    > = &::const_format::pmr::__priv_concatenate(
                                        CONCATP_NHPMWYD3NJA,
                                    );
                                    #[doc(hidden)]
                                    #[allow(clippy::transmute_ptr_to_ptr)]
                                    const CONCAT_STR: &str = unsafe {
                                        let slice = ::const_format::pmr::transmute::<
                                            &[u8; ARR_LEN],
                                            &[u8; CONCAT_ARR.len],
                                        >(&CONCAT_ARR.array);
                                        {
                                            let bytes: &'static [::const_format::pmr::u8] = slice;
                                            let string: &'static ::const_format::pmr::str = {
                                                ::const_format::__hidden_utils::PtrToRef {
                                                    ptr: bytes as *const [::const_format::pmr::u8] as *const str,
                                                }
                                                    .reff
                                            };
                                            string
                                        }
                                    };
                                    CONCAT_STR
                                }
                            })
                        },
                    }
                        .x;
                    {
                        ::core::panicking::panic_display(&msg);
                    };
                }
                const fn duplicate_info(
                    array: &[(usize, &'static str); 2usize],
                ) -> (bool, usize, usize) {
                    let len = 2usize;
                    let mut i = 0usize;
                    while i < len {
                        let mut j = i + 1;
                        while j < len {
                            if array[i].0 == array[j].0 {
                                return (true, i, j);
                            }
                            j += 1;
                        }
                        i += 1;
                    }
                    (false, 0, 0)
                }
                const DUP_INFO: (bool, usize, usize) = duplicate_info(&indices);
                if DUP_INFO.0 {
                    let msg = ::const_format::pmr::__AssertStr {
                        x: {
                            use ::const_format::__cf_osRcTFl4A;
                            ({
                                #[doc(hidden)]
                                #[allow(unused_mut, non_snake_case)]
                                const CONCATP_NHPMWYD3NJA: &[__cf_osRcTFl4A::pmr::PArgument] = {
                                    let fmt = __cf_osRcTFl4A::pmr::FormattingFlags::NEW;
                                    &[
                                        __cf_osRcTFl4A::pmr::PConvWrapper(
                                                "Found variants that have duplicate indexes. Both `",
                                            )
                                            .to_pargument_display(fmt),
                                        __cf_osRcTFl4A::pmr::PConvWrapper(indices[DUP_INFO.1].1)
                                            .to_pargument_display(fmt),
                                        __cf_osRcTFl4A::pmr::PConvWrapper("` and `")
                                            .to_pargument_display(fmt),
                                        __cf_osRcTFl4A::pmr::PConvWrapper(indices[DUP_INFO.2].1)
                                            .to_pargument_display(fmt),
                                        __cf_osRcTFl4A::pmr::PConvWrapper("` have the index `")
                                            .to_pargument_display(fmt),
                                        __cf_osRcTFl4A::pmr::PConvWrapper(indices[DUP_INFO.1].0)
                                            .to_pargument_display(fmt),
                                        __cf_osRcTFl4A::pmr::PConvWrapper(
                                                "`. Use different indexes for each variant.",
                                            )
                                            .to_pargument_display(fmt),
                                    ]
                                };
                                {
                                    #[doc(hidden)]
                                    const ARR_LEN: usize = ::const_format::pmr::PArgument::calc_len(
                                        CONCATP_NHPMWYD3NJA,
                                    );
                                    #[doc(hidden)]
                                    const CONCAT_ARR: &::const_format::pmr::LenAndArray<
                                        [u8; ARR_LEN],
                                    > = &::const_format::pmr::__priv_concatenate(
                                        CONCATP_NHPMWYD3NJA,
                                    );
                                    #[doc(hidden)]
                                    #[allow(clippy::transmute_ptr_to_ptr)]
                                    const CONCAT_STR: &str = unsafe {
                                        let slice = ::const_format::pmr::transmute::<
                                            &[u8; ARR_LEN],
                                            &[u8; CONCAT_ARR.len],
                                        >(&CONCAT_ARR.array);
                                        {
                                            let bytes: &'static [::const_format::pmr::u8] = slice;
                                            let string: &'static ::const_format::pmr::str = {
                                                ::const_format::__hidden_utils::PtrToRef {
                                                    ptr: bytes as *const [::const_format::pmr::u8] as *const str,
                                                }
                                                    .reff
                                            };
                                            string
                                        }
                                    };
                                    CONCAT_STR
                                }
                            })
                        },
                    }
                        .x;
                    {
                        ::core::panicking::panic_display(&msg);
                    };
                }
            };
            match __codec_input_edqy
                .read_byte()
                .map_err(|e| {
                    e
                        .chain(
                            "Could not decode `RuntimeParametersKey`, failed to read variant byte",
                        )
                })?
            {
                #[allow(clippy::unnecessary_cast)]
                #[allow(clippy::cast_possible_truncation)]
                __codec_x_edqy if __codec_x_edqy
                    == (0usize) as ::core::primitive::u8 => {
                    #[allow(clippy::redundant_closure_call)]
                    return (move || {
                        ::core::result::Result::Ok(
                            RuntimeParametersKey::InflationAV({
                                let __codec_res_edqy = <<dynamic_params::inflationAV::Parameters as polkadot_sdk::frame_support::traits::dynamic_params::AggregatedKeyValue>::Key as ::codec::Decode>::decode(
                                    __codec_input_edqy,
                                );
                                match __codec_res_edqy {
                                    ::core::result::Result::Err(e) => {
                                        return ::core::result::Result::Err(
                                            e
                                                .chain(
                                                    "Could not decode `RuntimeParametersKey::InflationAV.0`",
                                                ),
                                        );
                                    }
                                    ::core::result::Result::Ok(__codec_res_edqy) => {
                                        __codec_res_edqy
                                    }
                                }
                            }),
                        )
                    })();
                }
                #[allow(clippy::unnecessary_cast)]
                #[allow(clippy::cast_possible_truncation)]
                __codec_x_edqy if __codec_x_edqy
                    == (1usize) as ::core::primitive::u8 => {
                    #[allow(clippy::redundant_closure_call)]
                    return (move || {
                        ::core::result::Result::Ok(
                            RuntimeParametersKey::Inflation2FG({
                                let __codec_res_edqy = <<dynamic_params::inflation2FG::Parameters as polkadot_sdk::frame_support::traits::dynamic_params::AggregatedKeyValue>::Key as ::codec::Decode>::decode(
                                    __codec_input_edqy,
                                );
                                match __codec_res_edqy {
                                    ::core::result::Result::Err(e) => {
                                        return ::core::result::Result::Err(
                                            e
                                                .chain(
                                                    "Could not decode `RuntimeParametersKey::Inflation2FG.0`",
                                                ),
                                        );
                                    }
                                    ::core::result::Result::Ok(__codec_res_edqy) => {
                                        __codec_res_edqy
                                    }
                                }
                            }),
                        )
                    })();
                }
                _ => {
                    #[allow(clippy::redundant_closure_call)]
                    return (move || {
                        ::core::result::Result::Err(
                            <_ as ::core::convert::Into<
                                _,
                            >>::into(
                                "Could not decode `RuntimeParametersKey`, variant doesn't exist",
                            ),
                        )
                    })();
                }
            }
        }
    }
};
#[allow(deprecated)]
const _: () = {
    fn check_struct() {
        fn check_field<T: ::codec::DecodeWithMemTracking>() {}
        check_field::<
            <dynamic_params::inflationAV::Parameters as polkadot_sdk::frame_support::traits::dynamic_params::AggregatedKeyValue>::Key,
        >();
        check_field::<
            <dynamic_params::inflation2FG::Parameters as polkadot_sdk::frame_support::traits::dynamic_params::AggregatedKeyValue>::Key,
        >();
    }
    #[automatically_derived]
    impl ::codec::DecodeWithMemTracking for RuntimeParametersKey {}
};
const _: () = {
    #[automatically_derived]
    impl ::codec::MaxEncodedLen for RuntimeParametersKey {
        fn max_encoded_len() -> ::core::primitive::usize {
            0_usize
                .max(
                    0_usize
                        .saturating_add(
                            <<dynamic_params::inflationAV::Parameters as polkadot_sdk::frame_support::traits::dynamic_params::AggregatedKeyValue>::Key as ::codec::MaxEncodedLen>::max_encoded_len(),
                        ),
                )
                .max(
                    0_usize
                        .saturating_add(
                            <<dynamic_params::inflation2FG::Parameters as polkadot_sdk::frame_support::traits::dynamic_params::AggregatedKeyValue>::Key as ::codec::MaxEncodedLen>::max_encoded_len(),
                        ),
                )
                .saturating_add(1)
        }
    }
};
impl core::fmt::Debug for RuntimeParametersKey {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::InflationAV(ref a0) => {
                fmt.debug_tuple("RuntimeParametersKey::InflationAV").field(a0).finish()
            }
            Self::Inflation2FG(ref a0) => {
                fmt.debug_tuple("RuntimeParametersKey::Inflation2FG").field(a0).finish()
            }
            _ => Ok(()),
        }
    }
}
#[allow(non_upper_case_globals, deprecated, unused_attributes, unused_qualifications)]
const _: () = {
    impl ::scale_info::TypeInfo for RuntimeParametersKey {
        type Identity = Self;
        fn type_info() -> ::scale_info::Type {
            ::scale_info::Type::builder()
                .path(
                    ::scale_info::Path::new_with_replace(
                        "RuntimeParametersKey",
                        "polkadot_test",
                        &[],
                    ),
                )
                .type_params(::alloc::vec::Vec::new())
                .variant(
                    ::scale_info::build::Variants::new()
                        .variant(
                            "InflationAV",
                            |v| {
                                v
                                    .index(0u8 as ::core::primitive::u8)
                                    .fields(
                                        ::scale_info::build::Fields::unnamed()
                                            .field(|f| {
                                                f
                                                    .ty::<
                                                        <dynamic_params::inflationAV::Parameters as polkadot_sdk::frame_support::traits::dynamic_params::AggregatedKeyValue>::Key,
                                                    >()
                                                    .type_name(
                                                        "<dynamic_params::inflationAV::Parameters as polkadot_sdk::frame_support\n::traits::dynamic_params::AggregatedKeyValue>::Key",
                                                    )
                                            }),
                                    )
                                    .docs(
                                        &[
                                            "Parameters used to calculate era payouts, see",
                                            "[`polkadot_runtime_common::impls::EraPayoutParams`].",
                                        ],
                                    )
                            },
                        )
                        .variant(
                            "Inflation2FG",
                            |v| {
                                v
                                    .index(1u8 as ::core::primitive::u8)
                                    .fields(
                                        ::scale_info::build::Fields::unnamed()
                                            .field(|f| {
                                                f
                                                    .ty::<
                                                        <dynamic_params::inflation2FG::Parameters as polkadot_sdk::frame_support::traits::dynamic_params::AggregatedKeyValue>::Key,
                                                    >()
                                                    .type_name(
                                                        "<dynamic_params::inflation2FG::Parameters as polkadot_sdk::\nframe_support::traits::dynamic_params::AggregatedKeyValue>::Key",
                                                    )
                                            }),
                                    )
                            },
                        ),
                )
        }
    }
};
#[doc(hidden)]
pub enum RuntimeParametersValue {
    /// Parameters used to calculate era payouts, see
    /// [`polkadot_runtime_common::impls::EraPayoutParams`].
    #[codec(index = 0)]
    InflationAV(
        <dynamic_params::inflationAV::Parameters as polkadot_sdk::frame_support::traits::dynamic_params::AggregatedKeyValue>::Value,
    ),
    #[codec(index = 1)]
    Inflation2FG(
        <dynamic_params::inflation2FG::Parameters as polkadot_sdk::frame_support::traits::dynamic_params::AggregatedKeyValue>::Value,
    ),
}
#[automatically_derived]
impl ::core::clone::Clone for RuntimeParametersValue {
    #[inline]
    fn clone(&self) -> RuntimeParametersValue {
        match self {
            RuntimeParametersValue::InflationAV(__self_0) => {
                RuntimeParametersValue::InflationAV(
                    ::core::clone::Clone::clone(__self_0),
                )
            }
            RuntimeParametersValue::Inflation2FG(__self_0) => {
                RuntimeParametersValue::Inflation2FG(
                    ::core::clone::Clone::clone(__self_0),
                )
            }
        }
    }
}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for RuntimeParametersValue {}
#[automatically_derived]
impl ::core::cmp::PartialEq for RuntimeParametersValue {
    #[inline]
    fn eq(&self, other: &RuntimeParametersValue) -> bool {
        let __self_discr = ::core::intrinsics::discriminant_value(self);
        let __arg1_discr = ::core::intrinsics::discriminant_value(other);
        __self_discr == __arg1_discr
            && match (self, other) {
                (
                    RuntimeParametersValue::InflationAV(__self_0),
                    RuntimeParametersValue::InflationAV(__arg1_0),
                ) => __self_0 == __arg1_0,
                (
                    RuntimeParametersValue::Inflation2FG(__self_0),
                    RuntimeParametersValue::Inflation2FG(__arg1_0),
                ) => __self_0 == __arg1_0,
                _ => unsafe { ::core::intrinsics::unreachable() }
            }
    }
}
#[automatically_derived]
impl ::core::cmp::Eq for RuntimeParametersValue {
    #[inline]
    #[doc(hidden)]
    #[coverage(off)]
    fn assert_receiver_is_total_eq(&self) -> () {
        let _: ::core::cmp::AssertParamIsEq<
            <dynamic_params::inflationAV::Parameters as polkadot_sdk::frame_support::traits::dynamic_params::AggregatedKeyValue>::Value,
        >;
        let _: ::core::cmp::AssertParamIsEq<
            <dynamic_params::inflation2FG::Parameters as polkadot_sdk::frame_support::traits::dynamic_params::AggregatedKeyValue>::Value,
        >;
    }
}
#[allow(deprecated)]
const _: () = {
    #[automatically_derived]
    impl ::codec::Encode for RuntimeParametersValue {
        fn size_hint(&self) -> usize {
            1_usize
                + match *self {
                    RuntimeParametersValue::InflationAV(ref aa) => {
                        0_usize.saturating_add(::codec::Encode::size_hint(aa))
                    }
                    RuntimeParametersValue::Inflation2FG(ref aa) => {
                        0_usize.saturating_add(::codec::Encode::size_hint(aa))
                    }
                    _ => 0_usize,
                }
        }
        fn encode_to<__CodecOutputEdqy: ::codec::Output + ?::core::marker::Sized>(
            &self,
            __codec_dest_edqy: &mut __CodecOutputEdqy,
        ) {
            #[automatically_derived]
            const _: () = {
                #[allow(clippy::unnecessary_cast)]
                #[allow(clippy::cast_possible_truncation)]
                const indices: [(usize, &'static str); 2usize] = [
                    ((0usize) as ::core::primitive::usize, "InflationAV"),
                    ((1usize) as ::core::primitive::usize, "Inflation2FG"),
                ];
                const fn search_for_invalid_index(
                    array: &[(usize, &'static str); 2usize],
                ) -> (bool, usize) {
                    let mut i = 0;
                    while i < 2usize {
                        if array[i].0 > 255 {
                            return (true, i);
                        }
                        i += 1;
                    }
                    (false, 0)
                }
                const INVALID_INDEX: (bool, usize) = search_for_invalid_index(&indices);
                if INVALID_INDEX.0 {
                    let msg = ::const_format::pmr::__AssertStr {
                        x: {
                            use ::const_format::__cf_osRcTFl4A;
                            ({
                                #[doc(hidden)]
                                #[allow(unused_mut, non_snake_case)]
                                const CONCATP_NHPMWYD3NJA: &[__cf_osRcTFl4A::pmr::PArgument] = {
                                    let fmt = __cf_osRcTFl4A::pmr::FormattingFlags::NEW;
                                    &[
                                        __cf_osRcTFl4A::pmr::PConvWrapper("Found variant `")
                                            .to_pargument_display(fmt),
                                        __cf_osRcTFl4A::pmr::PConvWrapper(
                                                indices[INVALID_INDEX.1].1,
                                            )
                                            .to_pargument_display(fmt),
                                        __cf_osRcTFl4A::pmr::PConvWrapper("` with invalid index: `")
                                            .to_pargument_display(fmt),
                                        __cf_osRcTFl4A::pmr::PConvWrapper(
                                                indices[INVALID_INDEX.1].0,
                                            )
                                            .to_pargument_display(fmt),
                                        __cf_osRcTFl4A::pmr::PConvWrapper(
                                                "`. Max supported index is 255.",
                                            )
                                            .to_pargument_display(fmt),
                                    ]
                                };
                                {
                                    #[doc(hidden)]
                                    const ARR_LEN: usize = ::const_format::pmr::PArgument::calc_len(
                                        CONCATP_NHPMWYD3NJA,
                                    );
                                    #[doc(hidden)]
                                    const CONCAT_ARR: &::const_format::pmr::LenAndArray<
                                        [u8; ARR_LEN],
                                    > = &::const_format::pmr::__priv_concatenate(
                                        CONCATP_NHPMWYD3NJA,
                                    );
                                    #[doc(hidden)]
                                    #[allow(clippy::transmute_ptr_to_ptr)]
                                    const CONCAT_STR: &str = unsafe {
                                        let slice = ::const_format::pmr::transmute::<
                                            &[u8; ARR_LEN],
                                            &[u8; CONCAT_ARR.len],
                                        >(&CONCAT_ARR.array);
                                        {
                                            let bytes: &'static [::const_format::pmr::u8] = slice;
                                            let string: &'static ::const_format::pmr::str = {
                                                ::const_format::__hidden_utils::PtrToRef {
                                                    ptr: bytes as *const [::const_format::pmr::u8] as *const str,
                                                }
                                                    .reff
                                            };
                                            string
                                        }
                                    };
                                    CONCAT_STR
                                }
                            })
                        },
                    }
                        .x;
                    {
                        ::core::panicking::panic_display(&msg);
                    };
                }
                const fn duplicate_info(
                    array: &[(usize, &'static str); 2usize],
                ) -> (bool, usize, usize) {
                    let len = 2usize;
                    let mut i = 0usize;
                    while i < len {
                        let mut j = i + 1;
                        while j < len {
                            if array[i].0 == array[j].0 {
                                return (true, i, j);
                            }
                            j += 1;
                        }
                        i += 1;
                    }
                    (false, 0, 0)
                }
                const DUP_INFO: (bool, usize, usize) = duplicate_info(&indices);
                if DUP_INFO.0 {
                    let msg = ::const_format::pmr::__AssertStr {
                        x: {
                            use ::const_format::__cf_osRcTFl4A;
                            ({
                                #[doc(hidden)]
                                #[allow(unused_mut, non_snake_case)]
                                const CONCATP_NHPMWYD3NJA: &[__cf_osRcTFl4A::pmr::PArgument] = {
                                    let fmt = __cf_osRcTFl4A::pmr::FormattingFlags::NEW;
                                    &[
                                        __cf_osRcTFl4A::pmr::PConvWrapper(
                                                "Found variants that have duplicate indexes. Both `",
                                            )
                                            .to_pargument_display(fmt),
                                        __cf_osRcTFl4A::pmr::PConvWrapper(indices[DUP_INFO.1].1)
                                            .to_pargument_display(fmt),
                                        __cf_osRcTFl4A::pmr::PConvWrapper("` and `")
                                            .to_pargument_display(fmt),
                                        __cf_osRcTFl4A::pmr::PConvWrapper(indices[DUP_INFO.2].1)
                                            .to_pargument_display(fmt),
                                        __cf_osRcTFl4A::pmr::PConvWrapper("` have the index `")
                                            .to_pargument_display(fmt),
                                        __cf_osRcTFl4A::pmr::PConvWrapper(indices[DUP_INFO.1].0)
                                            .to_pargument_display(fmt),
                                        __cf_osRcTFl4A::pmr::PConvWrapper(
                                                "`. Use different indexes for each variant.",
                                            )
                                            .to_pargument_display(fmt),
                                    ]
                                };
                                {
                                    #[doc(hidden)]
                                    const ARR_LEN: usize = ::const_format::pmr::PArgument::calc_len(
                                        CONCATP_NHPMWYD3NJA,
                                    );
                                    #[doc(hidden)]
                                    const CONCAT_ARR: &::const_format::pmr::LenAndArray<
                                        [u8; ARR_LEN],
                                    > = &::const_format::pmr::__priv_concatenate(
                                        CONCATP_NHPMWYD3NJA,
                                    );
                                    #[doc(hidden)]
                                    #[allow(clippy::transmute_ptr_to_ptr)]
                                    const CONCAT_STR: &str = unsafe {
                                        let slice = ::const_format::pmr::transmute::<
                                            &[u8; ARR_LEN],
                                            &[u8; CONCAT_ARR.len],
                                        >(&CONCAT_ARR.array);
                                        {
                                            let bytes: &'static [::const_format::pmr::u8] = slice;
                                            let string: &'static ::const_format::pmr::str = {
                                                ::const_format::__hidden_utils::PtrToRef {
                                                    ptr: bytes as *const [::const_format::pmr::u8] as *const str,
                                                }
                                                    .reff
                                            };
                                            string
                                        }
                                    };
                                    CONCAT_STR
                                }
                            })
                        },
                    }
                        .x;
                    {
                        ::core::panicking::panic_display(&msg);
                    };
                }
            };
            match *self {
                RuntimeParametersValue::InflationAV(ref aa) => {
                    #[allow(clippy::unnecessary_cast)]
                    __codec_dest_edqy.push_byte((0usize) as ::core::primitive::u8);
                    ::codec::Encode::encode_to(aa, __codec_dest_edqy);
                }
                RuntimeParametersValue::Inflation2FG(ref aa) => {
                    #[allow(clippy::unnecessary_cast)]
                    __codec_dest_edqy.push_byte((1usize) as ::core::primitive::u8);
                    ::codec::Encode::encode_to(aa, __codec_dest_edqy);
                }
                _ => {}
            }
        }
    }
    #[automatically_derived]
    impl ::codec::EncodeLike for RuntimeParametersValue {}
};
#[allow(deprecated)]
const _: () = {
    #[automatically_derived]
    impl ::codec::Decode for RuntimeParametersValue {
        fn decode<__CodecInputEdqy: ::codec::Input>(
            __codec_input_edqy: &mut __CodecInputEdqy,
        ) -> ::core::result::Result<Self, ::codec::Error> {
            #[automatically_derived]
            const _: () = {
                #[allow(clippy::unnecessary_cast)]
                #[allow(clippy::cast_possible_truncation)]
                const indices: [(usize, &'static str); 2usize] = [
                    ((0usize) as ::core::primitive::usize, "InflationAV"),
                    ((1usize) as ::core::primitive::usize, "Inflation2FG"),
                ];
                const fn search_for_invalid_index(
                    array: &[(usize, &'static str); 2usize],
                ) -> (bool, usize) {
                    let mut i = 0;
                    while i < 2usize {
                        if array[i].0 > 255 {
                            return (true, i);
                        }
                        i += 1;
                    }
                    (false, 0)
                }
                const INVALID_INDEX: (bool, usize) = search_for_invalid_index(&indices);
                if INVALID_INDEX.0 {
                    let msg = ::const_format::pmr::__AssertStr {
                        x: {
                            use ::const_format::__cf_osRcTFl4A;
                            ({
                                #[doc(hidden)]
                                #[allow(unused_mut, non_snake_case)]
                                const CONCATP_NHPMWYD3NJA: &[__cf_osRcTFl4A::pmr::PArgument] = {
                                    let fmt = __cf_osRcTFl4A::pmr::FormattingFlags::NEW;
                                    &[
                                        __cf_osRcTFl4A::pmr::PConvWrapper("Found variant `")
                                            .to_pargument_display(fmt),
                                        __cf_osRcTFl4A::pmr::PConvWrapper(
                                                indices[INVALID_INDEX.1].1,
                                            )
                                            .to_pargument_display(fmt),
                                        __cf_osRcTFl4A::pmr::PConvWrapper("` with invalid index: `")
                                            .to_pargument_display(fmt),
                                        __cf_osRcTFl4A::pmr::PConvWrapper(
                                                indices[INVALID_INDEX.1].0,
                                            )
                                            .to_pargument_display(fmt),
                                        __cf_osRcTFl4A::pmr::PConvWrapper(
                                                "`. Max supported index is 255.",
                                            )
                                            .to_pargument_display(fmt),
                                    ]
                                };
                                {
                                    #[doc(hidden)]
                                    const ARR_LEN: usize = ::const_format::pmr::PArgument::calc_len(
                                        CONCATP_NHPMWYD3NJA,
                                    );
                                    #[doc(hidden)]
                                    const CONCAT_ARR: &::const_format::pmr::LenAndArray<
                                        [u8; ARR_LEN],
                                    > = &::const_format::pmr::__priv_concatenate(
                                        CONCATP_NHPMWYD3NJA,
                                    );
                                    #[doc(hidden)]
                                    #[allow(clippy::transmute_ptr_to_ptr)]
                                    const CONCAT_STR: &str = unsafe {
                                        let slice = ::const_format::pmr::transmute::<
                                            &[u8; ARR_LEN],
                                            &[u8; CONCAT_ARR.len],
                                        >(&CONCAT_ARR.array);
                                        {
                                            let bytes: &'static [::const_format::pmr::u8] = slice;
                                            let string: &'static ::const_format::pmr::str = {
                                                ::const_format::__hidden_utils::PtrToRef {
                                                    ptr: bytes as *const [::const_format::pmr::u8] as *const str,
                                                }
                                                    .reff
                                            };
                                            string
                                        }
                                    };
                                    CONCAT_STR
                                }
                            })
                        },
                    }
                        .x;
                    {
                        ::core::panicking::panic_display(&msg);
                    };
                }
                const fn duplicate_info(
                    array: &[(usize, &'static str); 2usize],
                ) -> (bool, usize, usize) {
                    let len = 2usize;
                    let mut i = 0usize;
                    while i < len {
                        let mut j = i + 1;
                        while j < len {
                            if array[i].0 == array[j].0 {
                                return (true, i, j);
                            }
                            j += 1;
                        }
                        i += 1;
                    }
                    (false, 0, 0)
                }
                const DUP_INFO: (bool, usize, usize) = duplicate_info(&indices);
                if DUP_INFO.0 {
                    let msg = ::const_format::pmr::__AssertStr {
                        x: {
                            use ::const_format::__cf_osRcTFl4A;
                            ({
                                #[doc(hidden)]
                                #[allow(unused_mut, non_snake_case)]
                                const CONCATP_NHPMWYD3NJA: &[__cf_osRcTFl4A::pmr::PArgument] = {
                                    let fmt = __cf_osRcTFl4A::pmr::FormattingFlags::NEW;
                                    &[
                                        __cf_osRcTFl4A::pmr::PConvWrapper(
                                                "Found variants that have duplicate indexes. Both `",
                                            )
                                            .to_pargument_display(fmt),
                                        __cf_osRcTFl4A::pmr::PConvWrapper(indices[DUP_INFO.1].1)
                                            .to_pargument_display(fmt),
                                        __cf_osRcTFl4A::pmr::PConvWrapper("` and `")
                                            .to_pargument_display(fmt),
                                        __cf_osRcTFl4A::pmr::PConvWrapper(indices[DUP_INFO.2].1)
                                            .to_pargument_display(fmt),
                                        __cf_osRcTFl4A::pmr::PConvWrapper("` have the index `")
                                            .to_pargument_display(fmt),
                                        __cf_osRcTFl4A::pmr::PConvWrapper(indices[DUP_INFO.1].0)
                                            .to_pargument_display(fmt),
                                        __cf_osRcTFl4A::pmr::PConvWrapper(
                                                "`. Use different indexes for each variant.",
                                            )
                                            .to_pargument_display(fmt),
                                    ]
                                };
                                {
                                    #[doc(hidden)]
                                    const ARR_LEN: usize = ::const_format::pmr::PArgument::calc_len(
                                        CONCATP_NHPMWYD3NJA,
                                    );
                                    #[doc(hidden)]
                                    const CONCAT_ARR: &::const_format::pmr::LenAndArray<
                                        [u8; ARR_LEN],
                                    > = &::const_format::pmr::__priv_concatenate(
                                        CONCATP_NHPMWYD3NJA,
                                    );
                                    #[doc(hidden)]
                                    #[allow(clippy::transmute_ptr_to_ptr)]
                                    const CONCAT_STR: &str = unsafe {
                                        let slice = ::const_format::pmr::transmute::<
                                            &[u8; ARR_LEN],
                                            &[u8; CONCAT_ARR.len],
                                        >(&CONCAT_ARR.array);
                                        {
                                            let bytes: &'static [::const_format::pmr::u8] = slice;
                                            let string: &'static ::const_format::pmr::str = {
                                                ::const_format::__hidden_utils::PtrToRef {
                                                    ptr: bytes as *const [::const_format::pmr::u8] as *const str,
                                                }
                                                    .reff
                                            };
                                            string
                                        }
                                    };
                                    CONCAT_STR
                                }
                            })
                        },
                    }
                        .x;
                    {
                        ::core::panicking::panic_display(&msg);
                    };
                }
            };
            match __codec_input_edqy
                .read_byte()
                .map_err(|e| {
                    e
                        .chain(
                            "Could not decode `RuntimeParametersValue`, failed to read variant byte",
                        )
                })?
            {
                #[allow(clippy::unnecessary_cast)]
                #[allow(clippy::cast_possible_truncation)]
                __codec_x_edqy if __codec_x_edqy
                    == (0usize) as ::core::primitive::u8 => {
                    #[allow(clippy::redundant_closure_call)]
                    return (move || {
                        ::core::result::Result::Ok(
                            RuntimeParametersValue::InflationAV({
                                let __codec_res_edqy = <<dynamic_params::inflationAV::Parameters as polkadot_sdk::frame_support::traits::dynamic_params::AggregatedKeyValue>::Value as ::codec::Decode>::decode(
                                    __codec_input_edqy,
                                );
                                match __codec_res_edqy {
                                    ::core::result::Result::Err(e) => {
                                        return ::core::result::Result::Err(
                                            e
                                                .chain(
                                                    "Could not decode `RuntimeParametersValue::InflationAV.0`",
                                                ),
                                        );
                                    }
                                    ::core::result::Result::Ok(__codec_res_edqy) => {
                                        __codec_res_edqy
                                    }
                                }
                            }),
                        )
                    })();
                }
                #[allow(clippy::unnecessary_cast)]
                #[allow(clippy::cast_possible_truncation)]
                __codec_x_edqy if __codec_x_edqy
                    == (1usize) as ::core::primitive::u8 => {
                    #[allow(clippy::redundant_closure_call)]
                    return (move || {
                        ::core::result::Result::Ok(
                            RuntimeParametersValue::Inflation2FG({
                                let __codec_res_edqy = <<dynamic_params::inflation2FG::Parameters as polkadot_sdk::frame_support::traits::dynamic_params::AggregatedKeyValue>::Value as ::codec::Decode>::decode(
                                    __codec_input_edqy,
                                );
                                match __codec_res_edqy {
                                    ::core::result::Result::Err(e) => {
                                        return ::core::result::Result::Err(
                                            e
                                                .chain(
                                                    "Could not decode `RuntimeParametersValue::Inflation2FG.0`",
                                                ),
                                        );
                                    }
                                    ::core::result::Result::Ok(__codec_res_edqy) => {
                                        __codec_res_edqy
                                    }
                                }
                            }),
                        )
                    })();
                }
                _ => {
                    #[allow(clippy::redundant_closure_call)]
                    return (move || {
                        ::core::result::Result::Err(
                            <_ as ::core::convert::Into<
                                _,
                            >>::into(
                                "Could not decode `RuntimeParametersValue`, variant doesn't exist",
                            ),
                        )
                    })();
                }
            }
        }
    }
};
#[allow(deprecated)]
const _: () = {
    fn check_struct() {
        fn check_field<T: ::codec::DecodeWithMemTracking>() {}
        check_field::<
            <dynamic_params::inflationAV::Parameters as polkadot_sdk::frame_support::traits::dynamic_params::AggregatedKeyValue>::Value,
        >();
        check_field::<
            <dynamic_params::inflation2FG::Parameters as polkadot_sdk::frame_support::traits::dynamic_params::AggregatedKeyValue>::Value,
        >();
    }
    #[automatically_derived]
    impl ::codec::DecodeWithMemTracking for RuntimeParametersValue {}
};
const _: () = {
    #[automatically_derived]
    impl ::codec::MaxEncodedLen for RuntimeParametersValue {
        fn max_encoded_len() -> ::core::primitive::usize {
            0_usize
                .max(
                    0_usize
                        .saturating_add(
                            <<dynamic_params::inflationAV::Parameters as polkadot_sdk::frame_support::traits::dynamic_params::AggregatedKeyValue>::Value as ::codec::MaxEncodedLen>::max_encoded_len(),
                        ),
                )
                .max(
                    0_usize
                        .saturating_add(
                            <<dynamic_params::inflation2FG::Parameters as polkadot_sdk::frame_support::traits::dynamic_params::AggregatedKeyValue>::Value as ::codec::MaxEncodedLen>::max_encoded_len(),
                        ),
                )
                .saturating_add(1)
        }
    }
};
impl core::fmt::Debug for RuntimeParametersValue {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::InflationAV(ref a0) => {
                fmt.debug_tuple("RuntimeParametersValue::InflationAV").field(a0).finish()
            }
            Self::Inflation2FG(ref a0) => {
                fmt.debug_tuple("RuntimeParametersValue::Inflation2FG")
                    .field(a0)
                    .finish()
            }
            _ => Ok(()),
        }
    }
}
#[allow(non_upper_case_globals, deprecated, unused_attributes, unused_qualifications)]
const _: () = {
    impl ::scale_info::TypeInfo for RuntimeParametersValue {
        type Identity = Self;
        fn type_info() -> ::scale_info::Type {
            ::scale_info::Type::builder()
                .path(
                    ::scale_info::Path::new_with_replace(
                        "RuntimeParametersValue",
                        "polkadot_test",
                        &[],
                    ),
                )
                .type_params(::alloc::vec::Vec::new())
                .variant(
                    ::scale_info::build::Variants::new()
                        .variant(
                            "InflationAV",
                            |v| {
                                v
                                    .index(0u8 as ::core::primitive::u8)
                                    .fields(
                                        ::scale_info::build::Fields::unnamed()
                                            .field(|f| {
                                                f
                                                    .ty::<
                                                        <dynamic_params::inflationAV::Parameters as polkadot_sdk::frame_support::traits::dynamic_params::AggregatedKeyValue>::Value,
                                                    >()
                                                    .type_name(
                                                        "<dynamic_params::inflationAV::Parameters as polkadot_sdk::frame_support\n::traits::dynamic_params::AggregatedKeyValue>::Value",
                                                    )
                                            }),
                                    )
                                    .docs(
                                        &[
                                            "Parameters used to calculate era payouts, see",
                                            "[`polkadot_runtime_common::impls::EraPayoutParams`].",
                                        ],
                                    )
                            },
                        )
                        .variant(
                            "Inflation2FG",
                            |v| {
                                v
                                    .index(1u8 as ::core::primitive::u8)
                                    .fields(
                                        ::scale_info::build::Fields::unnamed()
                                            .field(|f| {
                                                f
                                                    .ty::<
                                                        <dynamic_params::inflation2FG::Parameters as polkadot_sdk::frame_support::traits::dynamic_params::AggregatedKeyValue>::Value,
                                                    >()
                                                    .type_name(
                                                        "<dynamic_params::inflation2FG::Parameters as polkadot_sdk::\nframe_support::traits::dynamic_params::AggregatedKeyValue>::Value",
                                                    )
                                            }),
                                    )
                            },
                        ),
                )
        }
    }
};
impl polkadot_sdk::frame_support::traits::dynamic_params::AggregatedKeyValue
for RuntimeParameters {
    type Key = RuntimeParametersKey;
    type Value = RuntimeParametersValue;
    fn into_parts(self) -> (Self::Key, Option<Self::Value>) {
        match self {
            RuntimeParameters::InflationAV(parameter) => {
                let (key, value) = parameter.into_parts();
                (
                    RuntimeParametersKey::InflationAV(key),
                    value.map(RuntimeParametersValue::InflationAV),
                )
            }
            RuntimeParameters::Inflation2FG(parameter) => {
                let (key, value) = parameter.into_parts();
                (
                    RuntimeParametersKey::Inflation2FG(key),
                    value.map(RuntimeParametersValue::Inflation2FG),
                )
            }
        }
    }
}
impl ::core::convert::From<
    <dynamic_params::inflationAV::Parameters as polkadot_sdk::frame_support::traits::dynamic_params::AggregatedKeyValue>::Key,
> for RuntimeParametersKey {
    fn from(
        key: <dynamic_params::inflationAV::Parameters as polkadot_sdk::frame_support::traits::dynamic_params::AggregatedKeyValue>::Key,
    ) -> Self {
        RuntimeParametersKey::InflationAV(key)
    }
}
impl ::core::convert::TryFrom<RuntimeParametersValue>
for <dynamic_params::inflationAV::Parameters as polkadot_sdk::frame_support::traits::dynamic_params::AggregatedKeyValue>::Value {
    type Error = ();
    fn try_from(value: RuntimeParametersValue) -> Result<Self, Self::Error> {
        match value {
            RuntimeParametersValue::InflationAV(value) => Ok(value),
            _ => Err(()),
        }
    }
}
impl ::core::convert::From<
    <dynamic_params::inflation2FG::Parameters as polkadot_sdk::frame_support::traits::dynamic_params::AggregatedKeyValue>::Key,
> for RuntimeParametersKey {
    fn from(
        key: <dynamic_params::inflation2FG::Parameters as polkadot_sdk::frame_support::traits::dynamic_params::AggregatedKeyValue>::Key,
    ) -> Self {
        RuntimeParametersKey::Inflation2FG(key)
    }
}
impl ::core::convert::TryFrom<RuntimeParametersValue>
for <dynamic_params::inflation2FG::Parameters as polkadot_sdk::frame_support::traits::dynamic_params::AggregatedKeyValue>::Value {
    type Error = ();
    fn try_from(value: RuntimeParametersValue) -> Result<Self, Self::Error> {
        match value {
            RuntimeParametersValue::Inflation2FG(value) => Ok(value),
            _ => Err(()),
        }
    }
}
