# praborrow-macros

Procedural macros for the PraBorrow system.

## Overview

This collection of procedural macros reduces boilerplate and enforces strict semantics within the PraBorrow ecosystem. It includes derivation logic for core traits.

## Key Features

- **`#[derive(Constitution)]`**: Enforces invariant checking on structs, generating `verify_invariants` methods.
- **`#[derive(Target)]`**: Implements resource addressing and identification logic.
- **Compile-Time Checks**: Performs static analysis on macro inputs to catch errors early.

---

# praborrow-macros (Bahasa Indonesia)

Macro prosedural untuk sistem PraBorrow.

## Ikhtisar (Overview)

Kumpulan macro prosedural ini mengurangi boilerplate dan menegakkan semantik yang ketat dalam ekosistem PraBorrow. Ini mencakup logika derivasi untuk trait inti.

## Fitur Utama (Key Features)

- **`#[derive(Constitution)]`**: Menegakkan pemeriksaan invarian pada struct, menghasilkan metode `verify_invariants`.
- **`#[derive(Target)]`**: Mengimplementasikan pengalamatan sumber daya dan logika identifikasi.
- **Pemeriksaan Waktu Kompilasi (Compile-Time Checks)**: Melakukan analisis statis pada input macro untuk menangkap error lebih awal.

