# How-to-launch-a-project-on-polkadot
In this tutorial I will explain how to develop and launch your dApp project on Polkadot.

##  WHY TO BUILD ONE DAPP ON POLKADOT?

Polkadot is a fast-growing ecosystem that enables cross-chain communication within its parachains. This interoperability and scalability will take blockchain technology to a new level; solving current problems such as high transition fees, rigid forks and low TPS.

In this tutorial I will explain how to develop and launch your dApp project on Polkadot.

## BUILDING A PARACHAIN FOR POLKADOT

For the creation of a parachain it is necessary to know the substrate.

Substrate and polkadot are written in Rust. As a result, it is possible to implement parachain development in Rust.

To facilitate parachain development, Polkadot provides two PDK (Parachain Development Kit). 

The first working and available PDK is called Substrate and the second Cumulus.

The basic tool set for the development of the polkadot parachain consists of:
* SUBSTRATE (newest version)_
* SOURCE CODE POLKADOT_
* WASM INTERPRETER AND WASM COMPILER_
* ROCOCÒ_

The first step for development is to set the version of the Substrate framework.

The best way is to use Parity Knowledge Base.

As soon as the environment is ready, you can start editing the code of the parachain model.

```
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
```

Substrate contains all the modules and frames needed for independent chain development, but does not have the required compatibility functionality with Polkadot. 

So you need to start using the Cumulus library.

Cumulus will add to the library the parachain code required when importing a substrate-based chain. 

This makes the chain compatible with the Polkadot environment.

If you got here, great, now you need ROCOCÒ. Because you will have to check your parachain, in fact the testnet ROCOCÒ, has been created to meet/test all the specifications required.

After your parachain passes the testnet exam, you will need a parachain slot to distribute it and connect to the Relay chain.

## DEVELOPING A DAPP ON POLKADOT WITH SUBSTRATE 

For creating the dApp for polkadot we will use FRAME Substrates because of the its wide use.

The basic toolkit for developing dApp polkadot consists of:
* SUBSTRATED (newest version)
* RUST
* !INK
* WASM INTERPRETER AND COMPILER WASM

Substrate FRAME is a code library that stores ready-made modules. 

To create a decentralized app on top of it, you need to combine the modules chosen in the framework runtime.

### ATTENTION! To enable smart contract functionality we should add a pallet _contracts pallet.
```
impl pallet_contracts::Trait for Runtime {
    type Time = Timestamp;
    type Randomness = RandomnessCollectiveFlip;
    type Currency = Balances;
    type Event = Event;
    type DetermineContractAddress = pallet_contracts::SimpleAddressDeterminer<Runtime>;
    type TrieIdGenerator = pallet_contracts::TrieIdFromParentCounter<Runtime>;
    type RentPayment = ();
    type SignedClaimHandicap = pallet_contracts::DefaultSignedClaimHandicap;
    type TombstoneDeposit = TombstoneDeposit;
    type StorageSizeOffset = pallet_contracts::DefaultStorageSizeOffset;
    type RentByteFee = RentByteFee;
    type RentDepositOffset = RentDepositOffset;
    type SurchargeReward = SurchargeReward;
    type MaxDepth = pallet_contracts::DefaultMaxDepth;
    type MaxValueSize = pallet_contracts::DefaultMaxValueSize;
    type WeightPrice = pallet_transaction_payment::Module<Self>;
}
```
 For customization, we can add all necessary imports and dependencies. 
 We can either add more custom pallets or develop custom strokes within this pallet and start configuration.

  ```
impl_runtime_apis! {
    impl pallet_contracts_rpc_runtime_api::ContractsApi<Block, AccountId, Balance, BlockNumber>
        for Runtime
    {
        fn call_the_contract(
            origin: AccountId,   //Contract caller (may be another contract)
            dest: AccountId,    //Contract address
            value: Balance,    //Analogue of Eth message.value
            gas_limit: u64,
            input_data: Vec<u8>,
        ) -> ContractExecResult {
            let (exec_result, gas_consumed) =
                Contracts::bare_call(origin, dest.into(), value, gas_limit, input_data);
            match exec_result {
                Ok(v) => ContractExecResult::Success {
                    flags: v.flags.bits(),
                    data: v.data,
                    gas_consumed: gas_consumed,
                },
                Err(_) => ContractExecResult::Error,
            }
        }
}
```
  
