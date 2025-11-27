use anyhow::Result;
use rand::distributions::{
    Distribution,
    Uniform,
};

const BASE62: &[u8; 62] = b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";

pub fn generate_token() -> Result<String> {
    let between = Uniform::from(0..BASE62.len());
    let mut rng = rand::thread_rng();
    let mut bytes = [0_u8; 27];
    for b in &mut bytes {
        *b = BASE62[between.sample(&mut rng)];
    }
    Ok(String::from_utf8(bytes.to_vec())?)
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
        assert_ne!(
            out1, out2,
            "random tokens should be different"
        );
        assert_ne!(
            out2, out3,
            "random tokens should be different"
        );
        assert_ne!(
            out3, out1,
            "random tokens should be different"
        );
    }
}
