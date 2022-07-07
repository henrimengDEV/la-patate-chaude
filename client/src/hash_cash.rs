use std::{env, fs};
use std::fs::File;
use std::io::Write;
use std::process::Command;
use shared::md5_hash_cash_input::MD5HashCashInput;
use shared::md5_hash_cash_output::MD5HashCashOutput;

pub struct HashCash {
    pub input: MD5HashCashInput,
    pub output: MD5HashCashOutput,
    counter: u64,
    is_valid: bool,
}

impl HashCash {
    pub fn new(input: MD5HashCashInput) -> HashCash {
        HashCash {
            input,
            output: MD5HashCashOutput { seed: 0, hashcode: String::from("") },
            counter: 0,
            is_valid: false,
        }
    }


    pub fn run(&mut self, log: bool) {
        println!("\n\tHASH-CASH_START -------------------------------------------\n");
        println!("\t+ message: {}", self.input.message);
        println!("\t+ complexity: {}", self.input.complexity);
        println!("\t+ complexity (pattern): {}", &self.get_complexity_pattern());
        loop {
            let seed: String = self.seed();
            let hashcode: String = self.md5(seed);
            let hashcode_as_binary: String = self.convert_hashcode_to_binary(&hashcode);
            self.is_valid = hashcode_as_binary.starts_with(&self.get_complexity_pattern());

            if log {
                println!("\t+ counter: {}", self.counter);
                println!("\t+ seed: {}", String::from(format!("{:016x}", self.counter).to_uppercase() + self.input.message.as_str()));
                println!("\t+ md5 (hex): {}", &self.output.hashcode);
                println!("\t+ md5 (binary): {}", &hashcode_as_binary);
                println!("\t+ md5 (starts with): {}", &self.is_valid);
            }

            if self.is_valid {
                self.output.hashcode = hashcode;
                self.output.seed = self.counter;
                println!("\n\t=> HashCode : {}", self.output.hashcode);
                break;
            }

            self.counter += 1;
        }
        println!("\n\tHASH-CASH_END --------------------------------------------\n");


    }

    fn get_complexity_pattern(&mut self) -> String {
        "0".repeat(self.input.complexity as usize)
    }

    fn convert_hashcode_to_binary(&mut self, hashcode: &str) -> String {
        hashcode.chars()
            .into_iter()
            .map(|x| { self.hex_to_binary(x) })
            .collect::<String>()
    }

    fn seed(&mut self) -> String {
        format!("{:016x}", &self.counter).to_uppercase() + &self.input.message
    }

    fn hex_to_binary(&mut self, c: char) -> String {
        let result = match c.to_ascii_uppercase() {
            '0' => "0000",
            '1' => "0001",
            '2' => "0010",
            '3' => "0011",
            '4' => "0100",
            '5' => "0101",
            '6' => "0110",
            '7' => "0111",
            '8' => "1000",
            '9' => "1001",
            'A' => "1010",
            'B' => "1011",
            'C' => "1100",
            'D' => "1101",
            'E' => "1110",
            'F' => "1111",
            _ => "",
        };

        result.to_string()
    }

    fn md5(&mut self, seed: String) -> String {
        let env = env::consts::OS;
        if env == "windows" {
            let mut file = File::create("md5.txt").expect("Cannot create file");
            // println!("{:?}", file.path());
            file.write_all(seed.as_bytes()).expect("Cannot write in file");
            let output_cmd = Command::new("certutil")
                .arg(" -hashfile")
                .arg(".\\md5.txt")
                .arg(" MD5")
                .output()
                .expect("failed to execute process");
            eprintln!("output_cmd = {:?}", output_cmd);

            fs::remove_file("md5.txt").expect("File delete failed");

            String::from_utf8(output_cmd.stdout).expect("NOT FIND TA MERE")

        } else {
            let output_cmd = Command::new("md5")
                .args(["-qs", seed.as_str()])
                .output()
                .expect("Failed to execute process");
            String::from_utf8(output_cmd.stdout).unwrap().replace("\n", "")
        }


    }
}

#[cfg(test)]
mod tests {
    use crate::hash_cash::HashCash;
    use shared::md5_hash_cash_input::MD5HashCashInput;
    use shared::md5_hash_cash_output::MD5HashCashOutput;

    #[test]
    fn test_hashcash() {
        let mut input = HashCash::new(MD5HashCashInput {
            complexity:4,
            message:String::from("hello")
        });
        input.run(true);
        let result = input.output.hashcode;

        assert_eq!(result, "");
    }

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

