use std::borrow::BorrowMut;

use ic_stable_structures::memory_manager::{MemoryId, VirtualMemory};
use ic_stable_structures::DefaultMemoryImpl;
use ic_stable_structures::StableBTreeMap;

use crate::init_file_contents;

const UPGRADES: MemoryId = MemoryId::new(0);

const FILE_CONTENTS: MemoryId = MemoryId::new(1);

pub type Memory = VirtualMemory<DefaultMemoryImpl>;
pub struct Example2 {
    pub number: StableBTreeMap<u32, u32, Memory>,
}

impl Example2 {
    pub fn new() -> Self {
        Self {
            number: init_file_contents(),
        }
    }

    pub fn init(&mut self) -> bool {
        self.number.insert(1, 10);
        return true;
    }
    pub fn whoami() -> String {
        return "I am Abhishek Sharma.".to_string();
    }
    pub fn increment(&mut self) -> u32 {
        let updated_number = self.number.get(&1);
        match updated_number {
            Some(num) => {
                self.number.insert(1, num + 1);
                return self.number.get(&1).unwrap();
            }
            None => {
                return 0;
            }
        }
    }
    pub fn decrement(&mut self) -> u32 {
        let updated_number = self.number.get(&1);
        match updated_number {
            Some(num) => {
                self.number.insert(1, num - 1);
                return self.number.get(&1).unwrap();
            }
            None => {
                return 0;
            }
        }
    }
    pub fn get_value(&mut self) -> u32 {
        let updated_number = self.number.get(&1);
        match updated_number {
            Some(num) => {
                return num;
            }
            None => {
                return 0;
            }
        }
    }
    pub fn set_value(&mut self, number: u32) -> u32 {
        self.number.insert(1, number);
        return self.number.get(&1).unwrap();
    }
}
