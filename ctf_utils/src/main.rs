use anyhow::Result;
use ctf_utils::morse;

// TODO: add colors
// TODO: complete xor.rs
// TODO: start general.rs
// TODO: complete caesar::vigenere()
// TODO: start on bases: 2(binary),8(octal),16(hex),32,64

#[tokio::main]
async fn main() -> Result<()> {
    println!("{}", morse::morse_encode("a"));
    Ok(())
}
