# Solana Security Reference Program

# This program is a collection of instructions demonstrating common Solana
# security vulnerabilities and their solutions. It is designed for educational
# purposes. Each vulnerability pattern is contained in its own module.

use anchor_lang::prelude::*;

pub mod vulnerable {
    pub mod missing_signer_check;
    pub mod incorrect_owner_check;
    pub mod insecure_cpi;
    pub mod integer_overflow;
    pub mod type_cosplay;
}

pub mod secure {
    pub mod missing_signer_check;
    pub mod incorrect_owner_check;
    pub mod insecure_cpi;
    pub mod integer_overflow;
    pub mod type_cosplay;
}

use vulnerable::{
    missing_signer_check::{log_message_vulnerable, LogMessageVulnerable},
    incorrect_owner_check::{write_to_log_vulnerable, WriteToLogVulnerable},
    insecure_cpi::{cpi_insecure, CpiInsecure},
    integer_overflow::{increment_vulnerable, IncrementVulnerable},
    type_cosplay::{withdraw_vulnerable, WithdrawVulnerable},
};

use secure::{
    missing_signer_check::{log_message_secure, LogMessageSecure},
    incorrect_owner_check::{write_to_log_secure, WriteToLogSecure},
    insecure_cpi::{cpi_secure, CpiSecure},
    integer_overflow::{increment_secure, IncrementSecure, ErrorCode as OverflowErrorCode},
    type_cosplay::{withdraw_secure, WithdrawSecure},
};


declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod solana_security_reference {
    use super::*;

    // --- Vulnerability 1: Missing Signer Check ---
    pub fn vulnerable_signer_check(ctx: Context<LogMessageVulnerable>) -> Result<()> {
        log_message_vulnerable(ctx)
    }
    pub fn secure_signer_check(ctx: Context<LogMessageSecure>) -> Result<()> {
        log_message_secure(ctx)
    }

    // --- Vulnerability 2: Incorrect Owner Check ---
    pub fn vulnerable_owner_check(ctx: Context<WriteToLogVulnerable>, data: u64) -> Result<()> {
        write_to_log_vulnerable(ctx, data)
    }
    pub fn secure_owner_check(ctx: Context<WriteToLogSecure>, data: u64) -> Result<()> {
        write_to_log_secure(ctx, data)
    }

    // --- Vulnerability 3: Insecure CPI ---
    pub fn vulnerable_insecure_cpi(ctx: Context<CpiInsecure>, message: String) -> Result<()> {
        cpi_insecure(ctx, message)
    }
    pub fn secure_insecure_cpi(ctx: Context<CpiSecure>, message: String) -> Result<()> {
        cpi_secure(ctx, message)
    }

    // --- Vulnerability 4: Integer Overflow ---
    pub fn vulnerable_integer_overflow(ctx: Context<IncrementVulnerable>) -> Result<()> {
        increment_vulnerable(ctx)
    }
    pub fn secure_integer_overflow(ctx: Context<IncrementSecure>) -> Result<()> {
        increment_secure(ctx)
    }

    // --- Vulnerability 5: Type Cosplay ---
    pub fn vulnerable_type_cosplay(ctx: Context<WithdrawVulnerable>) -> Result<()> {
        withdraw_vulnerable(ctx)
    }
    pub fn secure_type_cosplay(ctx: Context<WithdrawSecure>) -> Result<()> {
        withdraw_secure(ctx)
    }

    pub fn initialize_counter(ctx: Context<InitializeCounter>, initial_count: u64) -> Result<()> {
        let counter = &mut ctx.accounts.counter;
        counter.count = initial_count;
        Ok(())
    }

    pub fn initialize_user(ctx: Context<InitializeUser>, authority: Pubkey, balance: u64) -> Result<()> {
        let user_account = &mut ctx.accounts.user_account;
        user_account.authority = authority;
        user_account.balance = balance;
        Ok(())
    }

    pub fn initialize_vault(ctx: Context<InitializeVault>, admin: Pubkey, locked_amount: u64) -> Result<()> {
        let vault_account = &mut ctx.accounts.vault_account;
        vault_account.admin = admin;
        vault_account.locked_amount = locked_amount;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeCounter<'info> {
    #[account(init, payer = signer, space = 8 + 8)] // Discriminator + u64
    pub counter: Account<'info, Counter>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct InitializeUser<'info> {
    #[account(init, payer = signer, space = 8 + 32 + 8)] // Discriminator + Pubkey + u64
    pub user_account: Account<'info, User>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct InitializeVault<'info> {
    #[account(init, payer = signer, space = 8 + 32 + 8)] // Discriminator + Pubkey + u64
    pub vault_account: Account<'info, Vault>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}
