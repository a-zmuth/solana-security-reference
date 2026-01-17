use anchor_lang::prelude::*;

// --- Vulnerable Instruction ---
// This instruction is supposed to allow a user to withdraw their balance.
// The vulnerability is that it doesn't check the account's discriminator.

pub fn withdraw_vulnerable(ctx: Context<WithdrawVulnerable>) -> Result<()> {
    // --- VULNERABILITY ---
    // The instruction uses `UncheckedAccount` and manually deserializes the data
    // using `try_from_slice`, which does NOT check the 8-byte discriminator.
    //
    // THE EXPLOIT:
    // An attacker can pass a `Vault` account instead of a `User` account.
    // The program will misinterpret the `Vault` data as `User` data, leading
    // to an authorization bypass and theft of funds from the vault.

    let user_account_info = &ctx.accounts.user_account;
    let user = User::try_from_slice(&user_account_info.data.borrow()[8..])?; // Skip discriminator

    if user.authority != ctx.accounts.authority.key() {
        return err!(ErrorCode::Unauthorized);
    }

    msg!("VULNERABLE: Withdrawing {} from account {}", user.balance, user_account_info.key());
    Ok(())
}

#[derive(Accounts)]
pub struct WithdrawVulnerable<'info> {
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
