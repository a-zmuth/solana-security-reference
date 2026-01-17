import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { SolanaSecurityReference } from "../target/types/solana_security_reference";
import { expect } from "chai";

describe("4. Integer Overflow/Underflow", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.SolanaSecurityReference as Program<SolanaSecurityReference>;

  const counterAccount = anchor.web3.Keypair.generate();
  const user = provider.wallet.publicKey;

  before(async () => {
    // Initialize the Counter account to 0
    await program.methods
      .initializeCounter(new anchor.BN(0))
      .accounts({
        counter: counterAccount.publicKey,
        signer: user,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([counterAccount])
      .rpc();
  });

  it("Is exploited (Overflow)", async () => {
    // Set the counter to u64::MAX - 2 (18446744073709551614 - 2)
    const initialCount = new anchor.BN("18446744073709551614");
    await program.methods
      .initializeCounter(initialCount)
      .accounts({
        counter: counterAccount.publicKey,
        signer: user,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([counterAccount])
      .rpc();


    // Increment it once to u64::MAX - 1
    await program.methods
      .vulnerableIntegerOverflow()
      .accounts({
        counter: counterAccount.publicKey,
      })
      .rpc();
      
    // Increment it once more to u64::MAX
    await program.methods
      .vulnerableIntegerOverflow()
      .accounts({
        counter: counterAccount.publicKey,
      })
      .rpc();

    // --- The Exploit ---
    // Incrementing one more time should cause an overflow to 0
    await program.methods
      .vulnerableIntegerOverflow()
      .accounts({
        counter: counterAccount.publicKey,
      })
      .rpc();

    // Check that the count has wrapped around to 0
    const data = await program.account.counter.fetch(counterAccount.publicKey);
    expect(data.count.toNumber()).to.equal(0);
  });

  it("Is prevented (Overflow)", async () => {
    // Set the counter to u64::MAX - 1 (18446744073709551614)
    const initialCount = new anchor.BN("18446744073709551614");
    await program.methods
      .initializeCounter(initialCount)
      .accounts({
        counter: counterAccount.publicKey,
        signer: user,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([counterAccount])
      .rpc();

    // Increment it once more to u64::MAX
    await program.methods
      .secureIntegerOverflow()
      .accounts({
        counter: counterAccount.publicKey,
      })
      .rpc();

    // --- The Fix ---
    // Attempt to increment one more time, which should now fail
    try {
      await program.methods
        .secureIntegerOverflow()
        .accounts({
          counter: counterAccount.publicKey,
        })
        .rpc();

      throw new Error("The instruction should have failed but didn't.");
    } catch (error) {
      // We expect the transaction to fail due to the custom Overflow error
      expect(error.message).to.include("The operation resulted in an integer overflow.");
    }
  });
});
