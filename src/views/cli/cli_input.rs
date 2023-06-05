use std::io::{self, Write};

pub struct CliInput {}

pub struct CliInputSchema {
    description: Option<String>,
    prefix: String,
}

pub enum CliInputResult {
    String(String),
}

impl CliInput {
    pub fn read(schema: CliInputSchema) -> CliInputResult {
        let mut input = String::new();

        Self::print_prompt_description(&schema);
        Self::print_prompt_prefix(&schema);

        io::stdin().read_line(&mut input).unwrap();

        CliInputResult::String(input)
    }

    fn print_prompt_prefix(schema: &CliInputSchema) {
        print!("{}", schema.prefix());
        io::stdout().flush().unwrap();
    }

    fn print_prompt_description(schema: &CliInputSchema) {
        println!("{}", schema.description().unwrap_or_default());
    }
}

impl CliInputSchema {
    pub fn new() -> Self {
        CliInputSchema {
            prefix: String::from("Mailman> "),
            description: None,
        }
    }

    pub fn prefix(&self) -> &str {
        &self.prefix
    }

    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    pub fn add_description(mut self, description: &str) -> Self {
        self.description = Some(description.to_owned());

        self
    }
}
