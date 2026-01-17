# Vulnerability 2: Incorrect Owner Check

## The Concept

Every account on Solana has an `owner`, which is the public key of the program that is allowed to modify it. The Solana runtime enforces that only the owner of an account can change its data. Forgetting this check is a critical vulnerability.

An **Incorrect Owner Check** vulnerability occurs when an instruction attempts to deserialize and use an account without first verifying that the account is owned by the correct program (i.e., the program currently executing).

### The Developer's Mistake

A developer might create an instruction that needs to write to a state account. They might use an `AccountInfo` or `UncheckedAccount` and attempt to deserialize its data manually. The mistake is assuming that any account passed in by a user is of the correct type and owned by their program. If they forget to check `account.owner == program_id`, they open the door for a major exploit.

### The Exploit

An attacker can exploit this by passing in an account that is owned by a *different* program but has a data layout that *looks* similar to the one the vulnerable program expects. When the vulnerable program deserializes this foreign account's data, it might misinterpret the bytes, leading to corrupted state or unexpected behavior.

Worse, if the instruction writes data to the account, the attacker can cause the vulnerable program to write garbage data into an account owned by another program. This could be used to corrupt the state of a third-party application, potentially leading to theft of funds or other exploits in that separate system.

Anchor's `Account<T>` type automatically handles this check. It ensures that `account.owner == program_id` and that the account's discriminator matches `T`'s type, effectively preventing this vulnerability. The risk appears when developers bypass `Account<T>` and use more primitive types like `UncheckedAccount` without performing the checks manually.
