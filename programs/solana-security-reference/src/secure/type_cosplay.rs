use anchor_lang::prelude::*;
use crate::vulnerable::type_cosplay::{User, Vault};

// --- Secure Instruction ---
// This instruction safely withdraws a user's balance by using Anchor's
// `Account<T>` type, which automatically validates the discriminator.

pub fn withdraw_secure(ctx: Context<WithdrawSecure>) -> Result<()> {
    // --- SECURE ---
    // By using `Account<'info, User>`, Anchor automatically checks the
    // account's discriminator.
    //
    // THE FIX:
    // If an attacker tries to pass a `Vault` account, the transaction will be
    // rejected because the `Vault` discriminator does not match the `User`
    // discriminator.

    let user = &ctx.accounts.user_account;
    msg!("SECURE: Withdrawing {} from account {}", user.balance, user.key());
    Ok(())
}

#[derive(Accounts)]
pub struct WithdrawSecure<'info> {
    // --- SECURE ---
    // The `Account` type enforces the specific type `User`.
    #[account(mut, has_one = authority)]
    pub user_account: Account<'info, User>,
    pub authority: Signer<'info>,
}
