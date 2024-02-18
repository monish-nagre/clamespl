use anchor_lang::prelude::*;
use anchor_spl::{token::{Token, TokenAccount, Mint}, associated_token::AssociatedToken};

declare_id!("EqC6pAzi3nkGvx7jwXQ6mhLjHR8YGUwWvAqG8fFkFy5y");

#[program]
pub mod clamespl {
    use anchor_lang::system_program;
    use anchor_spl::{token::{initialize_mint, InitializeMint, mint_to, MintTo, transfer, Transfer}, associated_token};

    use super::*;

    pub fn start_stuff_off(ctx: Context<StartStuffOff>) -> Result <()> {
        let base_account = &mut ctx.accounts.base_account;
        base_account.total_add = 0;
        Ok(())
      }
      
  
      pub fn add_address(ctx: Context<AddAddresses>, spl_token_amt: u64, user_address: Pubkey) -> Result <()> {
        let base_account = &mut ctx.accounts.base_account;
        
    
        let item = ItemStruct {
          spl_token_amt,
          user_address,
        };
        
        base_account.add_list.push(item);
        base_account.total_add += 1;
        Ok(())
      }

      

    pub fn check(ctx: Context<Check>) -> Result<Vec<u64>> {
        let base_account = &ctx.accounts.base_account;
        let caller_pubkey = ctx.accounts.user.key();
    
        // Check if the caller's public key matches any of the stored user addresses
        let is_authorized = base_account
            .add_list
            .iter()
            .any(|item| item.user_address == caller_pubkey);
    
        if !is_authorized {
            return Err(ProgramError::Custom(ProgramErrors::NotAuthorized as u32).into());
        }
    
        let spl_token_amts_for_user: Vec<u64> = base_account
            .add_list
            .iter()
            .filter(|item| item.user_address == caller_pubkey)
            .map(|item| item.spl_token_amt)
            .collect();
    
        Ok(spl_token_amts_for_user)
    }

    pub fn create_token(ctx: Context<CreateToken>,decimals:u8,amount:u64) -> Result<()> {

        system_program::create_account(
            CpiContext::new(
                ctx.accounts.system_program.to_account_info(), 
                system_program::CreateAccount { from: ctx.accounts.signer.to_account_info(), to: ctx.accounts.mint_token.to_account_info() }
            ), 
            10_000_000, 
            82, 
            ctx.accounts.token_program.key
        )?;

        initialize_mint(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                InitializeMint{mint:ctx.accounts.mint_token.to_account_info(),rent:ctx.accounts.rent.to_account_info()}
            ), 
            decimals, 
            ctx.accounts.signer.key, 
            Some(ctx.accounts.signer.key)
        )?;


        associated_token::create(
            CpiContext::new(
                ctx.accounts.associate_token_program.to_account_info(), 
                associated_token::Create { 
                    payer: ctx.accounts.signer.to_account_info(), 
                    associated_token: ctx.accounts.token_account.to_account_info(), 
                    authority: ctx.accounts.signer.to_account_info(), 
                    mint: ctx.accounts.mint_token.to_account_info(), 
                    system_program: ctx.accounts.system_program.to_account_info(), 
                    token_program: ctx.accounts.token_program.to_account_info() 
                }
            )
        )?;

        mint_to(
            CpiContext::new(
                ctx.accounts.token_account.to_account_info(), 
                MintTo{authority:ctx.accounts.signer.to_account_info(),mint:ctx.accounts.mint_token.to_account_info(),to:ctx.accounts.token_account.to_account_info()}
            ), 
            amount
        )?;

        Ok(())
    }

    pub fn transer_token(ctx: Context<TransferToken>,amount:u64)->Result<()>{

        msg!("Started {:} tokens transfer from account {:} to {:}",amount,ctx.accounts.from_account.key(),ctx.accounts.to_account.key());

        transfer(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(), 
                Transfer{authority:ctx.accounts.signer.to_account_info(),from:ctx.accounts.from_account.to_account_info(),to:ctx.accounts.to_account.to_account_info()}
            ), 
            amount
        )?;

        Ok(())
    }
    
}

#[derive(Accounts)]
pub struct StartStuffOff<'info> {
  #[account(init, payer = user, space = 9000)]
  pub base_account: Account<'info, BaseAccount>,
  #[account(mut)]
  pub user: Signer<'info>,
  pub system_program: Program <'info, System>,
}


#[derive(Accounts)]
pub struct AddAddresses<'info> {
  #[account(mut)]
  pub base_account: Account<'info, BaseAccount>,
  #[account(mut)]
  pub user: Signer<'info>,
}

#[derive(Accounts)]
pub struct Check<'info> {
    #[account(mut)]
    pub base_account: Account<'info, BaseAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
}

// Create a custom struct for us to work with.
#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize)]
pub struct ItemStruct {
    pub spl_token_amt: u64,
    pub user_address: Pubkey,
}

#[account]
pub struct BaseAccount {
    pub total_add: u64,
	// Attach a Vector of type ItemStruct to the account.
    pub add_list: Vec<ItemStruct>,
}

#[derive(Accounts)]
pub struct CreateToken<'info> {
    #[account(mut)]
    pub mint_token:Signer<'info>,
    #[account(mut)]
    pub signer:Signer<'info>,
    ///CHECK:
    #[account(mut)]
    pub token_account:AccountInfo<'info>,
    pub system_program:Program<'info,System>,
    pub token_program:Program<'info,Token>,
    pub associate_token_program:Program<'info,AssociatedToken>,
    pub rent:Sysvar<'info,Rent>
}

#[derive(Accounts)]
pub struct TransferToken<'info>{    
    #[account(mut)]
    pub mint_token:Account<'info,Mint>,
    #[account(mut)]
    pub from_account:Account<'info,TokenAccount>,
    #[account(mut)]
    pub to_account:Account<'info,TokenAccount>,
    #[account(mut)]
    pub signer:Signer<'info>,
    pub system_program:Program<'info,System>,
    pub token_program:Program<'info,Token>,
    pub associate_token_program:Program<'info,AssociatedToken>,
}



#[error_code]
pub enum ProgramErrors {
    #[msg("PDA account not matched")]
    PdaNotMatched,
    #[msg("Not authorized to perform this action")]
    NotAuthorized,
    #[msg("Address is already in the whitelist")]
    AlreadyInWhitelist,
}