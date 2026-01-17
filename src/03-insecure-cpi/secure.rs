//! A secure instruction that makes a CPI to a verified program.

use anchor_lang::prelude::*;

// We declare a new crate for the target program to demonstrate how Anchor
// links them. In a real project, this would be in a separate crate.
// We are simulating that here.
pub mod safe_logging_program {
    use super::*;
    declare_id!("metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s"); // Example ID
    #[program]
    pub mod safe_logger {
        use super::*;
        pub fn log_message(_ctx: Context<Log>, _message: String) -> Result<()> {
            // This is the trusted instruction we want to call.
            Ok(())
        }
    }
    #[derive(Accounts)]
    pub struct Log {}
}


// --- Secure Instruction ---
// This instruction securely makes a CPI by verifying the target program's address.

pub fn cpi_secure(ctx: Context<CpiSecure>, message: String) -> Result<()> {
    // --- SECURE ---
    // By using the `Program<'info, ...>` type, Anchor automatically validates that
    // the account provided for `logging_program` has a key that matches the
    // `declare_id!` of the `safe_logging_program`.
    //
    // THE FIX:
    // An attacker can no longer substitute a malicious program. If they provide an
    // account with a different program ID, the transaction will be rejected by the
    // Anchor runtime. This ensures that we only ever call the program we intend to.

    let cpi_program = ctx.accounts.logging_program.to_account_info();
    let cpi_accounts = safe_logging_program::cpi::accounts::Log {};
    let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

    safe_logging_program::cpi::log_message(cpi_ctx, message)?;

    msg!("SECURE: Made a CPI to a verified program: {}", ctx.accounts.logging_program.key());
    Ok(())
}

#[derive(Accounts)]
pub struct CpiSecure<'info> {
    // --- SECURE ---
    // The `Program` type is the key. It tells Anchor that this account must be
    // an executable program with the specific program ID of `safe_logging_program`.
    pub logging_program: Program<'info, safe_logging_program::safe_logger>,
}
