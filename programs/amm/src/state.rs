use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Config{
   min_a:Pubkey,
   min_b:Pubkey,
   fee:u16,
   bump:u8,
   lp_bump:u8
}   