use anchor_lang::prelude::*;

declare_id!("Fdswagc7gUcFvqmCgqe5uMvNE3A5Skoyr17gZQpRmQtP");

#[program]
pub mod heritage_vault {
    use super::*;

    // 1. INITIALIZATION: Sets up the vault
    pub fn initialize(ctx: Context<Initialize>, unlock_date: i64) -> Result<()> {
        let vault = &mut ctx.accounts.vault;
        vault.owner = *ctx.accounts.owner.key;
        vault.unlock_date = unlock_date;
        vault.balance = 0;
        
        msg!("Vault initialized. Funds locked until: {}", unlock_date);
        Ok(())
    }

    // 2. DEPOSIT: Anyone can add funds, but only the owner can withdraw
    pub fn deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
        let transfer_instruction = anchor_lang::system_program::Transfer {
            from: ctx.accounts.owner.to_account_info(),
            to: ctx.accounts.vault.to_account_info(),
        };
        
        let cpi_ctx = CpiContext::new(
            ctx.accounts.system_program.to_account_info(),
            transfer_instruction,
        );

        anchor_lang::system_program::transfer(cpi_ctx, amount)?;
        ctx.accounts.vault.balance += amount;
        Ok(())
    }

    // 3. WITHDRAW: The core security logic
    pub fn withdraw(ctx: Context<Withdraw>) -> Result<()> {
        let vault = &mut ctx.accounts.vault;
        let clock = Clock::get()?; // Get the current blockchain time

        // SECURITY CHECK: Is the signer the owner?
        require_keys_eq!(vault.owner, ctx.accounts.owner.key(), VaultError::Unauthorized);

        // SECURITY CHECK: Has the time passed?
        require!(clock.unix_timestamp >= vault.unlock_date, VaultError::TooEarly);

        // Transfer funds back to the owner
        let amount = vault.balance;
        **vault.to_account_info().try_borrow_mut_lamports()? -= amount;
        **ctx.accounts.owner.to_account_info().try_borrow_mut_lamports()? += amount;

        vault.balance = 0;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = owner, space = 8 + 32 + 8 + 8)]
    pub vault: Account<'info, Vault>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Deposit<'info> {
    #[account(mut)]
    pub vault: Account<'info, Vault>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(mut)]
    pub vault: Account<'info, Vault>,
    #[account(mut)]
    pub owner: Signer<'info>,
}

#[account]
pub struct Vault {
    pub owner: Pubkey,
    pub unlock_date: i64,
    pub balance: u64,
}

#[error_code]
pub enum VaultError {
    #[msg("Unauthorized: You are not the owner.")]
    Unauthorized,
    #[msg("Patience: The unlock date has not arrived yet.")]
    TooEarly,
}
