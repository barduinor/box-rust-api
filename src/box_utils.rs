// function to generate random state
use rand::distributions::Alphanumeric;
use rand::Rng;
pub fn generate_state(length: u8) -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(length.into())
        .map(char::from)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_generate_state() {
        let state = generate_state(16);
        println!("{:?}", state);
        assert_eq!(state.len(), 16);
    }
}
