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

    pub fn run(&mut self) {
        loop {
            println!("RUN -----------------------------------------------------------------------");
            println!("message: {}", self.input.message);
            println!("complexity: {}", self.input.complexity);
            println!("complexity (pattern): {}", &self.get_complexity_pattern());
            println!("counter: {}", self.counter);
            println!("seed: {}", String::from(format!("{:016x}", self.counter).to_uppercase() + self.input.message.as_str()));

            let hashcode = self.md5();
            let hashcode_as_binary = self.convert_hashcode_to_binary(&hashcode);
            self.output.hashcode = hashcode;
            self.is_valid = hashcode_as_binary.starts_with(&self.get_complexity_pattern());

            println!("md5 (hex): {}", &self.output.hashcode);
            println!("md5 (binary): {}", &hashcode_as_binary);
            println!("md5 (starts with): {}", &self.is_valid);

            if self.is_valid {
                print!("\nHashCode : {}", self.output.hashcode);
                println!("\nEND -----------------------------------------------------------------------");
                break;
            }

            println!("\nEND -----------------------------------------------------------------------");
            self.counter += 1;
            break;
        }
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

    fn md5(&mut self) -> String {
        let output_cmd = Command::new("md5")
            .args(["-qs", self.seed().as_str()])
            .output()
            .expect("failed to execute process");

        String::from_utf8(output_cmd.stdout).unwrap().replace("\n", "")
    }
}