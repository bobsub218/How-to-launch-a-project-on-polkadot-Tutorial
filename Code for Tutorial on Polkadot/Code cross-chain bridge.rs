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