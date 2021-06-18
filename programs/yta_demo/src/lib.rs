use anchor_lang::prelude::*;
use yta_token::{YTAToken,Increase};

#[program]
pub mod yta_demo {
    use super::*;
    pub fn initialize(ctx: Context<Initialize>,total:u64,authority:Pubkey) -> ProgramResult {
        let yta_config = &mut ctx.accounts.yta_config;
        yta_config.authority = authority;
        yta_config.total = total;
        Ok(())
    }
    pub fn Reward(ctx: Context<Reward>,amount:u64)->ProgramResult{
        let yta_config = &mut ctx.accounts.yta_config;
        if yta_config.total - amount >=0 {
            yta_config.total-=amount;
            
            let cpi_program = ctx.accounts.yta_config.clone();
            let cpi_accounts = Increase{
                yta_account:ctx.accounts.receiver.clone().into(),
                authority:ctx.accounts.authority.clone(),
            };

            let cpi_ctx = CpiContext::new(cpi_program,cpi_accounts);
           yta_token::cpi::increase(cpi_ctx,amount);
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init)]
    pub yta_config:ProgramAccount<'info,YTAConfig>,
    pub rent:Sysvar<'info,Rent>
}
#[derive(Accounts)]
pub struct Reward<'info>{
    #[account(mut)]
    pub yta_config:ProgramAccount<'info,YTAConfig>,
    pub receiver:CpiAccount<'info,YTAToken>,
    pub authority:AccountInfo<'info>,
}

#[account]
pub struct YTAConfig{
    pub total:u64,
    pub authority:Pubkey,
}