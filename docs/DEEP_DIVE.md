# Solana Security: A Deep Dive

This document provides a deeper conceptual understanding of the security principles illustrated in this repository. The examples in the `src/` directory show you the *what* and the *how*. This deep dive explains the *why*.

## The Core Principle: Zero Trust

The single most important principle for Solana security is **Zero Trust**.

You must treat every account passed into your instruction as potentially malicious. The Solana runtime and the Anchor framework provide you with the tools to establish trust, but it is your responsibility to use them. Never assume an account is what it claims to be. **Verify, then trust.**

This principle underlies every vulnerability discussed in this repository.

*   **Missing Signer Check**: You trusted an account to be an authority without verifying it signed the transaction.
*   **Incorrect Owner Check**: You trusted an account to be owned by your program without verifying its owner field.
*   **Insecure CPI**: You trusted a program account to be the correct one without verifying its address.
*   **Type Cosplay**: You trusted an account's data to be of a certain type without verifying its discriminator.
*   **Integer Overflow**: You trusted an arithmetic operation not to wrap around.

Security on Solana is not about memorizing a list of exploits. It's about adopting a mindset of radical skepticism and using the tools available to enforce the invariants your program requires.

## The Solana Account Model: A Double-Edged Sword

Solana's account model is incredibly powerful. The fact that programs are stateless and all data lives in separate "account" files is what makes the network so performant and composable. However, this separation of code and data is the primary source of security vulnerabilities.

Unlike in a traditional object-oriented model where an object's data and methods are tightly coupled, a Solana program is a universal executable that can be called by anyone. The accounts it operates on are passed in from the outside.

This means you must re-establish context and verify permissions in *every single instruction*.

### Who Can Modify Account Data?

Only the **owner** of an account can modify its data.

This is a fundamental rule enforced by the Solana runtime. When your program creates an account (e.g., via `#[account(init, ...)]`), it sets itself as the owner. From that point on, only your program can write to that account's data.

This is why the **Incorrect Owner Check** is so critical. If your program fails to verify ownership, it might deserialize and read data from an account owned by another program. But if it tries to *write* to that account, the transaction will fail at the runtime level. The more dangerous scenario is when your instruction is tricked into writing to an account it *does* own, but based on misinterpreted data from an account it *doesn't* own.

## Anchor: Your Safety Net (With Holes)

The Anchor framework is an invaluable tool for Solana security. It provides a set of "account types" that automatically handle most of the common security checks for you.

*   `Signer<'info>`: Verifies `account.is_signer == true`.
*   `Account<'info, T>`: Verifies `account.owner == program_id` AND that the account's discriminator matches the type `T`.
*   `Program<'info, T>`: Verifies `account.key == T::id()`.

Using these types is the single best thing you can do to improve the security of your programs.

So where do the vulnerabilities come from? They arise when developers, for one reason or another, choose to bypass these safety nets. The most common culprit is `UncheckedAccount<'info>` (or its even more raw cousin, `AccountInfo<'info>`).

There are legitimate reasons to use these types, especially for complex operations or when dealing with accounts that don't conform to the standard Anchor layout. However, the moment you type `UncheckedAccount`, you are telling Anchor, "I know what I'm doing. I will perform all the necessary security checks myself."

This repository shows what happens when that responsibility is not met.

## A Mental Checklist for Every Instruction

When writing an instruction, go through this mental checklist for every account you receive:

1.  **Authorization**: If this instruction performs a privileged action, have I verified the authority? Is the correct account marked as a `Signer`?
2.  **Ownership**: If I am reading or writing to a state account, have I ensured it is owned by my program? Am I using `Account<T>`?
3.  **Type Safety**: If I am deserializing an account, am I certain it is the correct type? Does it have the right discriminator? Am I using `Account<T>`?
4.  **Program Calls**: If I am making a CPI, am I certain I am calling the correct program? Is the address hardcoded or checked via the `Program<T>` type?
5.  **Arithmetic**: If I am performing any math that affects user balances, token amounts, or critical state, am I using `checked_` methods to prevent overflow/underflow?

By rigorously applying this checklist, you can move from a reactive "bug-fixing" model of security to a proactive, principled one. This is the difference between a program that is "not known to be insecure" and one that is demonstrably secure.
