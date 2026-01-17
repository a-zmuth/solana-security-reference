use anchor_lang::prelude::*;
use crate::vulnerable::incorrect_owner_check::LogAccount;

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

    let log_account = &mut ctx.accounts.log_account;
    log_account.data = data;

    msg!("SECURE: Wrote data {} to log account {}", data, log_account.key());

    Ok(())
}

#[derive(Accounts)]
pub struct WriteToLogSecure<'info> {
    // --- SECURE ---
    // `Account<'info, LogAccount>` tells Anchor to deserialize the account's
    // data into the `LogAccount` struct and perform all necessary checks.
    #[account(mut)]
    pub log_account: Account<'info, LogAccount>,
}
