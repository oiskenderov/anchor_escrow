use anchor_lang::prelude::*;


#[account]
#[derive(InitSpace)]

pub struct Escrow{
    pub seed: u64,
    pub maker: Pubkey,
    pub mint_a: Pubkey,
    pub mint_b: Pubkey,
    pub recieve: u64,
    pub bump: u8,
}

impl Escrow {
    pub const LEN: usize = 8 + 8 + 32 + 32 + 8 + 1;
}