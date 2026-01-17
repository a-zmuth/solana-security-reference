use anchor_lang::prelude::*;

// This module is included in the main lib.rs file.
// It is kept in a separate file for organizational purposes.

// --- Secure Instruction ---
// This instruction fixes the vulnerability present in the "vulnerable" version.
// It correctly ensures that the `authority` account has signed the transaction
// before allowing any modifications to the `GameData` account.

pub fn log_message_secure(ctx: Context<LogMessageSecure>) -> Result<()> {
    // --- SECURE ---
    // By using the `Signer` type in the `LogMessageSecure` context, Anchor
    // automatically verifies that `ctx.accounts.authority.is_signer == true`
    // before the instruction body is ever entered.
    //
    // THE FIX:
    // If an attacker tries the same exploit as before (passing the authority's
    // public key but signing with their own key), the transaction will be
    // rejected by the Anchor framework before this instruction logic ever runs.
    // This is the power of using Anchor's built-in account types for security.

    msg!("SECURE: Authority signed: {}", ctx.accounts.authority.key());
    msg!("SECURE: Game data is being modified...");
    ctx.accounts.game_data_account.score = 100;

    Ok(())
}

#[derive(Accounts)]
pub struct LogMessageSecure<'info> {
    #[account(mut, has_one = authority)]
    pub game_data_account: Account<'info, crate::vulnerable::missing_signer_check::GameData>,

    // --- SECURE ---
    // The `Signer` type automatically enforces the constraint that this account
    // must be a signer on the transaction. If it's not, the transaction is
    // rejected. This is the simplest and most effective way to prevent this
    // vulnerability.
    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,
}
