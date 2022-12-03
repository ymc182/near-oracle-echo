/*
 * Example smart contract written in RUST
 *
 * Learn more about writing NEAR smart contracts with Rust:
 * https://near-docs.io/develop/Contract
 *
 */

use data::OracleData;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LookupMap, LookupSet, UnorderedMap};

use near_sdk::json_types::U128;
use near_sdk::serde::{Deserialize, Serialize};

use near_sdk::{
    env, near_bindgen, require, AccountId, Balance, BorshStorageKey, PanicOnDefault, StorageUsage,
    ONE_NEAR,
};
use storage::StorageBalance;

// Define the default message
mod data;
mod storage;
// Define the contract structure
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    pub owner_id: AccountId,
    pub queued_data: UnorderedMap<String, OracleData>,
    pub accounts: LookupMap<AccountId, StorageBalance>,
    pub account_storage_usage: StorageUsage,
    pub whitelisted: LookupSet<AccountId>,
    pub fee_per_call: U128,
}

// Define the default, which automatically initializes the contract
#[derive(BorshSerialize, BorshStorageKey)]
enum StorageKey {
    OracleData,
    Whitelisted,
    Accounts,
}
// Implement the contract structure
#[near_bindgen]
impl Contract {
    #[init]
    pub fn new(owner_id: AccountId) -> Self {
        Self {
            owner_id,
            queued_data: UnorderedMap::new(StorageKey::OracleData.try_to_vec().unwrap()),
            whitelisted: LookupSet::new(StorageKey::Whitelisted.try_to_vec().unwrap()),
            accounts: LookupMap::new(StorageKey::Accounts.try_to_vec().unwrap()),
            account_storage_usage: 0,
            fee_per_call: U128(ONE_NEAR / 100),
        }
    }
    pub fn assert_owner(&self) {
        require!(
            self.owner_id == env::predecessor_account_id(),
            "ERR_ONLY_OWNER_ACCESS"
        );
    }
}

/*
 * The rest of this file holds the inline tests for the code above
 * Learn more about Rust tests: https://doc.rust-lang.org/book/ch11-01-writing-tests.html
 */
#[cfg(test)]
mod tests {
    use near_sdk::{
        json_types::U128,
        serde_json::{self, json, Value},
        test_utils::{accounts, VMContextBuilder},
        testing_env, ONE_NEAR,
    };

    use crate::storage::StorageManagement;

    use super::*;

    #[test]
    fn queue_oracle() {
        //near sdk testing env
        let context = VMContextBuilder::new()
            .signer_account_id(accounts(0))
            .predecessor_account_id(accounts(0))
            .attached_deposit(ONE_NEAR / 10)
            .build();
        testing_env!(context);

        let mut contract = Contract::new(accounts(0));
        contract.storage_deposit(Some(accounts(0)), None);
        let storage = contract.storage_balance_of(accounts(0)).unwrap();
        assert_eq!(storage.available, U128(ONE_NEAR / 10));

        let res = contract.create_oracle(
            "https://api.coingecko.com/api/v3/simple/price?ids=bitcoin&vs_currencies=usd"
                .to_string(),
            json!({"id": "bitcoin", "symbol": "btc", "name": "Bitcoin"}).to_string(),
        );
        let json: Value = serde_json::from_str(&res.data).unwrap();
        assert!(json["id"].as_str().unwrap() == "bitcoin");
        // this test did not call set_greeting so should return the default "Hello" greeting
    }
}
