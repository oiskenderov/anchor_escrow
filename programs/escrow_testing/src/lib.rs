use anchor_lang::prelude::*;

pub mod state;

pub use state::*;


declare_id!("Ag6HWC592Cw1uwFto4kTdFsYiaixrM1tUjtVja7oxMgY");

#[program]
pub mod escrow_testing {
    use crate::instruction::Make;

    use super::*;

    // pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
    //     msg!("Greetings from: {:?}", ctx.program_id);
    //     Ok(())

    pub fn make(ctx: Context<Make>, seed: u64, deposit: u64, recieve: u64) -> Result<()> {
        // ctx.accounts.deposit(deposit)?;
        // ctx.accounts.init_escrow(seed, recieve, &ctx.bumps)
        Ok(())
    }

        pub fn refund(ctx: Context<Refund>) -> Result<()> {
            ctx.accounts.refund_and_close_vault()
        }


    }

// #[derive(Accounts)]
// pub struct Initialize {}
