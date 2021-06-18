use anchor_lang::prelude::*;

#[program]
pub mod yta_token {
    use super::*;
    pub fn initialize(ctx: Context<Initialize>) -> ProgramResult {
        let yta_token = &mut ctx.accounts.yta_account;
        yta_token.amount=0;
        yta_token.authority = *ctx.accounts.authority.key;
        Ok(())
    }
    pub fn increase(ctx: Context<Increase>,amount:u64)->ProgramResult{
        let yta_token = &mut  ctx.accounts.yta_account;
        yta_token.amount+=amount;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init)]
    pub yta_account:ProgramAccount<'info,YTAToken>,
    #[account(signer)]
    pub authority:AccountInfo<'info>,
    pub rent:Sysvar<'info,Rent>
}

#[derive(Accounts)]
pub struct Increase<'info>{
    #[account(mut, has_one = authority)]
    pub yta_account:ProgramAccount<'info,YTAToken>,
    #[account(signer)]
    pub authority:AccountInfo<'info>,
}
#[account]
pub struct YTAToken{
    pub amount:u64,
    pub authority:Pubkey
}