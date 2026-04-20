use anyhow::Result;
use rand::distr::Distribution;

const BASE62: &[u8; 62] = b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";

pub fn generate_token() -> Result<String> {
    let distribution = rand::distr::slice::Choose::new(BASE62)?;
    let mut rng = rand::rng();
    Ok(distribution
        .sample_iter(&mut rng)
        .take(10)
        .map(|c| *c as char)
        .collect::<String>())
}

#[cfg(test)]
mod tests {
    use super::generate_token;

    #[test]
    fn test_generate_token() {
        let out1 = generate_token().unwrap();
        let out2 = generate_token().unwrap();
        let out3 = generate_token().unwrap();

        // Sort of weak as far as assertions go, but this is the least we can expect, right?
        assert_ne!(out1, out2, "random tokens should be different");
        assert_ne!(out2, out3, "random tokens should be different");
        assert_ne!(out3, out1, "random tokens should be different");
    }
}
