use anchor_lang::{prelude::*, solana_program::program::invoke};
use crate::context::{MintNFT};
use crate::account::{PreSaleMarket};
use crate::error::*;
use anchor_spl::token::{mint_to, MintTo, transfer};
use mpl_token_metadata::instruction::{create_master_edition_v3, create_metadata_accounts_v2};

pub fn handle_mint_nft(
    ctx: Context<MintNFT>,
    creator_key: Pubkey,
    uri: String,
) -> Result<()> {
    let clock: Clock = Clock::get().unwrap();

    let pre_sale_market: &mut Account<PreSaleMarket> = &mut ctx.accounts.pre_sale_market;

    if clock.unix_timestamp < pre_sale_market.start_time {
        return Err(ProgramErrorCode::SaleNotStarted.into());
    }

    if clock.unix_timestamp > pre_sale_market.end_time {
        return Err(ProgramErrorCode::SaleEnded.into());
    }

    let cpi_accounts = MintTo {
        mint: ctx.accounts.mint.to_account_info(),
        to: ctx.accounts.token_account.to_account_info(),
        authority: ctx.accounts.payer.to_account_info(),
    };

    let cpi_program = ctx.accounts.token_program.to_account_info();
    msg!("CPI Program Assigned");
    let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
    msg!("CPI Context Assigned");
    mint_to(cpi_ctx, 1)?;

    let metadata_account: &UncheckedAccount = &ctx.accounts.metadata;
    let mint: &UncheckedAccount = &ctx.accounts.mint;
    let mint_authority: &Signer = &ctx.accounts.mint_authority;
    let payer: &AccountInfo = &ctx.accounts.payer;

    let creators = vec![
        mpl_token_metadata::state::Creator {
            address: creator_key,
            verified: false,
            share: 100,
        },
        mpl_token_metadata::state::Creator {
            address: ctx.accounts.mint_authority.key(),
            verified: false,
            share: 0,
        },
    ];

    let account_info = vec![
        metadata_account.to_account_info(),
        mint.to_account_info().to_account_info(),
        mint_authority.to_account_info().to_account_info(),
        payer.to_account_info().to_account_info(),
        ctx.accounts.token_metadata_program.to_account_info(),
        ctx.accounts.token_program.to_account_info(),
        ctx.accounts.system_program.to_account_info(),
        ctx.accounts.rent.to_account_info(),
    ];

    let mut nft_name: String = pre_sale_market.name.clone();
    nft_name.push_str(&" #".to_string());
    nft_name.push_str(&pre_sale_market.sold_out.to_string());

    invoke(
        &create_metadata_accounts_v2(
            mpl_token_metadata::ID,
            metadata_account.key(),
            mint.to_account_info().key(),
            mint_authority.to_account_info().key(),
            payer.to_account_info().key(),
            payer.to_account_info().key(),
            nft_name,
            pre_sale_market.symbol.clone(),
            uri,
            Some(creators),
            1,
            true,
            false,
            None,
            None,
        ),
        account_info.as_slice(),
    )?;

    let master_edition_infos = vec![
        ctx.accounts.master_edition.to_account_info(),
        ctx.accounts.mint.to_account_info(),
        ctx.accounts.mint_authority.to_account_info(),
        ctx.accounts.payer.to_account_info(),
        ctx.accounts.metadata.to_account_info(),
        ctx.accounts.token_metadata_program.to_account_info(),
        ctx.accounts.token_program.to_account_info(),
        ctx.accounts.system_program.to_account_info(),
        ctx.accounts.rent.to_account_info(),
    ];

    msg!("Master Edition Account Infos Assigned");

    invoke(
        &create_master_edition_v3(
            ctx.accounts.token_metadata_program.key(),
            ctx.accounts.master_edition.key(),
            ctx.accounts.mint.key(),
            ctx.accounts.payer.key(),
            ctx.accounts.mint_authority.key(),
            ctx.accounts.metadata.key(),
            ctx.accounts.payer.key(),
            Some(0),
        ),
        master_edition_infos.as_slice(),
    )?;

    pre_sale_market.sold_out += 1;

    let price = pre_sale_market.sale_price;

    transfer(
        ctx.accounts
            .into_transfer_tokens_to_treasury(),
        price
    )?;

    Ok(())
}