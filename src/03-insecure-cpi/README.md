# Vulnerability 3: Insecure Cross-Program Invocation (CPI)

## The Concept

Cross-Program Invocations (CPIs) are a fundamental part of Solana's composability. They allow one program to call instructions on another program. However, this power comes with risks.

An **Insecure CPI** vulnerability occurs when a program makes a CPI to another program whose address is not verified. If the address of the target program is supplied by the user without any checks, an attacker can substitute a malicious program, leading to completely unintended consequences.

### The Developer's Mistake

A developer wants their program to interact with another trusted program on the network (e.g., the SPL Token program, a specific oracle). They create an instruction that accepts the target program's address as one of its accounts. The mistake is assuming the user will always provide the *correct* program address. They use this account directly in a CPI call without hardcoding the expected address or checking it against a known value.

### The Exploit

An attacker can deploy their own malicious program that has an instruction with the same signature as the one the victim program intends to call. When calling the victim program's instruction, the attacker passes the address of their *malicious* program instead of the legitimate one.

The vulnerable program doesn't check the address. It just sees an executable account and makes the CPI. The attacker's program now receives the CPI, potentially with privileged signer seeds from a PDA, allowing it to impersonate the victim program and sign transactions on its behalf. This can be used to drain funds from PDAs, mint unauthorized tokens, or corrupt state.

Anchor helps mitigate this by providing the `Program<T>` account type. When you define a field as `pub another_program: Program<'info, AnotherProgram>`, Anchor verifies that the public key of the passed account matches the ID declared in the `another_program` crate. This ensures you are always calling the program you expect. The vulnerability is introduced when this type is bypassed in favor of `AccountInfo` or `UncheckedAccount`.
