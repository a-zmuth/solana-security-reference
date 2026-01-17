use anchor_lang::prelude::*;
use crate::vulnerable::integer_overflow::Counter;

// --- Secure Instruction ---
// This instruction safely increments a counter by using `checked_add`.

pub fn increment_secure(ctx: Context<IncrementSecure>) -> Result<()> {
    // --- SECURE ---
    // We use `checked_add`, which returns an `Option`. If the operation
    // would overflow, it returns `None`, which we handle by returning an error.
    //
    // THE FIX:
    // The program will now fail explicitly instead of silently corrupting state.

    let counter = &mut ctx.accounts.counter;
    counter.count = counter.count.checked_add(1).ok_or(ErrorCode::Overflow)?;

    msg!("SECURE: Counter incremented to {}", counter.count);
    Ok(())
}

#[derive(Accounts)]
pub struct IncrementSecure<'info> {
    #[account(mut)]
    pub counter: Account<'info, Counter>,
}

// --- Custom Error ---
#[error_code]
pub enum ErrorCode {
    #[msg("The operation resulted in an integer overflow.")]
    Overflow,
}
