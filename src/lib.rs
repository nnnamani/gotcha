use rand::Rng;

pub struct Generator {
    pub use_number: bool,
    pub use_lowercase: bool,
    pub use_uppercase: bool,
    pub use_symbol: bool,
}

impl Generator {
    pub fn new(number: bool, lowercase: bool, uppercase: bool, symbol: bool) -> Generator {
        let mut generator = Generator {
            use_number: true,
            use_lowercase: true,
            use_uppercase: true,
            use_symbol: true,
        };

        if number || lowercase || uppercase || symbol {
            generator.use_number = number;
            generator.use_lowercase = lowercase;
            generator.use_uppercase = uppercase;
            generator.use_symbol = symbol;
        }

        generator
    }

    fn generate_seed_of_numbers() -> Vec<char> {
        "0123456789".chars().collect()
    }

    fn generate_seed_of_lowercase_letters() -> Vec<char> {
        "abcdefghijklmnopqrstuvwxyz".chars().collect()
    }

    fn generate_seed_of_uppercase_letters() -> Vec<char> {
        "ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().collect()
    }

    fn generate_seed_of_symbols() -> Vec<char> {
        "!#$%&'()*+,-./;<=>?@[]^_`{|}~".chars().collect()
    }

    pub fn generate_random_string(&self, length: u32) -> Vec<char> {
        let mut seed: Vec<char> = Vec::new();

        if self.use_number {
            seed.extend(Generator::generate_seed_of_numbers());
        }

        if self.use_lowercase {
            seed.extend(Generator::generate_seed_of_lowercase_letters());
        }

        if self.use_uppercase {
            seed.extend(Generator::generate_seed_of_uppercase_letters());
        }

        if self.use_symbol {
            seed.extend(Generator::generate_seed_of_symbols());
        }

        let seed_size = seed.len();
        let mut rng = rand::thread_rng();
        let mut result: Vec<char> = Vec::new();

        for _ in 0..length {
            result.push(seed[rng.gen_range(0, seed_size)]);
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use regex::Regex;

    #[test]
    fn test_random_string_include_numbers_only() {
        let re = Regex::new(r"^\d+$").unwrap();
        let generator = Generator::new(true, false, false, false);

        let result: String = generator.generate_random_string(10).iter().collect();
        assert!(re.is_match(&result));
    }

    #[test]
    fn test_random_string_include_lowercase_only() {
        let re = Regex::new(r"^[a-z]+$").unwrap();
        let generator = Generator::new(false, true, false, false);

        let result: String = generator.generate_random_string(10).iter().collect();
        assert!(re.is_match(&result));
    }

    #[test]
    fn test_random_string_include_uppercase_only() {
        let re = Regex::new(r"^[A-Z]+$").unwrap();
        let generator = Generator::new(false, false, true, false);

        let result: String = generator.generate_random_string(10).iter().collect();
        assert!(re.is_match(&result));
    }

    #[test]
    fn test_random_string_include_symbol_only() {
        let re = Regex::new(r"^[!#\$%&'\(\)*+,-./;<=>?@\[\]\^_`\{\|\}~]+$").unwrap();
        let generator = Generator::new(false, false, false, true);

        let result: String = generator.generate_random_string(10).iter().collect();
        assert!(re.is_match(&result));
    }

    #[test]
    fn test_random_string_include_lowercase_and_numbers() {
        let re = Regex::new(r"^[0-9a-z]+$").unwrap();
        let generator = Generator::new(true, true, false, false);

        let result: String = generator.generate_random_string(10).iter().collect();
        assert!(re.is_match(&result));
    }
}
