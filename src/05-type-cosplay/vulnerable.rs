//! A vulnerable instruction that is susceptible to type cosplay.

use anchor_lang::prelude::*;

// --- Vulnerable Instruction ---
// This instruction is supposed to allow a user to withdraw their balance.
// The vulnerability is that it doesn't check the account's discriminator,
// allowing an attacker to pass a `Vault` account instead of a `User` account.

pub fn withdraw_vulnerable(ctx: Context<WithdrawVulnerable>) -> Result<()> {
    // --- VULNERABILITY ---
    // The instruction uses `UncheckedAccount` and manually deserializes the data
    // using `try_from_slice`. This function does NOT check the 8-byte discriminator.
    //
    // THE EXPLOIT:
    // An attacker can create a `Vault` account where their own pubkey is the `admin`.
    // They then call this `withdraw_vulnerable` instruction, passing the `Vault`
    // account in place of the `user_account`.
    //
    // The program will deserialize the `Vault`'s data as a `User` account.
    // - `vault.admin` (the attacker's key) is read as `user.authority`.
    // - `vault.locked_amount` is read as `user.balance`.
    //
    // The `authority` check will pass because the program thinks the attacker is the
    // authority. The program then "withdraws" the `locked_amount` from the vault,
    // effectively stealing it.

    let user_account_info = &ctx.accounts.user_account;
    let user = User::try_from_slice(&user_account_info.data.borrow())?;

    // Check if the provided authority matches the one in the account.
    // This check will pass if the attacker is the admin of the vault.
    if user.authority != ctx.accounts.authority.key() {
        return err!(ErrorCode::Unauthorized);
    }

    msg!("VULNERABLE: Withdrawing {} from account {}", user.balance, user_account_info.key());
    // In a real exploit, this would transfer `user.balance` lamports.

    Ok(())
}

#[derive(Accounts)]
pub struct WithdrawVulnerable<'info> {
    // Using `UncheckedAccount` is the source of the vulnerability.
    #[account(mut)]
    pub user_account: UncheckedAccount<'info>,
    pub authority: Signer<'info>,
}

// --- Account Type 1 ---
#[account]
pub struct User {
    pub authority: Pubkey,
    pub balance: u64,
}

// --- Account Type 2 (The "Cosplayer") ---
// This account has a similar data layout to `User` (Pubkey, u64).
#[account]
pub struct Vault {
    pub admin: Pubkey,
    pub locked_amount: u64,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Unauthorized.")]
    Unauthorized,
}
