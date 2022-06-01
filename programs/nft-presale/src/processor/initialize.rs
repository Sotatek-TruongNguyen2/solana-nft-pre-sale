use anchor_lang::prelude::*;
use crate::context::{InitializePreSaleMarket};
use crate::account::{PreSaleMarket};
use crate::error::*;
use crate::constant::{TREASURY_AUTHORITY_PDA_SEED};
use anchor_spl::token::{set_authority};
use spl_token::instruction::AuthorityType;

pub fn handle_init_pre_sale_market(
    ctx: Context<InitializePreSaleMarket>,
    name: String,
    symbol: String,
    start_time: i64,
    end_time: i64,
    total_sell: u16,
    sale_price: u64
) -> Result<()> {
    // let clock: Clock = Clock::get().unwrap();
    // let current_timestamp = clock.unix_timestamp;

    // if start_time < current_timestamp {
    //     return Err(ProgramErrorCode::InvalidPreSaleStartTime.into());
    // }

    if end_time < start_time {
        return Err(ProgramErrorCode::InvalidPreSaleEndTime.into());
    }

    if name.chars().count() > 20 || symbol.chars().count() > 20 {
        return Err(ProgramErrorCode::StringTooLong.into());
    }

    let pre_sale_market: &mut Account<PreSaleMarket> = &mut ctx.accounts.pre_sale_market;

    pre_sale_market.sale_price = sale_price;
    pre_sale_market.end_time = end_time;
    pre_sale_market.start_time = start_time;
    pre_sale_market.total_sell = total_sell;
    pre_sale_market.name = name;
    pre_sale_market.symbol = symbol;
    pre_sale_market.accept_payment_token = ctx.accounts.accept_payment_token.key();

    pre_sale_market.treasury = ctx.accounts.treasury.key();
    pre_sale_market.treasury_bump = *ctx.bumps.get("treasury").unwrap();

    let (treasury_authority, _treasury_authority_bump) = Pubkey::find_program_address(&[TREASURY_AUTHORITY_PDA_SEED], ctx.program_id);

    set_authority(
        ctx.accounts.into_set_authority_context(),
        AuthorityType::AccountOwner,
        Some(treasury_authority),
    )?;

    Ok(())
}