use std::collections::HashMap;
use crate::custtom_types::{PoolData};

pub struct Service {
    pool_map: HashMap<String, PoolData>,
    removed_pool_map: HashMap<String, PoolData>,
}

impl Service {
    pub fn new(init_state: State) -> Self {
        let mut pool_map = HashMap::new();
        let mut removed_pool_map = HashMap::new();

        for (key, pool_data) in init_state.pool_entries {
            pool_map.insert(key, pool_data);
        }

        for (key, pool_data) in init_state.removed_pool_entries {
            removed_pool_map.insert(key, pool_data);
        }

        Service {
            pool_map,
            removed_pool_map,
        }
    }

    pub fn get_pools(&self) -> &HashMap<String, PoolData> {
        &self.pool_map
    }

    pub fn get_removed_pools(&self) -> &HashMap<String, PoolData> {
        &self.removed_pool_map
    }

    pub fn put_pool(&mut self, pool_key: String, pool_data: PoolData) {
        self.pool_map.insert(pool_key, pool_data);
    }

    pub fn remove_pool(&mut self, pool_key: &str) -> String {
        match self.pool_map.remove(pool_key) {
            Some(pool_data) => {
                self.removed_pool_map.insert(pool_data.canister_id.clone(), pool_data);
                pool_data.canister_id
            },
            None => String::new(),
        }
    }

    pub fn restore_pool(&mut self, pool_id: &str) -> String {
        match self.removed_pool_map.remove(pool_id) {
            Some(pool_data) => {
                self.pool_map.insert(pool_data.key.clone(), pool_data);
                pool_data.canister_id
            },
            None => String::new(),
        }
    }

    pub fn delete_pool(&mut self, canister_id: &str) -> String {
        match self.removed_pool_map.remove(canister_id) {
            Some(pool_data) => canister_id.to_string(),
            None => String::new(),
        }
    }

    pub fn get_state(&self) -> State {
        State {
            pool_entries: self.pool_map.iter().map(|(k, v)| (k.clone(), v.clone())).collect(),
            removed_pool_entries: self.removed_pool_map.iter().map(|(k, v)| (k.clone(), v.clone())).collect(),
        }
    }
}

pub struct State {
    pool_entries: Vec<(String, PoolData)>,
    removed_pool_entries: Vec<(String, PoolData)>,
}
