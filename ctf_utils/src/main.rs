use anyhow::Result;
// use ctf_utils::caesar;
use ctf_utils::caesar;
// TODO: add colors
// TODO: complete xor.rs
// TODO: start general.rs
// TODO: complete caesar::vigenere()

#[tokio::main]
async fn main() -> Result<()> {
    println!("{}", caesar::vigenere("bruh", "nice"));
    Ok(())
}

// COMPLETED:
// 1. caesar.rs: rot13, caesar, vigenere
// 2. hasher.rs
// 3. morse.rs: morse_encode, morse_decode
// 4. xor.rs: str, hex, byte, etc
