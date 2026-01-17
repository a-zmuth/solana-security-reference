# Vulnerability 5: Type Cosplay / Incorrect Account Deserialization

## The Concept

When an Anchor program defines a struct with `#[account]`, it automatically creates an 8-byte "discriminator" which is a hash of the struct's name. This discriminator is stored as the first 8 bytes of the account's data and serves as a unique type identifier.

A **Type Cosplay** vulnerability (also known as type confusion or incorrect deserialization) occurs when an instruction fails to check an account's discriminator before deserializing its data. This allows an attacker to pass an account of a *different* type that may have a similar data layout, causing the program to misinterpret the data.

### The Developer's Mistake

A developer might need to access an account's data without wanting to use Anchor's full `Account<T>` machinery. They might use `UncheckedAccount` or `AccountInfo` and then attempt to deserialize the data manually using a function like `try_from_slice`. The mistake is that `try_from_slice` only cares about the data layout; it does *not* check the 8-byte discriminator. The developer assumes that the user will always pass an account of the correct type.

### The Exploit

An attacker finds two different account types in a program that have a similar memory layout but different meanings. For example:

*   `struct User { authority: Pubkey, balance: u64 }`
*   `struct Vault { admin: Pubkey, locked_amount: u64 }`

Both structs consist of a `Pubkey` followed by a `u64`. An instruction that is supposed to operate on a `User` account might have a function to withdraw the `balance`. If this instruction is vulnerable to type cosplay, an attacker can pass a `Vault` account instead of a `User` account.

The vulnerable instruction skips the discriminator check and deserializes the `Vault` account's data *as if it were a User*. It reads the `vault.admin` field as the `user.authority` and the `vault.locked_amount` as the `user.balance`. If the attacker is the `vault.admin`, the program now thinks they are the `user.authority` and may authorize them to "withdraw" the `locked_amount`, effectively stealing the funds from the vault.

Anchor's `Account<T>` type completely prevents this vulnerability by always checking that `account.data[0..8]` matches the discriminator for type `T`. The vulnerability is only introduced when a developer bypasses this feature.
