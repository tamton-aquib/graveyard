pub mod base;
pub mod caesar;
pub mod general;
pub mod hasher;
pub mod morse;
pub mod xor;

// TODO: add test cases for each module.

#[cfg(test)]
mod tests {
    use super::*;

    use anyhow::Result;
    #[tokio::test]
    async fn check_hasher() -> Result<()> {
        assert_eq!(
            hasher::start_cracker("5f4dcc3b5aa765d61d8327deb882cf99").await?,
            "password"
        );
        Ok(())
    }

    #[test]
    fn check_caesar() {
        assert_eq!(caesar::rot13("nice"), "avpr")
    }
}
