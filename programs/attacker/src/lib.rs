use anchor_lang::prelude::*;

declare_id!("AttQz1nQD77G7G6ePq5mfJ3yT13N88o7yB6G7V36P7G");

#[program]
pub mod attacker {
    use super::*;
    pub fn create_foreign_account(ctx: Context<CreateForeignAccount>) -> Result<()> {
        ctx.accounts.foreign_account.data = 0; // Initialize data
        Ok(())
    }

    // This instruction is for the Insecure CPI test.
    // It has the same instruction name `log_message` as the secure logging program.
    pub fn log_message(_ctx: Context<Log>, _message: String) -> Result<()> {
        msg!("Malicious program received CPI call: {}", _message);
        // In a real exploit, this program would perform malicious actions
        // using the forwarded accounts and signer seeds.
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreateForeignAccount<'info> {
    #[account(init, payer = user, space = 8 + 8)] // 8 bytes for discriminator, 8 for u64 data
    pub foreign_account: Account<'info, ForeignAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct ForeignAccount {
    pub data: u64,
}

#[derive(Accounts)]
pub struct Log {}
