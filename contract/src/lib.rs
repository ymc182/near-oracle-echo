/*
 * Example smart contract written in RUST
 *
 * Learn more about writing NEAR smart contracts with Rust:
 * https://near-docs.io/develop/Contract
 *
 */

use data::Data;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LookupMap, UnorderedMap};

use near_sdk::serde::{Deserialize, Serialize};

use near_sdk::{
    env, near_bindgen, require, AccountId, Balance, BorshStorageKey, PanicOnDefault, StorageUsage,
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
    pub data: UnorderedMap<AccountId, Data>,
    pub data_map: LookupMap<AccountId, LookupMap<String, Data>>,
    pub accounts: LookupMap<AccountId, StorageBalance>,
    pub account_storage_usage: StorageUsage,
}

// Define the default, which automatically initializes the contract
#[derive(BorshSerialize, BorshStorageKey)]
enum StorageKey {
    Data,
    Accounts,
    DataMap,
    DataMapInner { account_hash: Vec<u8> },
}
// Implement the contract structure
#[near_bindgen]
impl Contract {
    #[init]
    pub fn new(owner_id: AccountId) -> Self {
        Self {
            owner_id,
            data: UnorderedMap::new(StorageKey::Data.try_to_vec().unwrap()),
            accounts: LookupMap::new(StorageKey::Accounts.try_to_vec().unwrap()),
            data_map: LookupMap::new(StorageKey::DataMap.try_to_vec().unwrap()),
            account_storage_usage: 0,
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
    use super::*;

    #[test]
    fn get_default_greeting() {
        let _contract = Contract::new("alice.testnet".parse().unwrap());
        // this test did not call set_greeting so should return the default "Hello" greeting
    }

    #[test]
    fn set_then_get_greeting() {}
}
