use anchor_lang::prelude::*;

// --- Vulnerable Instruction ---
// This instruction increments a counter. The vulnerability is that it uses
// standard arithmetic, which can overflow in a production `release` build.

pub fn increment_vulnerable(ctx: Context<IncrementVulnerable>) -> Result<()> {
    // --- VULNERABILITY ---
    // The instruction uses the `+` operator. In `release` mode, this will
    // silently wrap around on overflow.
    //
    // THE EXPLOIT:
    // If `counter.count` is at `u64::MAX`, adding 1 will cause it to wrap to 0.

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
