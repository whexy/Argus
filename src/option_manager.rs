use std::{
    fs::File,
    io::{self, BufReader, Read},
    path::PathBuf,
};

use crate::compiler_option::CompilerOption;

pub struct CompilerOptionManager {
    pub options: Vec<CompilerOption>,
}

impl CompilerOptionManager {
    pub fn new(args: Vec<String>) -> Self {
        let options = args
            .iter()
            .flat_map(|arg| {
                if let Some(response_file_path) = arg.strip_prefix('@') {
                    match read_response_file(PathBuf::from(response_file_path)) {
                        Ok(response_options) => response_options,
                        Err(_) => vec![],
                    }
                } else {
                    vec![CompilerOption::from_arg(arg)]
                }
            })
            .collect();
        CompilerOptionManager { options }
    }

    pub fn get_command(&self) -> Vec<String> {
        return self
            .options
            .iter()
            .filter(|option| option.is_enabled)
            .map(|option| option.to_string())
            .collect();
    }

    pub fn cleanup(&mut self) {
        self.options.retain(|option| option.is_enabled);
    }
}

// implement the Display trait for CompilerOptionManager
impl std::fmt::Display for CompilerOptionManager {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let command = self.get_command();
        write!(f, "{}", command.join(" "))
    }
}

pub fn check_if_option_exists(options: &[CompilerOption], name: &str) -> bool {
    options.iter().any(|o| o.name == name)
}

fn read_response_file(path: PathBuf) -> io::Result<Vec<CompilerOption>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut args = Vec::new();
    let mut current_arg = String::new();
    let mut in_double_quote = false;
    let mut in_single_quote = false;
    let mut escape = false;

    for byte in reader.bytes() {
        let c = byte? as char;

        if escape {
            current_arg.push(c);
            escape = false;
        } else {
            match c {
                '\\' => escape = true,
                '"' if !in_single_quote => in_double_quote = !in_double_quote,
                '\'' if !in_double_quote => in_single_quote = !in_single_quote,
                ' ' | '\n' | '\t' | '\r' if !in_double_quote && !in_single_quote => {
                    if !current_arg.is_empty() {
                        args.push(CompilerOption::from_arg(&current_arg));
                        current_arg.clear();
                    }
                }
                _ => current_arg.push(c),
            }
        }
    }

    if !current_arg.is_empty() {
        args.push(CompilerOption::from_arg(&current_arg));
    }

    Ok(args)
}
