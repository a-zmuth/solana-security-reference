import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { SolanaSecurityReference } from "../target/types/solana_security_reference";
import { expect } from "chai";

describe("5. Type Cosplay", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.SolanaSecurityReference as Program<SolanaSecurityReference>;

  const userAccount = anchor.web3.Keypair.generate();
  const vaultAccount = anchor.web3.Keypair.generate();
  const authority = anchor.web3.Keypair.generate();

  before(async () => {
    // Airdrop funds to the authority
    await provider.connection.requestAirdrop(authority.publicKey, 10 * anchor.web3.LAMPORTS_PER_SOL);

    // Initialize a User account
    await program.methods
      .initializeUser(authority.publicKey, new anchor.BN(100))
      .accounts({
        userAccount: userAccount.publicKey,
        signer: provider.wallet.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([userAccount])
      .rpc();

    // Initialize a Vault account (similar data layout, different type)
    await program.methods
      .initializeVault(authority.publicKey, new anchor.BN(5000))
      .accounts({
        vaultAccount: vaultAccount.publicKey,
        signer: provider.wallet.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([vaultAccount])
      .rpc();
  });

  it("Is exploited", async () => {
    // --- The Exploit ---
    // The attacker calls `vulnerable_type_cosplay`, passing the `vaultAccount`
    // instead of the `userAccount`.
    try {
      await program.methods
        .vulnerableTypeCosplay()
        .accounts({
          userAccount: vaultAccount.publicKey, // Pass the Vault account
          authority: authority.publicKey, // The authority is the admin of the vault
        })
        .rpc();
    } catch (error) {
      console.error("Exploit failed when it should have succeeded.");
      throw error;
    }

    // No explicit check needed here, the instruction would log the "withdrawal" of 5000 from the vault.
    // In a real scenario, this would have drained the vault.
  });

  it("Is prevented", async () => {
    // --- The Fix ---
    // The attacker attempts the same exploit against the `secure_type_cosplay` instruction.
    try {
      await program.methods
        .secureTypeCosplay()
        .accounts({
          userAccount: vaultAccount.publicKey, // Still pass the Vault account
          authority: authority.publicKey,
        })
        .rpc();

      throw new Error("The instruction should have failed but didn't.");
    } catch (error) {
      // We expect this to fail because the `Account<'info, User>` deserializer
      // in Anchor checks the discriminator.
      expect(error.message).to.include("AccountDiscriminatorMismatch");
    }
  });
});
