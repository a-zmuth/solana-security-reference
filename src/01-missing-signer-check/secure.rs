//! A secure instruction that correctly checks if the authority has signed the transaction.

use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod secure_missing_signer {
    use super::*;
    pub fn log_message(ctx: Context<LogMessage>) -> Result<()> {
        // --- SECURE ---
        // By using the `Signer` type in the `LogMessage` context, Anchor automatically
        // verifies that `ctx.accounts.authority.is_signer == true` before the instruction
        // body is ever entered.
        //
        // THE FIX:
        // If an attacker tries the same exploit as before (passing the authority's
        // public key but signing with their own key), the transaction will be rejected
        // by the Anchor framework before this instruction logic ever runs.
        // This is the power of using Anchor's built-in account types.

        msg!("Authority signed: {}", ctx.accounts.authority.key());
        msg!("Game data is being modified...");
        ctx.accounts.game_data_account.score = 100;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct LogMessage<'info> {
    #[account(mut, has_one = authority)]
    pub game_data_account: Account<'info, GameData>,

    // --- SECURE ---
    // The `Signer` type automatically enforces the constraint that this account
    // must be a signer on the transaction. If it's not, the transaction is
    // rejected. This is the simplest and most effective way to prevent this
    // vulnerability.
    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,
}


#[account]
pub struct GameData {
    pub score: u64,
    pub authority: Pubkey,
}