To deploy smart contracts for your dApp, there are two leading solutions (Moonbeam and Edgeware) both offering smart contracts running through the Polkadot environment.

If you’ve chosen the EVM Substrate pallet, a Moonbeam smart contract solution is better. 

In which an interoperable layer containing the current Ethereum toolbox called Frontier is used. Moonbeam will support all contracts written for the EVM environment.

Instead, if you’ve chosen Substrate FRAME, it’s better to use an Edgeware Smart Contract solution. 
It’s a substrate-based chain that will connect to the relay chain.
Thanks to its compilation in WASM it allows the execution of smart contracts.
  
## CREATE A CROSS-CHAIN BRIDGE ON POLKADOT 

Building a cross-chain bridge is a way to connect to the Polkadot ecosystem. 

Creating a cross-chain bridge means that the tokens move between chains or protocols, but in reality it’s all about a smart contract that burns tokens on one chain and minted them on the other.

### ATTENTION! To confirm any transaction, the contract requires a signature from a subset of validators.

Now let’s build our relay bridge from Ethereum to Polkadot. To build our project we will use !ink because it is the main language for smart contracts on Substrate. Alternatively you could use Rust.

The basic set of tools for the development of the bridge consists of:
* INTELLIGENT CONTRACTS
* SUBSTRATE PALLET (use relayer pallets)
* CONVALIDATION PROGRAMME

In the development of the bridge an important part is the validation program, which should take into account the parameter analyzing the number of validators (relays). Another important parameter is the threshold for validators.
 
The threshold is the limit of validators needed to approve the transaction. 
 
By setting the threshold function it is possible to specify the exact amount of funds that can be transferred from one side to the other per day.

Example of validation method for approval of cross-chain transactions:
```
// Validator method
#[ink(message)]
pub fn request_swap(&mut self, transfer_info: SwapMessage) {
    let caller: AccountId = self.env().caller();
    assert!(self.validators.get(&caller).is_some(), "Only Validator can send requests to swap assets");

    assert!(transfer_info.chain_id == self.chain_id, "Swap request's chain ID doesn't match contract's chain ID");

    assert!(self.check_expiration_time(transfer_info.timestamp.clone()), "Transaction can't be sent because of expiration time");

    assert!(self.check_asset(&transfer_info.asset), "Unknown asset is trying to transfer");

    let message_hash: Vec<u8> = self.hash_message(transfer_info.clone());

    let validators_who_approved_swap: Option<Vec<AccountId>> = self.get_validators_who_approved(&message_hash);
    match validators_who_approved_swap {
        Some(n) => {
            assert!(self.is_in(&n, &caller) == false, "This Validator has already sent approval");
            if (n.len() as u16) + 1 >= self.signature_threshold {
                self.make_swap(transfer_info.asset, transfer_info.amount, transfer_info.receiver);
                self.swap_requests.take(&message_hash);
            } else {
                let mut updated_validator_list: Vec<AccountId> = n.clone();
                updated_validator_list.push(caller);
                self.swap_requests.insert(message_hash, updated_validator_list);
            }
        },
        None => {
            let mut init_vec_of_validators: Vec<AccountId> = Vec::new();
            init_vec_of_validators.push(caller);
            self.swap_requests.insert(message_hash, init_vec_of_validators);
        }
    }
}
  ```
 Keep in mind that to interact with both sides of the bridge and the smart contracts on it, you’ll need a dApp.
 
 This is how the end user will transfer funds from one chain to another.

## CONCLUSION 

 In this tutorial we’ve seen how to build a parachain on Polkadot and use a substrate-based chain to create a dApp.
 In addition, we also saw the design/construction process of a cross-chain bridge between Ethereum and Polkadot.
 I hope that after reading this tutorial you have understood how you can join the Polkadot world and build into it.
