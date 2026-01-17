use anchor_lang::prelude::*;

// --- Mock Target Program ---
// In a real project, this would be a separate crate. We define it here
// to simulate the dependency and allow Anchor to check the program ID.
pub mod safe_logging_program {
    use super::*;
    // This is the hardcoded, trusted program ID we expect to call.
    declare_id!("metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s");
    #[program]
    pub mod safe_logger {
        use super::*;
        pub fn log_message(_ctx: Context<Log>, _message: String) -> Result<()> {
            msg!("Log message instruction was successfully called in the target program.");
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
    // An attacker can no longer substitute a malicious program.

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
