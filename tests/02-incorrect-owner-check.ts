import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { SolanaSecurityReference } from "../target/types/solana_security_reference";
import { expect } from "chai";

describe("2. Incorrect Owner Check", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.SolanaSecurityReference as Program<SolanaSecurityReference>;
  const attackerProgram = anchor.workspace.Attacker as Program<any>; // A separate malicious program

  const user = anchor.web3.Keypair.generate();
  const foreignAccount = anchor.web3.Keypair.generate();

  before(async () => {
    // Airdrop funds
    await provider.connection.requestAirdrop(user.publicKey, 10 * anchor.web3.LAMPORTS_PER_SOL);

    // Attacker program creates an account that looks like our LogAccount
    await attackerProgram.methods
      .createForeignAccount()
      .accounts({
        foreignAccount: foreignAccount.publicKey,
        user: user.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([foreignAccount, user])
      .rpc();
  });

  it("Is exploited", async () => {
    // --- The Exploit ---
    // The user calls `vulnerable_owner_check`, but passes in the `foreignAccount`
    // which is owned by the `attackerProgram`.
    const newData = new anchor.BN(777);
    try {
      await program.methods
        .vulnerableOwnerCheck(newData)
        .accounts({
          logAccount: foreignAccount.publicKey, // Pass the account owned by the other program
        })
        .rpc();
    } catch (error) {
      console.error("Exploit failed when it should have succeeded.");
      throw error;
    }

    // Check that our program wrote data to the foreign account
    const data = await attackerProgram.account.foreignAccount.fetch(foreignAccount.publicKey);
    expect(data.data.toNumber()).to.equal(newData.toNumber());
  });

  it("Is prevented", async () => {
    // --- The Fix ---
    // The user attempts the same exploit against the `secure_owner_check` instruction.
    const newData = new anchor.BN(888);
    try {
      await program.methods
        .secureOwnerCheck(newData)
        .accounts({
          logAccount: foreignAccount.publicKey, // Pass the account owned by the other program
        })
        .rpc();

      throw new Error("The instruction should have failed but didn't.");
    } catch (error) {
      // We expect this to fail because the `Account<T>` deserializer in Anchor
      // checks if `logAccount.owner == program.programId`. Since the owner is
      // the `attackerProgram`, this check fails.
      expect(error.message).to.include("Account does not have correct owner");
    }
  });
});

// A simple attacker program is assumed to be in the workspace for this test.
// You would need to add a program named `attacker` to `programs/` with the following:
/*
use anchor_lang::prelude::*;

declare_id!("attack81...);

#[program]
pub mod attacker {
    use super::*;
    pub fn create_foreign_account(ctx: Context<CreateForeignAccount>) -> Result<()> {
        ctx.accounts.foreign_account.data = 0;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreateForeignAccount<'info> {
    #[account(init, payer = user, space = 8 + 8)]
    pub foreign_account: Account<'info, ForeignAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct ForeignAccount {
    pub data: u64,
}
*/
