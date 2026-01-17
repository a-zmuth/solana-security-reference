# Vulnerability 1: Missing Signer Check

## The Concept

On Solana, many critical actions require explicit authorization. This authorization is proven by requiring an account to be a "signer" on the transaction. If an account is passed into an instruction, the Solana runtime checks whether that account's private key was used to sign the transaction.

A **Missing Signer Check** vulnerability occurs when an instruction performs a privileged action based on an account's authority but fails to verify that the account has actually signed the transaction.

### The Developer's Mistake

A developer might write an instruction that takes an `authority` account and a `state` account. The instruction then modifies the `state` account, assuming that the `authority` account provided is legitimate. The mistake is assuming that simply *receiving* an account's public key is proof of its consent.

### The Exploit

An attacker can exploit this by creating a transaction that calls the vulnerable instruction. For the `authority` account, the attacker simply passes the public key of the *actual* authority. However, the attacker signs the transaction with their *own* key, not the authority's.

The program, failing to check `authority.is_signer`, sees the public key and assumes it has granted permission. The program then proceeds with the privileged action (e.g., transferring ownership, withdrawing funds), allowing the attacker to control the state account without ever having the authority's private key.

Anchor helps prevent this with the `Signer` account type, which combines the check `is_signer == true` with deserialization. However, a developer can still introduce this vulnerability by using `AccountInfo` or `UncheckedAccount` and forgetting to perform the check manually.
