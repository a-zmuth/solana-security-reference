//! A vulnerable instruction that is susceptible to integer overflow.

use anchor_lang::prelude::*;

// --- Vulnerable Instruction ---
// This instruction increments a counter. The vulnerability is that it uses
// standard arithmetic, which can overflow in a production `release` build.

pub fn increment_vulnerable(ctx: Context<IncrementVulnerable>) -> Result<()> {
    // --- VULNERABILITY ---
    // The instruction uses the `+` operator for arithmetic. In a `debug` build,
    // this would panic on overflow. However, on-chain programs are built in
    // `release` mode, where overflow *silently wraps around*.
    //
    // THE EXPLOIT:
    // If `counter.count` is at `u64::MAX`, adding 1 will cause it to wrap
    // around to 0. An attacker could abuse this to reset a counter or manipulate
    // logic that depends on it. For example, if this counter tracked the number
    // of items in a vault, an attacker could make it appear empty.

    let counter = &mut ctx.accounts.counter;
    counter.count += 1; // Unsafe addition

    msg!("VULNERABLE: Counter incremented to {}", counter.count);
    Ok(())
}

#[derive(Accounts)]
pub struct IncrementVulnerable<'info> {
    #[account(mut)]
    pub counter: Account<'info, Counter>,
}

// --- Shared Account State ---
#[account]
pub struct Counter {
    pub count: u64,
}
