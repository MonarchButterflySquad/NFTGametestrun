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
pub meta_data:AccountInfo<'info>,
#[account(seeds = [b"signtwo".as_ref()],bump=sign_bump)]
pub signer_account: Account<'info,SignerA>,
#[account(mut)]
pub user: Signer<'info>,
pub system_program: Program <'info, System>,
}


#[account]
pub struct Meta2 {

pub level: String,
pub hp: String,
pub attack: String,
pub defense: String,
pub accuracy: String,
pub speed: String,
pub specattack: String,
pub specdefense: String,
pub item1: String,
pub item2: String,
pub item3: String,
pub item4: String,
pub item5: String,
pub item6: String,
pub item7: String,
pub item8: String,
pub item9: String,
pub item10: String,
pub world: String,
pub mutation: String,
pub image1: String,
pub image2: String,
pub image3: String,
pub dummy: String,

}

#[account]
pub struct SignerA {

pub signerA: String,
pub signerB: String,
pub signerC: String,

}
