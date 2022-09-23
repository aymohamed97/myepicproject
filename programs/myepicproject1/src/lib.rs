use anchor_lang::prelude::*;

declare_id!("6YJvRpLhVBZfmLxHy5m3P2Nv61A25H4cGmVnE2BxMsdF");

#[program]
pub mod myepicproject1 {
    use super::*;
    pub fn start_stuff_off(ctx: Context <StartStuffOff>) -> Result <()
> {
    Ok(())
}}

#[derive(Accounts)]
pub struct StartStuffOff {}

