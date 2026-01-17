# Vulnerability 4: Integer Overflow and Underflow

## The Concept

Integer overflow and underflow are classic vulnerabilities in computer science that are especially dangerous in on-chain programs where arithmetic operations often determine ownership of assets. An overflow occurs when an arithmetic operation results in a number that is larger than the maximum value for its data type, causing it to "wrap around" to a small number. An underflow is the opposite, where an operation results in a number smaller than the minimum value, causing it to wrap around to a large number.

### The Developer's Mistake

In Rust, arithmetic operations in `debug` mode will panic on overflow or underflow, which is helpful during development. However, in `release` mode (which is used for on-chain deployments), these operations will silently wrap around. A developer might write code that adds or subtracts from a value (e.g., a token balance, a user's score) without considering the case where the value is at its maximum or minimum limit.

For example, `u64::MAX + 1` becomes `0`. And `u64::MIN - 1` (which is `0 - 1`) becomes `u64::MAX`.

### The Exploit

An attacker can abuse this behavior to manipulate the program's state in their favor.

*   **Overflow Exploit**: Imagine a program that allows users to deposit tokens and stores their balance in a `u64`. If a user has a balance close to `u64::MAX`, they could deposit a small amount, causing their balance to overflow and wrap around to a near-zero value. If the program then calculates the difference to determine a withdrawal amount, the logic may be tricked into giving the user more tokens than they are owed.

*   **Underflow Exploit**: Consider a vault where a user can withdraw tokens. If a user has 0 tokens and withdraws 1, their balance could underflow to `u64::MAX`. The program now thinks they have a massive balance, which could allow them to drain the entire vault. This exact scenario has been a major source of exploits in Solana protocols.

The fix is to never use standard arithmetic operators (`+`, `-`, `*`, `/`) for critical calculations. Instead, use the `checked_*` methods provided by Rust's integer types (e.g., `checked_add`, `checked_sub`). These methods return an `Option`, which will be `None` if an overflow or underflow occurs, allowing you to handle the error gracefully instead of letting the state become corrupted.
