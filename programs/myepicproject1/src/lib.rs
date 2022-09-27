use anchor_lang::prelude::*;
// This is kinda like an import statement, allowing us to import anchors tools

declare_id!("6YJvRpLhVBZfmLxHy5m3P2Nv61A25H4cGmVnE2BxMsdF");
//this is a macro, dexlaring what the program's account id is and should be used at the root of all anchor programs
#[program]
//this is an attribute macro, it attaches code to the modules and lets anchor know this is our programs instruction handlers
//this allows you to call your solana program from the dapp via fetch request

pub mod myepicproject1 {
    //pub mod is a module (collection of functions and vars, pub makes it public)
    use super::*;
    //brings all the things imported from the anchor prelude into the current modules context
    //pub fn - the function sso is a handler with context as a param and outputs result

    pub fn start_stuff_off(ctx: Context <StartStuffOff>) -> Result <()> {
        let base_account = &mut ctx.accounts.base_account;
        //&mut allows to get a mutable ref to baseacc, allowing you to make changes
        base_account.total_gifs = 0;
        //just initializing the totalgifs
        Ok(())
    }
    

    pub fn add_gif(ctx: Context<AddGif>) -> Result <()> {
        let base_account = &mut ctx.accounts.base_account;
        base_account.total_gifs += 1;
        Ok(())

    }
}

 
#[derive(Accounts)]
pub struct StartStuffOff<'info> {
  #[account(init, payer = user, space = 9000)]
    //change 9000 to 10000 if error, this is the bytes of space for your account

    //initializing baseaccounts
  pub base_account: Account<'info, BaseAccount>,

  #[account(mut)]
  pub user: Signer <'info>,
    //this is data passed into program that orives to the program that the user calling this actually owns their wallet

  pub system_program: Program <'info, System>,
    //this is a ref to the system program, a program that creators of solana depoloyed that other programs can talk to
}
//this is a derive macro that has a param of accounts, defines account constraints 
#[derive(Accounts)]
pub struct AddGif<'info> {
  #[account(mut)]
  pub base_account: Account<'info, BaseAccount>,
}

#[account]
pub struct BaseAccount {
    pub total_gifs: u64,
}