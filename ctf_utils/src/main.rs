use clap::Parser;

use ctf_utils::caesar;
use ctf_utils::morse;
// TODO: add colors
// TODO: complete xor.rs
// TODO: start general.rs
// TODO: complete caesar::vigenere()

#[derive(Parser, Debug)]
enum Cli {
    /// Caesar decryption
    Caesar { query: String },
    /// Vigenere decryption
    Vigenere { query: String, key: String },
    /// Rot13 decryption
    Rot13 { query: String },

    /// Morse decryption
    Morse { query: String },
}

fn main() {
    match Cli::parse() {
        Cli::Caesar { query } => {
            println!("Trying caesar:\n {}", caesar::caesar(&query).join("\n"));
        }
        Cli::Vigenere { query, key } => {
            println!("Trying vigenere: {}", caesar::vigenere(&query, &key));
        }
        Cli::Rot13 { query } => {
            println!("Trying rot13: {}", caesar::rot13(&query));
        }
        Cli::Morse { query } => {
            println!("Trying morse: {}", morse::morse_decode(&query));
        }
    }
}

// COMPLETED:
// 1. caesar.rs: rot13, caesar, vigenere
// 2. morse.rs: morse_encode, morse_decode
// 3. xor.rs: str, hex, byte, etc
