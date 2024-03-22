use ic_cdk::query;
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