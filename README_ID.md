# praborrow-macros (Bahasa Indonesia)

[English](./README.md) | Indonesia

Macro prosedural untuk sistem PraBorrow.

## Ikhtisar (Overview)

Kumpulan macro prosedural ini mengurangi boilerplate dan menegakkan semantik yang ketat dalam ekosistem PraBorrow. Ini mencakup logika derivasi untuk trait inti.

## Fitur Utama (Key Features)

- **`#[derive(Constitution)]`**: Menegakkan pemeriksaan invarian pada struct, menghasilkan metode `verify_invariants`.
- **`#[derive(Target)]`**: Mengimplementasikan pengalamatan sumber daya dan logika identifikasi.
- **Pemeriksaan Waktu Kompilasi (Compile-Time Checks)**: Melakukan analisis statis pada input macro untuk menangkap error lebih awal.
