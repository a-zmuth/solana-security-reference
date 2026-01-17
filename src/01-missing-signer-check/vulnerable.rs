//! A vulnerable instruction that fails to check if the authority has signed the transaction.

use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod vulnerable_missing_signer {
    use super::*;
    pub fn log_message(ctx: Context<LogMessage>) -> Result<()> {
        // --- VULNERABILITY ---
        // The developer assumes that by providing the `authority` account via the context,
        // they are implicitly checking for the authority's permission.
        //
        // THE EXPLOIT:
        // An attacker can call this instruction and pass in the public key of the
        // legitimate authority. However, the transaction itself can be signed by ANY key.
        // The program doesn't check if `authority.is_signer` is true, so it proceeds
        // as if the real authority approved it.
        //
        // This instruction doesn't do anything dangerous, but if it were to, for example,
        // transfer ownership of `game_data_account` to a new authority, the attacker
        // could steal the account.

        msg!("Authority provided: {}", ctx.accounts.authority.key());
        msg!("Game data is being modified...");
        ctx.accounts.game_data_account.score = 100;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct LogMessage<'info> {
    #[account(mut, has_one = authority)]
    pub game_data_account: Account<'info, GameData>,

    // --- VULNERABILITY ---
    // The authority account is passed in, but there is no constraint to ensure
    // it is a signer on the transaction. Anchor's `Signer` type would fix this,
    // but by using `AccountInfo` or `UncheckedAccount`, the developer takes on
    // the responsibility of checking `is_signer` manually, which they have failed to do.
    // Using `UncheckedAccount` is a common way to accidentally introduce this bug.
    #[account(mut)]
    pub authority: UncheckedAccount<'info>,

    pub system_program: Program<'info, System>,
}


#[account]
pub struct GameData {
    pub score: u64,
    pub authority: Pubkey,
}
