use crate::*;

#[derive(
    BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone, PanicOnDefault, Debug,
)]
#[serde(crate = "near_sdk::serde")]
pub struct Data {
    pub string: String,
    //boosted: bool,
}

#[near_bindgen]
impl Contract {
    pub fn set_data(&mut self, string: String) -> Data {
        let init_storage = env::storage_usage();
        let data = Data { string };
        self.data.insert(&env::predecessor_account_id(), &data);
        self.cal_storage(init_storage, &env::predecessor_account_id());

        data
    }
    pub fn set_data_map(&mut self, data_id: String, string: String) -> Data {
        let init_storage = env::storage_usage();
        let mut user_data = self
            .data_map
            .get(&env::predecessor_account_id())
            .unwrap_or_else(|| {
                LookupMap::new(StorageKey::DataMapInner {
                    account_hash: env::sha256(env::predecessor_account_id().as_bytes()),
                })
            });
        let data = Data { string };
        user_data.insert(&data_id, &data);
        self.data_map
            .insert(&env::predecessor_account_id(), &user_data);
        self.cal_storage(init_storage, &env::predecessor_account_id());
        data
    }
    pub fn get_data(&self, account_id: AccountId) -> Data {
        self.data.get(&account_id).unwrap()
    }
    pub fn get_data_map(&self, account_id: AccountId, data_id: String) -> Data {
        let user_data = self.data_map.get(&account_id).unwrap();
        user_data.get(&data_id).unwrap()
    }

    pub fn cal_storage(&mut self, init_storage: u64, account_id: &AccountId) {
        if init_storage < env::storage_usage() {
            //storage used
            let total_used_storage = env::storage_usage() - init_storage;
            self.account_storage_usage += total_used_storage;
            let mut storage_available_by_user = self.accounts.get(account_id).unwrap();
            storage_available_by_user.available.0 = storage_available_by_user
                .available
                .0
                .checked_sub(total_used_storage as u128 * env::storage_byte_cost())
                .unwrap();
            self.accounts.insert(account_id, &storage_available_by_user);
        } else {
            //storage freed
            let total_freed_storage = init_storage - env::storage_usage();
            self.account_storage_usage -= total_freed_storage;
            let mut storage_available_by_user = self.accounts.get(account_id).unwrap();
            storage_available_by_user.available.0 = storage_available_by_user
                .available
                .0
                .checked_add(total_freed_storage as u128 * env::storage_byte_cost())
                .unwrap();
            self.accounts.insert(account_id, &storage_available_by_user);
        }
    }
}
