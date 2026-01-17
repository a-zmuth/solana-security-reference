//! A vulnerable instruction that makes a CPI to an unverified program.

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
    // An attacker can deploy a malicious program with an instruction that has the
    // same signature as the `log_message` instruction in the intended target program.
    // They can then call this `cpi_insecure` instruction, but pass their malicious
    // program's address as the `logging_program` account.
    //
    // Our program will then make a CPI into the attacker's program. If our
    // program were to sign the CPI with a PDA, the attacker's program could now
    // use that signature to drain funds or perform other malicious actions.

    let logging_program = &ctx.accounts.logging_program;

    // Manually construct the instruction for the CPI.
    // The instruction data is crafted to match the target `log_message` instruction.
    let instruction = Instruction {
        program_id: logging_program.key(),
        accounts: vec![], // Assuming the target instruction takes no accounts
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
    // `UncheckedAccount` is used here, so no checks are performed. An attacker
    // can pass any executable account's address here.
    pub logging_program: UncheckedAccount<'info>,
    // In a real exploit, other accounts like PDAs would be passed here
    // and forwarded to the malicious program.
}
