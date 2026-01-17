import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { SolanaSecurityReference } from "../target/types/solana_security_reference";
import { Attacker } from "../target/types/attacker"; // Import the attacker program IDL
import { expect } from "chai";

describe("3. Insecure CPI", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.SolanaSecurityReference as Program<SolanaSecurityReference>;
  const attackerProgram = anchor.workspace.Attacker as Program<Attacker>;

  // A mock trusted logging program (this would typically be a separate deploy)
  const trustedLoggerProgram = anchor.workspace.SafeLogger as Program<any>; // Using 'any' as we don't have its IDL directly

  it("Is exploited", async () => {
    // --- The Exploit ---
    // The attacker calls `vulnerable_insecure_cpi` but passes their `attackerProgram.programId`
    // instead of the `trustedLoggerProgram.programId`.
    const message = "Malicious CPI initiated!";

    try {
      await program.methods
        .vulnerableInsecureCpi(message)
        .accounts({
          loggingProgram: attackerProgram.programId, // Pass the attacker's program ID
        })
        .rpc();
    } catch (error) {
      console.error("Exploit failed when it should have succeeded.");
      throw error;
    }
    // No explicit check here, but the log output from the malicious program would confirm the exploit.
    // In a real scenario, the malicious program would then attempt to drain funds or corrupt state.
  });

  it("Is prevented", async () => {
    // --- The Fix ---
    // The attacker attempts the same exploit against the `secure_insecure_cpi` instruction.
    const message = "Attempted malicious CPI.";

    try {
      await program.methods
        .secureInsecureCpi(message)
        .accounts({
          loggingProgram: attackerProgram.programId, // Still pass the attacker's program ID
        })
        .rpc();

      throw new Error("The instruction should have failed but didn't.");
    } catch (error) {
      // We expect this to fail because the `Program<safe_logging_program::safe_logger>`
      // constraint in Anchor will verify that `loggingProgram.key()` matches the
      // `declare_id!` of the `safe_logging_program`.
      expect(error.message).to.include("Program ID mismatch");
    }
  });
});
