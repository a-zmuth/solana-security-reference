use anchor_lang::prelude::*;

// --- Vulnerable Instruction ---
// This instruction writes data to a `LogAccount`. The vulnerability is that it
// fails to check if the `log_account` is actually owned by this program.

pub fn write_to_log_vulnerable(ctx: Context<WriteToLogVulnerable>, data: u64) -> Result<()> {
    // --- VULNERABILITY ---
    // The instruction uses `UncheckedAccount` for `log_account`. This means Anchor
    // performs no ownership or type checks. The code then manually deserializes
    // the account's data.
    //
    // THE EXPLOIT:
    // An attacker can create an account owned by a *different* program and pass it
    // to this instruction. Because there is no owner check, this program will
    // blindly write data to an account it doesn't own.
    //
    // This could be used to corrupt the state of another application.

    let log_account = &mut ctx.accounts.log_account;

    // Manually and unsafely deserialize and serialize the account data.
    let mut account_data = log_account.data.borrow_mut();
    // This is a simplified representation. A real exploit might involve more
    // complex data structures that are misinterpreted.
    let mut log = LogAccount::try_deserialize(&mut &account_data[8..])?;
    log.data = data;
    log.serialize(&mut &mut account_data[8..])?;

    msg!("VULNERABLE: Wrote data {} to log account {}", data, log_account.key());

    Ok(())
}

#[derive(Accounts)]
pub struct WriteToLogVulnerable<'info> {
    // --- VULNERABILITY ---
    // Using `UncheckedAccount` puts the full responsibility of validation on the
    // developer. Without an explicit `owner` check, this can be owned by any program.
    #[account(mut)]
    pub log_account: UncheckedAccount<'info>,
}


// --- Shared Account State ---
#[account]
pub struct LogAccount {
    pub data: u64,
}
