use anchor_lang::prelude::*;
use anchor_lang::AccountDeserialize;
use metaplex_token_metadata::instruction::update_metadata_accounts;
use metaplex_token_metadata::state::{Data,Metadata};
use std::{fs::File, str::FromStr,vec::Vec};
use solana_program::{program::invoke_signed, borsh::try_from_slice_unchecked};

declare_id!("89DUgEczqCXstqCZcxvn5hquX93n2scxZa61pdk8RjWm");

#[program]
pub mod myepicproject {
  use super::*;
 
pub fn init_signer(ctx: Context<InitSigner>,  sign_bump: u8)-> ProgramResult {

let meta_account = &mut ctx.accounts.signer_account;

meta_account.signerA = "true".to_string();
meta_account.signerB = "true".to_string();
meta_account.signerC = "true".to_string();
 Ok(())
}
  
pub fn evolve(ctx: Context<Evolve>,  sign_bump: u8)-> ProgramResult {

   let new_data = Data {
        name: "Mutant Stick Figures #2".to_string(),
        symbol: "MSF".to_string(),
        uri: "dumb".to_string(),
        seller_fee_basis_points: 500,
        creators: None,
    };
    
let instxt = update_metadata_accounts(
                *ctx.accounts.token_metadata_program.key,
                *ctx.accounts.meta_data.key,
                *ctx.accounts.signer_account.to_account_info().key,
                None,
                Some(new_data),
                Some(true),
            );
            
msg!("{:?}", instxt);            

invoke_signed(
            &instxt,
            &[
                ctx.accounts.meta_data.clone(),
                ctx.accounts.signer_account.to_account_info(),
            ],
            &[&[b"signtwo".as_ref(), &[sign_bump]]],
        )?;



 Ok(())
}

#[derive(Accounts)]
#[instruction(sign_bump: u8)]
pub struct InitSigner<'info> {
#[account(init, payer = user,seeds = [b"signtwo".as_ref()], bump=sign_bump, space = 9000)]
pub signer_account: Account<'info,SignerA>,
#[account(mut)]
pub user: Signer<'info>,
pub system_program: Program <'info, System>,
}

#[derive(Accounts)]
#[instruction(sign_bump: u8)]
pub struct Evolve<'info> {
pub token_metadata_program:AccountInfo<'info>,
#[account(seeds = [b"signtwo".as_ref()],bump=sign_bump)]
pub signer_account: Account<'info,SignerA>,
#[account(mut)]
 pub meta_data:AccountInfo<'info>,
pub user: Signer<'info>,
pub system_program: Program <'info, System>,
}


#[account]
pub struct SignerA {

pub signerA: String,
pub signerB: String,
pub signerC: String,

}
