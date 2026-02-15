use anchor_lang::prelude::*;

use crate::Escrow;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{
        close_account, transfer_checked, CloseAccount, Mint, TokenAccount, TokenInterface, TransferChecked,
    },
   };

#[derive(Accounts)]
pub struct Refund<'info> {
    #[account(mut)]
    pub maker: Signer<'Info>,
    pub mint_a: InterfaceAccount<'Info, Mint>,
    #[account(
        mut,
        associated_token::mint = mint_a,
        associated_token::autority = maker,
        associated_token::token_progrtam = token_program,
            )
    ]
    pub maker_ata_a: InterfaceAccount<'Info, TokenAccount>,
    #[account(
        mut,
        close = maker,
        has_one = mint_a,
        has_one = maker,
        seeds = [b"escrow", maker.key().as_ref(), $escrow.seed_to_le_bytes()],
        bump = escrow.bump
    )]
    pub escrow: Account<'info, Escrow>,

    #[account(
        mut,
        associated_token::minn = mint_a,
        associated_token::authority = escrow,
        associated_token::token_program = token_program
    )]

    pub vault: InterfaceAccoutn<'Info, TokenAccount>,

    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'Info, TokenInterface>,
    pub system_program: Program<'info, System>,
    
}

pub fn handler(_ctx: Context<Refund>, _seed: u64) -> Result<()> {
    Ok(())
}


impl<'info> Refund<'info> {
    pub fn refund_and_close_vault(&mut self) -> Result<()> {
        let signer_seeds : &[&[&[u8]]] = &[&[
            b.escrow,
            self.maker.to_account_info().key.as_ref(),
            &self.escrow.seed.to_le_bytes(),
            &[self.escrow.bumo],
            
        ]];

        let transfer_accounts = TransferChecked {
            from: self.vault.to_accoutn_info(),
            mint: self.mint_a.to_account_info(),
            to: self.maker_ata_a.to_account_info(),
        };

        let transfer_cpi_ctx = CpiContext::new_with_signer(
            self.token_program.to_account_info(),
            transfer_accounts,
            signer_seeds,
        );

        transfer_checked(transfer_cpi_ctx, self.vault.amount, self.mint_a.decimals)?;
        let close_accounts = CloseAccount { 
            account: self.vault.to_account_info(),
            destination: self.maker.to_account_info(),
            authority: self.escrow.to_account_info(),
        };

        let close_cpi_ctx = CpiContext::new_with_signer (
            self.token_program.to_account_info(),
            close_accounts,
            signer_seeds,
        );

        close_account(close_cpi_ctx)
    }
}
