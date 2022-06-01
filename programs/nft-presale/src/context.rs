use anchor_lang::prelude::*;
use anchor_spl::token::{Token, Mint, TokenAccount, SetAuthority, Transfer};
use crate::account::{PreSaleMarket};

#[derive(Accounts)]
pub struct InitializePreSaleMarket<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    pub accept_payment_token: Account<'info, Mint>,
    #[account(
        init,
        payer = authority,
        space = PreSaleMarket::LEN
    )]
    pub pre_sale_market: Account<'info, PreSaleMarket>,
    #[account(
        init,
        payer = authority,
        seeds = [
            b"treasury",
            pre_sale_market.key().as_ref()
        ],
        bump,
        token::mint = accept_payment_token,
        token::authority = authority,
    )]
    pub treasury: Account<'info, TokenAccount>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,
}


#[derive(Accounts)]
pub struct MintNFT<'info> {
    #[account(has_one = authority, has_one = treasury)]
    pub pre_sale_market: Account<'info, PreSaleMarket>,
    #[account(
        constraint = buyer_token_account.mint == pre_sale_market.accept_payment_token,
        constraint = buyer_token_account.owner == mint_authority.key()
    )]
    pub buyer_token_account: Account<'info, TokenAccount>,
    pub treasury: Account<'info, TokenAccount>,
    // Market authority
    pub authority: Signer<'info>,

    // Mint authority. Pay attention to this account, it's different than market_authority
    pub mint_authority: Signer<'info>,

    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    pub mint: UncheckedAccount<'info>,

    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    pub metadata: UncheckedAccount<'info>,

    /// CHECK: account checked in CPI
    #[account(mut)]
    pub master_edition: UncheckedAccount<'info>,

    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    pub token_account: UncheckedAccount<'info>,

    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(address = mpl_token_metadata::id())]
    pub token_metadata_program: UncheckedAccount<'info>,

    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    pub payer: AccountInfo<'info>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub rent: AccountInfo<'info>,
}

impl<'info> InitializePreSaleMarket<'info> {
    pub fn into_set_authority_context(&self) -> CpiContext<'_, '_, '_, 'info, SetAuthority<'info>> {
        let cpi_accounts = SetAuthority {
            account_or_mint: self.treasury.to_account_info().clone(),
            current_authority: self.authority.to_account_info().clone(),
        };
        CpiContext::new(self.token_program.to_account_info().clone(), cpi_accounts)
    }
}

impl<'info> MintNFT<'info> {
    pub fn into_transfer_tokens_to_treasury(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let cpi_accounts = Transfer {
            from: self.buyer_token_account.to_account_info(),
            to: self.treasury.to_account_info(),
            authority: self.payer.to_account_info()
        };
        CpiContext::new(self.token_program.to_account_info().clone(), cpi_accounts)
    }
}

