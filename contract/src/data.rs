use crate::*;
use near_sdk::{
    serde_json::{from_str, Value},
    PromiseOrValue,
};

pub trait ExtSelf {
    fn loop_await_return(&mut self, id: String) -> PromiseOrValue<String>;
    fn demo_callback(&mut self) -> bool;
}

type URL = String;
const MAX_ITERATIONS: u32 = 15;
#[derive(
    BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone, PanicOnDefault, Debug,
)]
#[serde(crate = "near_sdk::serde")]
pub struct OracleData {
    pub id: String,
    pub url: URL,
    pub data: String,
    pub timestamp: u64,
    pub executed: bool,
    pub return_value: Option<String>,
    pub creator: AccountId,
}
//view
#[near_bindgen]
impl Contract {
    pub fn get_queued_data(&self, from_index: Option<u64>, limit: Option<u64>) -> Vec<OracleData> {
        let mut data: Vec<OracleData> = vec![];
        self.queued_data
            .iter()
            .skip(from_index.unwrap_or(0) as usize)
            .take(limit.unwrap_or(50) as usize)
            .for_each(|(_, v)| {
                data.push(v);
            });
        data
    }
    pub fn get_queued_data_by_id(&self, id: String) -> Option<OracleData> {
        self.queued_data.get(&id)
    }
    pub fn get_queued_data_by_url(
        &self,
        url: URL,
        from_index: Option<u64>,
        limit: Option<u64>,
    ) -> Vec<OracleData> {
        let mut data: Vec<OracleData> = vec![];
        self.queued_data
            .iter()
            .skip(from_index.unwrap_or(0) as usize)
            .take(limit.unwrap_or(50) as usize)
            .for_each(|(_, v)| {
                if v.url == url {
                    data.push(v);
                }
            });
        data
    }
    pub fn get_queued_data_by_executed(
        &self,
        executed: bool,
        from_index: Option<u64>,
        limit: Option<u64>,
    ) -> Vec<OracleData> {
        let mut data: Vec<OracleData> = vec![];
        self.queued_data
            .iter()
            .skip(from_index.unwrap_or(0) as usize)
            .take(limit.unwrap_or(50) as usize)
            .for_each(|(_, v)| {
                if v.executed == executed {
                    data.push(v);
                }
            });
        data
    }
}
impl Contract {
    pub fn internal_create_oracle(&mut self, url: URL, data: String) -> OracleData {
        let init_storage = env::storage_usage();
        let body_json: Value = from_str(&data).unwrap();
        //create id hash with timestamp and accountId

        //Vec<u8> to String
        let id = hex::encode(env::sha256(
            format!(
                "{}{}",
                hex::encode(env::random_seed()),
                env::predecessor_account_id()
            )
            .as_bytes(),
        ));
        let oracle_data = OracleData {
            id: id.to_string(),
            url,
            data: body_json.to_string(),
            timestamp: env::block_timestamp_ms(),
            executed: false,
            return_value: None,
            creator: env::predecessor_account_id(),
        };
        self.queued_data.insert(&id, &oracle_data);

        self.cal_storage(init_storage, &env::predecessor_account_id());
        oracle_data
    }
}
#[near_bindgen]
impl Contract {
    #[payable]
    pub fn create_oracle(&mut self, url: URL, data: String) -> OracleData {
        require!(
            env::attached_deposit() >= self.fee_per_call.0,
            "ERR_NOT_ENOUGH_DEPOSIT"
        );
        self.internal_create_oracle(url, data)
    }
    #[payable]
    pub fn create_oracle_await(&mut self, url: URL, data: String) -> PromiseOrValue<String> {
        require!(
            env::attached_deposit() >= self.fee_per_call.0,
            "ERR_NOT_ENOUGH_DEPOSIT"
        );
        let oracle_data = self.internal_create_oracle(url, data);
        PromiseOrValue::Promise(
            Self::ext(env::current_account_id())
                .with_unused_gas_weight(100)
                .loop_await_return(oracle_data.id, 0),
        )
    }
    #[payable]
    pub fn demo_power(&mut self) -> PromiseOrValue<String> {
        require!(
            env::attached_deposit() >= self.fee_per_call.0,
            "ERR_NOT_ENOUGH_DEPOSIT"
        );
        let oracle_data = self.internal_create_oracle(
            "https://api.coingecko.com/api/v3/simple/price?ids=near&vs_currencies=usd".to_string(),
            "{}".to_string(),
        );
        PromiseOrValue::Promise(
            Self::ext(env::current_account_id())
                .with_unused_gas_weight(100)
                .loop_await_return(oracle_data.id, 0)
                .then(
                    Self::ext(env::current_account_id())
                        .with_unused_gas_weight(100)
                        .demo_callback(),
                ),
        )
    }
    //calculate Gas cost of this function
    pub fn execute_oracle(&mut self, id: String, return_value: String) -> OracleData {
        /*  let init_storage = env::storage_usage(); */
        let mut oracle_data = self.queued_data.get(&id).unwrap();
        oracle_data.executed = true;
        oracle_data.return_value = Some(return_value);
        self.queued_data.insert(&oracle_data.id, &oracle_data);
        /*  self.cal_storage(init_storage, &oracle_data.creator); */
        oracle_data
    }
    pub fn execute_oracle_batch(&mut self, ids: Vec<String>, return_values: Vec<String>) {
        /*   let init_storage = env::storage_usage(); */
        for i in 0..ids.len() {
            let mut oracle_data = self.queued_data.get(&ids[i]).unwrap();
            oracle_data.executed = true;
            oracle_data.return_value = Some(return_values[i].to_string());
            self.queued_data.insert(&oracle_data.id, &oracle_data);
            /*    self.cal_storage(init_storage, &oracle_data.creator); */
        }
    }
    pub fn delete_oracle_batch(&mut self, ids: Vec<String>) {
        /*  let init_storage = env::storage_usage(); */
        for i in 0..ids.len() {
            let oracle_data = self.queued_data.get(&ids[i]).unwrap();
            self.queued_data.remove(&ids[i]);
            /*  self.cal_storage(init_storage, &oracle_data.creator); */
        }
    }
    //calculate Gas cost of this function
    pub fn delete_oracle(&mut self, id: String) -> bool {
        /*   let init_storage = env::storage_usage(); */

        if let Some(oracle_data) = self.queued_data.get(&id) {
            self.queued_data.remove(&id);
            /*  self.cal_storage(init_storage, &oracle_data.creator); */
            return true;
        }
        false
    }
}

