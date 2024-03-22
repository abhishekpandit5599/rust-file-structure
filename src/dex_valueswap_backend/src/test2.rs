use ic_cdk::{query, update};
use crate::{with_state, with_state2};

#[query]
fn get_data() -> i32 {
    with_state(|example| {
        let data = example.get_value();
        data
    })
}

#[query]
fn get_data2() -> u32 {
    with_state2(|example| {
        let data = example.get_value();
        data
    })
}

#[query]
fn get_data_data2() -> u32 {
    with_state2(|example| {
        let data = example.get_value();
        data
    })
}
#[update]
fn set_name2(name: String) -> String {
    with_state2(|example| {
        let data = example.set_name(name);
        data
    })
}
#[query]
fn get_name2() -> String {
    with_state2(|example| {
        let data = example.get_name();
        data
    })
}