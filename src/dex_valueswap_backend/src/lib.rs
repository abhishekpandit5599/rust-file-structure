use std::{cell::RefCell, collections::BTreeMap, sync::{Arc, Mutex, RwLock}};

use ic_cdk::{export_candid, post_upgrade, pre_upgrade};
mod test2;
mod memory;
mod test;
mod upgrade;
use memory::Memory;
use test::Example;
mod stable_test;
use stable_test::Example2;
use ic_cdk_macros::{query, update};
use ic_stable_structures::StableBTreeMap;

thread_local! {
    static EXAMPLE_INSTANCE: Mutex<Example> = Mutex::new(Example { number: 0 });
    static EXAMPLE_INSTANCE2: RefCell<Example2> = RefCell::new(Example2::new());
    // static EXAMPLE_INSTANCE2: Mutex<Example2> = Mutex::new(Example2::new());
}

pub fn with_state<R>(f: impl FnOnce(&mut Example) -> R) -> R {
    EXAMPLE_INSTANCE.with(|cell| f(&mut cell.lock().unwrap()))
}

pub fn with_state2<R>(f: impl FnOnce(&mut Example2) -> R) -> R {
    EXAMPLE_INSTANCE2.with(|cell| f(&mut cell.borrow_mut()))
}
pub fn with_state2_without<R>(f: impl FnOnce(&Example2) -> R) -> R {
    EXAMPLE_INSTANCE2.with(|cell| f(&mut cell.borrow_mut()))
}

pub fn init_file_contents() -> StableBTreeMap<u32, u32, Memory> {
    StableBTreeMap::init(crate::memory::get_file_contents_memory())
}

#[query]
fn greet() -> String {
    let name = Example::whoami();
    format!("Hello, {}!", name)
}

#[update]
fn init() -> bool {
    EXAMPLE_INSTANCE.with(|states: &Mutex<Example>| {
        let mut example = states.lock().unwrap();
        let data = example.init();
        true
    })
}

#[update]
fn increment_number() -> i32 {
    EXAMPLE_INSTANCE.with(|states: &Mutex<Example>| {
        let mut example = states.lock().unwrap();
        let data = example.increment();
        data
    })
}

#[update]
fn decrement_number() -> i32 {
    EXAMPLE_INSTANCE.with(|states: &Mutex<Example>| {
        let mut example = states.lock().unwrap();
        let data = example.decrement();
        data
    })
}

#[update]
fn set_number(num: i32) -> i32 {
    EXAMPLE_INSTANCE.with(|states: &Mutex<Example>| {
        let mut example = states.lock().unwrap();
        let data = example.set_value(num);
        data
    })
}

#[query]
fn get_number() -> i32 {
    EXAMPLE_INSTANCE.with(|states: &Mutex<Example>| {
        let mut example = states.lock().unwrap();
        let data = example.get_value();
        data
    })
}

#[update]
fn increment_number2() -> u32 {
    EXAMPLE_INSTANCE2.with(|states| {
        let mut example = states.borrow_mut();
        let data = example.increment();
        data
    })
}

#[update]
fn decrement_number2() -> u32 {
    EXAMPLE_INSTANCE2.with(|states| {
        let mut example = states.borrow_mut();
        let data = example.decrement();
        data
    })
}

#[query]
fn get_number2() -> u32 {
    EXAMPLE_INSTANCE2.with(|states| {
        let mut example = states.borrow_mut();
        let data = example.get_value();
        data
    })
}

#[update]
fn set_number2(num: u32) -> u32 {
    EXAMPLE_INSTANCE2.with(|states| {
        let mut example = states.borrow_mut();
        let data = example.set_value(num);
        data
    })
}

#[pre_upgrade]
fn pre_upgrade() {
    upgrade::pre_upgrade();
}

#[post_upgrade]
fn post_upgrade() {
    upgrade::post_upgrade();
}

export_candid!();
