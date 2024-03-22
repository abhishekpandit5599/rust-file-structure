use std::collections::HashMap;

pub enum Value {
    Nat(u64),
    Int(i64),
    Blob(Vec<u8>),
    Text(String),
}

pub enum Error {
    CommonError,
    InternalError(String),
    UnsupportedToken(String),
    InsufficientFunds,
}

pub struct Token {
    pub address: String,
    pub standard: String,
}

pub type AccountIdentifier = String;

pub struct Account {
    pub principal: String,
    pub subaccount: Option<Vec<u8>>,
}

pub struct User {
    pub address: AccountIdentifier,
    pub account: Account,
}

pub struct PoolMetadata {
    pub key: String,
    pub token0: Token,
    pub token1: Token,
    pub fee: u64,
    pub tick: i64,
    pub liquidity: u64,
    pub sqrt_price_x96: u64,
    pub max_liquidity_per_tick: u64,
    pub next_position_id: u64,
}

pub struct PoolData {
    pub key: String,
    pub token0: Token,
    pub token1: Token,
    pub fee: u64,
    pub tick_spacing: i64,
    pub canister_id: String,
}

pub struct PositionInfo {
    pub liquidity: u64,
    pub fee_growth_inside0_last_x128: u64,
    pub fee_growth_inside1_last_x128: u64,
    pub tokens_owed0: u64,
    pub tokens_owed1: u64,
}

pub struct PositionInfoWithId {
    pub id: String,
    pub liquidity: u64,
    pub fee_growth_inside0_last_x128: u64,
    pub fee_growth_inside1_last_x128: u64,
    pub tokens_owed0: u64,
    pub tokens_owed1: u64,
}

pub struct UserPositionInfo {
    pub tick_lower: i64,
    pub tick_upper: i64,
    pub liquidity: u64,
    pub fee_growth_inside0_last_x128: u64,
    pub fee_growth_inside1_last_x128: u64,
    pub tokens_owed0: u64,
    pub tokens_owed1: u64,
}

// Define other types similarly...

pub struct SwapPoolActor {
    // Define actor fields and methods...
}

pub struct SwapFactoryActor {
    // Define actor fields and methods...
}

pub struct SwapFeeReceiverActor {
    // Define actor fields and methods...
}

