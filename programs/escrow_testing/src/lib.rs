use anchor_lang::prelude::*;

pub mod instructions;
pub mod state;


use instructions::*;
use instructions::Make;
use instructions::make::Make;
use instruction::take::Take;
use instructions::refund::Refund;



pub use state::*;


declare_id!("Ag6HWC592Cw1uwFto4kTdFsYiaixrM1tUjtVja7oxMgY");

#[program]
pub mod escrow_testing {
    use anchor_lang::Bumps;

    use crate::Instructions::Make;

    use super::*;

    pub fn make(
        ctx: Context<Make>,
        seed: u64,
        deposit: u64,
        receive: u64,
    ) -> Result<()> {
        make::handler(ctx, seed, deposit, receive)
    }

    pub fn take(ctx: Context<Take>) -> Result<()> {
        take::handler(ctx)
    }

    pub fn refund(ctx: Context<Refund>) -> Result<()> {
        refund::handler(ctx)
    }

    
    // pub fn make(ctx: Context<Make>, seed: u64, deposit: u64, receive: u64, bumps: u8) -> Result<()> {
    //     ctx.accounts.deposit(deposit)?;
    //     ctx.accounts.init_escrow(seed, receive, &ctx.MakeBumps());
    //     instructions::Make::handler(ctx, seed, deposit, recieve)
    // }

    // pub fn refund(ctx: Context<Refund>) -> Result<()> {
    //     ctx.accounts.refund_and_close_vault();
    //     instructions::refund::handler(ctx, seed)
    // }

    // pub fn take(ctx: Context<Take>) -> Result<()> {
    //     ctx.accounts.deposit()?;
    //     ctx.accounts.withdraw_and_close_vault()
    // }
}

// #[derive(Accounts)]
// pub struct Initialize {}
