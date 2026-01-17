//! A secure instruction that prevents type cosplay.

use anchor_lang::prelude::*;

// --- Secure Instruction ---
// This instruction safely withdraws a user's balance by using Anchor's
// `Account<T>` type, which automatically validates the discriminator.

pub fn withdraw_secure(ctx: Context<WithdrawSecure>) -> Result<()> {
    // --- SECURE ---
    // By using `Account<'info, User>`, Anchor automatically checks that the first
    // 8 bytes of the `user_account`'s data match the unique discriminator for
    // the `User` struct.
    //
    // THE FIX:
    // If an attacker tries to pass a `Vault` account, its discriminator will not
    // match the one expected for a `User` account. The transaction will be
    // rejected by the Anchor runtime before this instruction logic is ever run.
    // The type confusion is no longer possible.

    let user = &ctx.accounts.user_account;
    msg!("SECURE: Withdrawing {} from account {}", user.balance, user.key());
    // In a real scenario, this would transfer `user.balance` lamports.

    Ok(())
}

#[derive(Accounts)]
pub struct WithdrawSecure<'info> {
    // --- SECURE ---
    // The `Account` type enforces that the provided account is not only owned by
    // the program but is also of the specific type `User`. The `has_one`
    // constraint also implicitly checks the authority.
    #[account(mut, has_one = authority)]
    pub user_account: Account<'info, User>,
    pub authority: Signer<'info>,
}


// --- Account Type 1 ---
#[account]
pub struct User {
    pub authority: Pubkey,
    pub balance: u64,
}

// --- Account Type 2 (The "Cosplayer") ---
// This can still exist in the program, but it can no longer be passed
// to the `withdraw_secure` instruction.
#[account]
pub struct Vault {
    pub admin: Pubkey,
    pub locked_amount: u64,
}
