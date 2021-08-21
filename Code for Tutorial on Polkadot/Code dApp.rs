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

'''
For customization, we can add all necessary imports and dependencies, 
develop custom traits within this pallet or even add more custom pallets and start the configuration.
'''
impl_runtime_apis! {
    impl pallet_contracts_rpc_runtime_api::ContractsApi<Block, AccountId, Balance, BlockNumber>
        for Runtime
    {
        fn call_the_contract(
            origin: AccountId,  //Contract caller (may be another contract)
            dest: AccountId,   //Contract address
            value: Balance,   //Analogue of Eth message.value
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
