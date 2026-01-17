//! A secure instruction that correctly checks the owner of a state account.

use anchor_lang::prelude::*;

// --- Secure Instruction ---
// This instruction fixes the owner check vulnerability by using Anchor's `Account<T>`
// type, which handles validation automatically.

pub fn write_to_log_secure(ctx: Context<WriteToLogSecure>, data: u64) -> Result<()> {
    // --- SECURE ---
    // By using `Account<'info, LogAccount>`, Anchor automatically performs two
    // critical checks before the instruction logic is run:
    // 1. Owner Check: It verifies that `log_account.owner == program_id`.
    // 2. Type Check: It checks that the 8-byte discriminator at the start of the
    //    account's data matches the hash of the `LogAccount` type.
    //
    // THE FIX:
    // An attacker can no longer pass an account owned by a different program.
    // The transaction would be rejected by the Anchor runtime. This prevents the
    // program from accidentally modifying the state of another application.

    let log_account = &mut ctx.accounts.log_account;
    log_account.data = data;

    msg!("SECURE: Wrote data {} to log account {}", data, log_account.key());

    Ok(())
}

#[derive(Accounts)]
pub struct WriteToLogSecure<'info> {
    // --- SECURE ---
    // The `Account<'info, LogAccount>` type is the key to the fix. It tells Anchor
    // to deserialize the account's data into the `LogAccount` struct and,
    // crucially, to perform the necessary ownership and type checks.
    #[account(mut, seeds = [b"log"], bump)]
    pub log_account: Account<'info, LogAccount>,
}

// --- Shared Account State ---
// We add an `init` constraint to the account to make it easier to create
// during testing and client-side interaction.
#[account]
pub struct LogAccount {
    pub data: u64,
}
