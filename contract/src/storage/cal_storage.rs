use crate::*;
#[near_bindgen]
impl Contract {
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
                .expect("ERR_NOT_ENOUGH_STORAGE_AVAILABLE_BALANCE");
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
