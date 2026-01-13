# praborrow-macros

English | [Indonesia](./README_ID.md)

Procedural macros for the PraBorrow system.

## Overview

This collection of procedural macros reduces boilerplate and enforces strict semantics within the PraBorrow ecosystem. It includes derivation logic for core traits.

## Key Features

- **`#[derive(Constitution)]`**: Enforces invariant checking on structs, generating `verify_invariants` methods.
- **`#[derive(Target)]`**: Implements resource addressing and identification logic.
- **Compile-Time Checks**: Performs static analysis on macro inputs to catch errors early.


