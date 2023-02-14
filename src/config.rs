use serde::{Deserialize, Serialize};
use std::{fs::File, io::prelude::*};

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct MailerConfig {
    sender: (String, String),
    smtp_uri: String,
    smtp_port: u16,
    smtp_username: String,
    smtp_password: String,
}

impl MailerConfig {
    pub fn init() -> MailerConfig {
        use std::env;

        let args: Vec<String> = env::args().collect();

        if args.len() < 2 {
            println!("Error. Provide the file path should be provided as an argument to the command. \n\n Example: ` xor-mailer mailer.toml `");

            panic!();
        }

        if args.len() > 2 {
            println!("Error. Multiple arguments provided. Only the file path should be provided as an argument. \n\n Example: ` xor-mailer mailer.toml `");

            panic!();
        }

        let file_path: &str = args[1].as_ref();

        let mut contents = String::new();

        let mut config_file = File::open(&file_path).unwrap();

        config_file.read_to_string(&mut contents).unwrap();

        toml::from_str::<MailerConfig>(&contents).unwrap()
    }

    pub fn sender(&self) -> (&str, &str) {
        (self.sender.0.as_str(), self.sender.1.as_str())
    }

    pub fn smtp_uri(&self) -> &str {
        self.smtp_uri.as_str()
    }

    pub fn smtp_port(&self) -> u16 {
        self.smtp_port
    }

    pub fn smtp_username(&self) -> &str {
        self.smtp_username.as_str()
    }

    pub fn smtp_password(&self) -> &str {
        self.smtp_password.as_str()
    }

    pub fn credentials(&self) -> (&str, &str) {
        (self.smtp_username.as_str(), self.smtp_password.as_str())
    }
}
