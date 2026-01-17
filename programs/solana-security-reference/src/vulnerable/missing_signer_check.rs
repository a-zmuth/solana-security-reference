use anchor_lang::prelude::*;

// This module is included in the main lib.rs file.
// It is kept in a separate file for organizational purposes.

// --- Vulnerable Instruction ---
// This instruction allows a user to "log a message" which simulates a privileged
// action on a `GameData` account. The vulnerability is that it does not check if
// the `authority` account has signed the transaction.

pub fn log_message_vulnerable(ctx: Context<LogMessageVulnerable>) -> Result<()> {
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
    // If this instruction were to transfer ownership of `game_data_account`, the
    // attacker could steal the account.

    msg!("VULNERABLE: Authority provided: {}", ctx.accounts.authority.key());
    msg!("VULNERABLE: Game data is being modified...");
    ctx.accounts.game_data_account.score = 100;

    Ok(())
}

#[derive(Accounts)]
pub struct LogMessageVulnerable<'info> {
    #[account(mut, has_one = authority)]
    pub game_data_account: Account<'info, GameData>,

    // --- VULNERABILITY ---
    // The authority account is passed in, but there is no constraint to ensure
    // it is a signer on the transaction. Anchor's `Signer` type would fix this,
    // but by using `UncheckedAccount`, the developer takes on the responsibility
    // of checking `is_signer` manually, which they have failed to do.
    #[account(mut)]
    pub authority: UncheckedAccount<'info>,

    pub system_program: Program<'info, System>,
}

// --- Shared Account State ---
// This is the state account that the instructions will operate on.
#[account]
pub struct GameData {
    pub score: u64,
    pub authority: Pubkey,
}
