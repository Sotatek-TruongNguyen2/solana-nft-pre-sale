use anchor_lang::prelude::*;

#[account]
pub struct PreSaleMarket {
    pub name: String,
    pub symbol: String,
    pub sale_price: u64,
    pub start_time: i64,
    pub end_time: i64,
    pub revealed: bool,
    pub paused: bool,
    pub treasury_bump: u8,
    pub authority: Pubkey,
    pub accept_payment_token: Pubkey,
    pub treasury: Pubkey,
    pub total_sell: u16,
    pub sold_out: u16
}

const DISCRIMINATOR_LENGTH: usize = 8;
const TIMESTAMP_LENGTH: usize = 8;
const BOOL_LENGTH: usize = 1;
const PUBLIC_KEY_LENGTH: usize = 32;
const CHARACTER_LENGTH: usize = 4;
const STRING_PREFIX: usize = 4;

impl PreSaleMarket {
    pub const LEN: usize = DISCRIMINATOR_LENGTH
        + TIMESTAMP_LENGTH * 2 // start_time + end_time
        + BOOL_LENGTH * 2 //  revealed + paused
        + PUBLIC_KEY_LENGTH * 3 // Authority + Accept payment token
        + 2 * 2 // total_sell + sold_out
        + STRING_PREFIX * 2 // name + symbol prefix
        + CHARACTER_LENGTH * 20 * 2 // Name + symbol
        + 16 // Sale price
        + 1; // Bump 
}