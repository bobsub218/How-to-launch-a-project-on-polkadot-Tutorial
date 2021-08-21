#![cfg_attr(not(feature = "std"), no_std)]
#![recursion_limit = "256"]

// Make the WASM binary available.
#[cfg(feature = "std")]
include!(concat!(env!("OUT_DIR"), "/wasm_binary.rs"));

use sp_api::impl_runtime_apis;
use sp_runtime::{
	create_runtime_str, generic, impl_opaque_keys,
	transaction_validity::{TransactionSource, TransactionValidity},
	ApplyExtrinsicResult,
};

//Place for additional imports

impl_opaque_keys! {
	pub struct SessionKeys {}
}

// This runtime version.
pub const VERSION: RuntimeVersion = RuntimeVersion {
	spec_name: create_runtime_str!("blaize-test-project"),
	impl_name: create_runtime_str!("blaize-test-project"),
	authoring_version: 1,
	spec_version: 1,
	impl_version: 1,
	apis: RUNTIME_API_VERSIONS,
	transaction_version: 1,
};

impl frame_system::Trait for Runtime {
    // Place for main trait interface
}

//Place for additional custom traits declarations

impl cumulus_parachain_upgrade::Trait for Runtime {
	type Event = Event;
	type OnValidationFunctionParams = ();
}

impl parachain_info::Trait for Runtime {}

// Configure the pallet template in pallets/template.
impl template::Trait for Runtime {
	type Event = Event;
}

construct_runtime! {
	pub enum Runtime where
		Block = Block,
		NodeBlock = opaque::Block,
		UncheckedExtrinsic = UncheckedExtrinsic
	{
		System: frame_system::{Module, Call, Storage, Config, Event<T>},
		Timestamp: pallet_timestamp::{Module, Call, Storage, Inherent},
		Balances: pallet_balances::{Module, Call, Storage, Config<T>, Event<T>},
		ParachainUpgrade: cumulus_parachain_upgrade::{Module, Call, Storage, Inherent, Event},
		ParachainInfo: parachain_info::{Module, Storage, Config},
	}
}

impl_runtime_apis! {
    //Traits implementation
}

cumulus_runtime::register_validate_block!(Block, Executive);