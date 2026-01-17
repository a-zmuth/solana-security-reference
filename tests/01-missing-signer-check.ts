import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { SolanaSecurityReference } from "../target/types/solana_security_reference";
import { expect } from "chai";

describe("1. Missing Signer Check", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.SolanaSecurityReference as Program<SolanaSecurityReference>;

  // The legitimate authority for the GameData account
  const authority = anchor.web3.Keypair.generate();
  // The attacker's keypair
  const attacker = anchor.web3.Keypair.generate();

  // The GameData account that will be manipulated
  const gameDataAccount = anchor.web3.Keypair.generate();

  before(async () => {
    // Airdrop funds to the authority and attacker
    await provider.connection.requestAirdrop(authority.publicKey, 100 * anchor.web3.LAMPORTS_PER_SOL);
    await provider.connection.requestAirdrop(attacker.publicKey, 100 * anchor.web3.LAMPORTS_PER_SOL);

    // Create the GameData account
    await program.methods
      .vulnerableOwnerCheck(new anchor.BN(0)) // Using another instruction to create the account for simplicity
      .accounts({
        logAccount: gameDataAccount.publicKey,
        user: authority.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([gameDataAccount, authority])
      .rpc();
  });

  it("Is exploited", async () => {
    // --- The Exploit ---
    // The attacker calls the `vulnerable_signer_check` instruction.
    // They pass the `authority`'s public key, but sign the transaction
    // with their own `attacker` key.
    try {
      await program.methods
        .vulnerableSignerCheck()
        .accounts({
          gameDataAccount: gameDataAccount.publicKey,
          authority: authority.publicKey, // Pass the real authority's key
        })
        .signers([attacker]) // But sign with the attacker's key
        .rpc();
    } catch (error) {
      // We expect this to succeed, so an error means the exploit failed
      console.error("Exploit failed when it should have succeeded.");
      throw error;
    }

    // Check that the score was updated, proving the exploit was successful
    const data = await program.account.gameData.fetch(gameDataAccount.publicKey);
    expect(data.score.toNumber()).to.equal(100);
  });

  it("Is prevented", async () => {
    // --- The Fix ---
    // The attacker attempts the same exploit against the `secure_signer_check`
    // instruction.
    try {
      await program.methods
        .secureSignerCheck()
        .accounts({
          gameDataAccount: gameDataAccount.publicKey,
          authority: authority.publicKey,
        })
        .signers([attacker]) // Signing with the wrong key
        .rpc();

      // If the instruction succeeds, the check has failed.
      throw new Error("The instruction should have failed but didn't.");
    } catch (error) {
      // We expect the transaction to fail. Anchor will throw an error when
      // it sees that the `authority` account is not a signer.
      // The error message should indicate a signature verification failure.
      expect(error.message).to.include("Signature verification failed");
    }
  });
});
