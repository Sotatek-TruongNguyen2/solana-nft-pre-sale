use anchor_lang::prelude::*;

pub mod account;
pub mod constant;
pub mod context;
pub mod error;
pub mod processor;

pub use context::*;
pub use processor::*;

declare_id!("EBo6LU66bXznJPUdySb4Rh9uZMNcwJF1gWkwuyWRMxcP");

#[program]
pub mod nft_presale {
    use super::*;

    pub fn mint_nft(ctx: Context<MintNFT>, creator_key: Pubkey, uri: String) -> Result<()> {
        handle_mint_nft(ctx, creator_key, uri)
    }

    pub fn initialize_pre_sale_market(
        ctx: Context<InitializePreSaleMarket>,
        name: String,
        symbol: String,
        start_time: i64,
        end_time: i64,
        total_sell: u16,
        sale_price: u64,
    ) -> Result<()> {
        handle_init_pre_sale_market(
            ctx, name, symbol, start_time, end_time, total_sell, sale_price,
        )
    }
}
