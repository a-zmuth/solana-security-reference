use anchor_lang::prelude::*;
use anchor_lang::solana_program::instruction::Instruction;
use anchor_lang::solana_program::program::invoke;


// --- Vulnerable Instruction ---
// This instruction intends to make a CPI to a `logging_program` to record a message.
// The vulnerability is that it doesn't check the address of the `logging_program`.

pub fn cpi_insecure(ctx: Context<CpiInsecure>, message: String) -> Result<()> {
    // --- VULNERABILITY ---
    // The `logging_program` is taken directly from the user-supplied accounts as an
    // `UncheckedAccount`. There is no validation to ensure this is the program we
    // actually want to call.
    //
    // THE EXPLOIT:
    // An attacker can deploy a malicious program and pass its address as the
    // `logging_program` account. Our program will then make a CPI into it.

    let logging_program = &ctx.accounts.logging_program;

    // Manually construct the instruction for the CPI.
    let instruction = Instruction {
        program_id: logging_program.key(),
        accounts: vec![],
        data: message.into_bytes(),
    };

    // Invoke the CPI.
    invoke(
        &instruction,
        &[
            logging_program.to_account_info(),
        ],
    )?;

    msg!("VULNERABLE: Made a CPI to an unchecked program: {}", logging_program.key());
    Ok(())
}

#[derive(Accounts)]
pub struct CpiInsecure<'info> {
    // --- VULNERABILITY ---
    // `UncheckedAccount` is used here, so no checks are performed.
    pub logging_program: UncheckedAccount<'info>,
}
