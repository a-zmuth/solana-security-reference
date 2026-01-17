//! A secure instruction that prevents integer overflow.

use anchor_lang::prelude::*;

// --- Secure Instruction ---
// This instruction safely increments a counter by using `checked_add` to prevent
// overflow.

pub fn increment_secure(ctx: Context<IncrementSecure>) -> Result<()> {
    // --- SECURE ---
    // Instead of standard arithmetic, we use the `checked_add` method. This method
    // returns an `Option<u64>`:
    // - `Some(result)` if the addition is successful.
    // - `None` if the addition would result in an overflow.
    //
    // THE FIX:
    // We can now explicitly handle the overflow case and return a custom error,
    // preventing the silent wrap-around. An attacker can no longer cause the
    // counter to reset to zero. The transaction will simply fail.

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

// --- Shared Account State ---
#[account]
pub struct Counter {
    pub count: u64,
}

// --- Custom Error ---
#[error_code]
pub enum ErrorCode {
    #[msg("The operation resulted in an integer overflow.")]
    Overflow,
}