/// loop await api return
#[near_bindgen]
impl Contract {
    pub fn loop_await_entry(&mut self, id: String) -> PromiseOrValue<Option<String>> {
        let data = self.get_queued_data_by_id(id.clone()).unwrap();
        if data.executed {
            PromiseOrValue::Value(Some(data.return_value.unwrap()))
        } else {
            PromiseOrValue::Promise(
                Self::ext(env::current_account_id())
                    .with_unused_gas_weight(100)
                    .loop_await_return(id.clone(), 0),
            )
        }
    }
    #[private]
    pub fn loop_await_return(&self, id: String, iteration: u32) -> PromiseOrValue<Option<String>> {
        if iteration > MAX_ITERATIONS {
            return PromiseOrValue::Value(None);
        }

        let data = self.get_queued_data_by_id(id.clone()).unwrap();
        if data.executed {
            PromiseOrValue::Value(Some(data.return_value.unwrap()))
        } else {
            PromiseOrValue::Promise(
                Self::ext(env::current_account_id())
                    .with_unused_gas_weight(100)
                    .loop_await_return(id.clone(), iteration + 1),
            )
        }
    }
    #[private]
    pub fn demo_callback(
        &mut self,
        #[callback_result] call_result: Result<String, near_sdk::PromiseError>,
    ) -> bool {
        // Return whether or not the promise succeeded using the method outlined in external.rs
        if call_result.is_err() {
            return false;
        } else {
            env::log_str(
                format!(
                    "You have just connected this contract to the real world by getting the near exchange rate from coingecko within this function call - , {}",
                    call_result.unwrap()
                )
                .as_str(),
            );
            return true;
        }
    }
}
