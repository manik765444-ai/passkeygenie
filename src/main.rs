use rand::Rng;
use std::error::Error;
use std::fmt;

// Define a custom error type
#[derive(Debug)]
enum PasswordGeneratorError {
    InvalidLength,
    InvalidCharacterSet,
}

impl fmt::Display for PasswordGeneratorError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            PasswordGeneratorError::InvalidLength => write!(f, "Password length must be greater than 0"),
            PasswordGeneratorError::InvalidCharacterSet => write!(f, "Invalid character set"),
        }
    }
}

impl Error for PasswordGeneratorError {}

// Define a struct to hold the password generator configuration
struct PasswordGeneratorConfig {
    length: usize,
    use_uppercase: bool,
    use_numbers: bool,
    use_special_chars: bool,
}

impl PasswordGeneratorConfig {
    // Create a new password generator configuration
    fn new(length: usize, use_uppercase: bool, use_numbers: bool, use_special_chars: bool) -> Result<Self, PasswordGeneratorError> {
        if length == 0 {
            return Err(PasswordGeneratorError::InvalidLength);
        }
        Ok(PasswordGeneratorConfig {
            length,
            use_uppercase,
            use_numbers,
            use_special_chars,
        })
    }
}

// Define a password generator struct
struct PasswordGenerator {
    config: PasswordGeneratorConfig,
}

impl PasswordGenerator {
    // Create a new password generator
    fn new(config: PasswordGeneratorConfig) -> Self {
        PasswordGenerator { config }
    }

    // Generate a password
    fn generate_password(&self) -> String {
        let mut password = String::new();
        let mut rng = rand::thread_rng();

        // Define the character sets to use
        let mut char_sets = vec!['a' as u8..='z' as u8];
        if self.config.use_uppercase {
            char_sets.push('A' as u8..='Z' as u8);
        }
        if self.config.use_numbers {
            char_sets.push('0' as u8..='9' as u8);
        }
        if self.config.use_special_chars {
            char_sets.push('!' as u8..='/' as u8);
            char_sets.push(':' as u8..='@' as u8);
            char_sets.push('[' as u8..='`' as u8);
            char_sets.push('{' as u8..='~' as u8);
        }

        // Generate the password
        for _ in 0..self.config.length {
            let char_set = rng.gen_range(0..char_sets.len());
            let char_range = char_sets[char_set].clone();
            let char = (char_range.start + rng.gen_range(0..char_range.end - char_range.start + 1)) as char;
            password.push(char);
        }

        password
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    // Create a new password generator configuration
    let config = PasswordGeneratorConfig::new(12, true, true, true)?;

    // Create a new password generator
    let generator = PasswordGenerator::new(config);

    // Generate a password
    let password = generator.generate_password();

    println!("Generated password: {}", password);

    Ok(())
}