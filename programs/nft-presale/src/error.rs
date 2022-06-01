use anchor_lang::prelude::*;

#[error_code]
pub enum ProgramErrorCode {
    #[msg("PreSale start time behinds current time!")]
    InvalidPreSaleStartTime,
    #[msg("PreSale end time behinds start time!")]
    InvalidPreSaleEndTime,
    #[msg("Name or symbol is too long! more than 20 chars!")]
    StringTooLong,
    #[msg("PreSale market not opens yet!")]
    SaleNotStarted,
    #[msg("PreSale market already ended!")]
    SaleEnded,
}
