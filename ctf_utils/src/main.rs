use anyhow::Result;
// use ctf_utils::caesar;
use ctf_utils::base;
// TODO: add colors
// TODO: complete xor.rs
// TODO: start general.rs
// TODO: complete caesar::vigenere()
// TODO: start on bases: 2(binary),8(octal),16(hex),32,64

#[tokio::main]
async fn main() -> Result<()> {
    println!("{}", base::b32("bruh"));
    Ok(())
}

// COMPLETED:
// 1. caesar.rs: rot13, caesar, vigenere
// 2. hasher.rs
// 3. morse.rs: morse_encode, morse_decode
// 4. xor.rs: str, hex, byte, etc
