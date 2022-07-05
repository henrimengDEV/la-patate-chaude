#[cfg(test)]
mod tests {
    use crate::hash_cash::HashCash;
    use shared::md5_hash_cash_input::MD5HashCashInput;

    #[test]
    fn test_get_complexity_pattern() {
        let mut input = HashCash::new(MD5HashCashInput{
            complexity: 2,
            message: String::from("hello")
        });

        let result = HashCash::get_complexity_pattern(&mut input);
        assert_eq!(result, "00");
    }

    #[test]
    fn test_convert_hashcode_to_binary() {
        let mut input = HashCash::new(MD5HashCashInput{
            complexity: 9,
            message: String::from("hello")
        });
        let seed: String = input.seed();
        let hashcode: String = input.md5(seed);
        println!("hashcode ---> {}", hashcode);

        let result = HashCash::convert_hashcode_to_binary(&mut input, &hashcode);
        assert_eq!(result,"01101101100110001011010010011001010000101010110000110100111011110000010011101000000111011000101111101001101101010101000010110010");
    }

    #[test]
    fn test_seed() {
        let mut input = HashCash::new(MD5HashCashInput{
            complexity: 2,
            message: String::from("hello")
        });

        let result = HashCash::seed(&mut input);
        assert_eq!(result,"0000000000000000hello");
    }

    #[test]
    fn test_hex_to_binary() {
        let mut input = HashCash::new(MD5HashCashInput{
            complexity: 9,
            message: String::from("hello")
        });

        let result = HashCash::hex_to_binary(&mut input, 'F');
        assert_eq!(result, "1111");
    }

    #[test]
    fn test_md5() {
        let mut input = HashCash::new(MD5HashCashInput{
            complexity: 9,
            message: String::from("hello")
        });
        let seed = input.seed();

        let result = HashCash::md5(&mut input, seed);
        assert_eq!(result,"6d98b49942ac34ef04e81d8be9b550b2");
    }
}