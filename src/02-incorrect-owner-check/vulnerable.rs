//! A vulnerable instruction that fails to check the owner of a state account.

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
    // This could be used to corrupt the state of another application, potentially
    // causing exploits in that other system. For example, an attacker could
    // overwrite a `price` field in another protocol's oracle account if the
    // data layouts are similar.

    let log_account = &mut ctx.accounts.log_account;

    // Manually and unsafely deserialize the account data.
    // This is a simplified example. In a real-world scenario, a developer might
    // use `try_from_slice_unchecked` or a similar unsafe method.
    let mut account_data = log_account.data.borrow_mut();
    let mut log = LogAccount::try_deserialize(&mut &account_data[..])?;
    log.data = data;
    log.serialize(&mut &mut account_data[..])?;

    msg!("VULNERABLE: Wrote data {} to log account {}", data, log_account.key());

    Ok(())
}


#[derive(Accounts)]
pub struct WriteToLogVulnerable<'info> {
    // --- VULNERABILITY ---
    // Using `UncheckedAccount` puts the full responsibility of validation on the
    // developer. Without an explicit `owner` check in the instruction logic, this
    // account can be owned by any program.
    #[account(mut)]
    pub log_account: UncheckedAccount<'info>,
}


// --- Shared Account State ---
// This is the account that the instruction will write to.
#[account]
pub struct LogAccount {
    pub data: u64,
}
