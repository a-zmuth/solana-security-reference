# Solana Security Reference

A comprehensive, open-source educational resource for Solana developers to learn about common security vulnerabilities through clear, practical, and heavily-annotated examples.

This repository is designed to make security failures obvious, intuitive, and difficult to repeat. It is not about building large-scale programs but about providing focused, minimal examples that highlight specific vulnerabilities and their solutions.

## Philosophy

The core principle of this repository is "learning by breaking." For each vulnerability, we provide two contrasting implementations:

1.  **Vulnerable**: A version of an instruction that contains a specific, common security flaw.
2.  **Secure**: A corrected version of the same instruction where the vulnerability is patched.

The business logic between the two versions is identical. The only difference is the security handling. This side-by-side comparison, coupled with detailed inline comments, is designed to build a deep, intuitive understanding of not just *what* to do, but *why* you're doing it.

## How to Use This Repository

This is not a library or a framework. It is a collection of educational examples. To get the most out of this resource:

1.  **Start with a Vulnerability**: Navigate to one of the `src/` directories (e.g., `src/01-missing-signer-check`).
2.  **Read the Concept**: Each directory contains a `README.md` that explains the vulnerability pattern at a high level.
3.  **Analyze the Vulnerable Code**: Open the `vulnerable.rs` file. Read the code and the comments carefully. The comments will explain:
    *   What the developer *assumed*.
    *   Why that assumption is dangerously wrong.
    *   How an attacker would exploit the flaw.
4.  **Study the Secure Code**: Open the `secure.rs` file. Compare it with the vulnerable version. The comments will explain:
    *   Why the fix works.
    *   How it neutralizes the specific exploit vector.
    *   The security principle being applied.
5.  **Consult the Deep Dive**: For a more thorough conceptual understanding, read the [DEEP_DIVE.md](docs/DEEP_DIVE.md) document, which connects these practical examples to the underlying mechanics of the Solana runtime and account model.

## A Note for Learners

This repository is a tool for learning, not a library of components to be copied and pasted. The primary goal is to help you build a security-first mindset.

*   **Understand the 'Why'**: Don't just memorize the secure patterns. Understand *why* the vulnerable code is unsafe and *why* the fix works. The inline comments and the `DEEP_DIVE.md` document are designed to facilitate this understanding.
*   **Run the Tests**: The `tests/` directory will contain tests that demonstrate how to exploit the vulnerable instructions and how the secure versions prevent those exploits. Running these tests is a crucial step in seeing the consequences of these vulnerabilities firsthand.
*   **Think Like an Attacker**: For each example, ask yourself: "How else could I abuse this? What other assumptions is the developer making?" This is the path to becoming a security-conscious developer.
*   **Get Your Code Audited**: The patterns in this repository represent best practices, but they are not a substitute for a professional security audit. Always have your code reviewed by experienced auditors before deploying to a production environment.

## Repository Structure

The repository is organized by vulnerability pattern to allow for focused study.

```
.
├── Anchor.toml
├── Cargo.toml
├── docs
│   └── DEEP_DIVE.md
├── package.json
├── programs
│   ├── attacker/
│   │   ├── Cargo.toml
│   │   └── src
│   │       └── lib.rs
│   └── solana-security-reference/
│       ├── Cargo.toml
│       └── src
│           ├── lib.rs
│           ├── secure/
│           │   └── ... (secure instruction files)
│           └── vulnerable/
│               └── ... (vulnerable instruction files)
├── README.md
├── src
│   ├── 01-missing-signer-check/
│   │   ├── README.md
│   │   ├── vulnerable.rs
│   │   └── secure.rs
│   ├── 02-incorrect-owner-check/
│   │   ├── README.md
│   │   ├── vulnerable.rs
│   │   └── secure.rs
│   ├── 03-insecure-cpi/
│   │   ├── README.md
│   │   ├── vulnerable.rs
│   │   └── secure.rs
│   ├── 04-integer-overflow/
│   │   ├── README.md
│   │   ├── vulnerable.rs
│   │   └── secure.rs
│   └── 05-type-cosplay/
│       ├── README.md
│       ├── vulnerable.rs
│       └── secure.rs
├── tests/
│   ├── 01-missing-signer-check.ts
│   ├── 02-incorrect-owner-check.ts
│   ├── 03-insecure-cpi.ts
│   ├── 04-integer-overflow.ts
│   └── 05-type-cosplay.ts
└── tsconfig.json
```

*   `Anchor.toml`, `Cargo.toml`, `package.json`, `tsconfig.json`: Top-level configuration files for the Anchor workspace, Rust programs, and TypeScript tests.
*   `docs/`: Contains the high-level conceptual explanations, such as `DEEP_DIVE.md`.
*   `programs/`: Contains the Anchor programs.
    *   `programs/attacker/`: A helper program used in tests to simulate an attacker's program.
    *   `programs/solana-security-reference/`: The main Anchor program containing all vulnerable and secure instruction implementations.
*   `src/`: Contains the educational material, with each vulnerability in its own numbered directory. The `.rs` files here are for reading and reference, not for direct compilation.
*   `tests/`: Contains tests that demonstrate both the exploit and the fix for each vulnerability.
*   `README.md`: This file, providing an overview and guide to the repository.


This repository is for educational purposes only. While the secure patterns represent best practices, always have your code audited by experienced professionals before deploying to mainnet.