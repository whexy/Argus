use crate::compiler_option::CompilerOption;

pub struct CompilerOptionManager {
    pub options: Vec<CompilerOption>,
}

impl CompilerOptionManager {
    pub fn new(args: Vec<String>) -> Self {
        let options = args
            .iter()
            .map(|arg| CompilerOption::from_arg(arg))
            .collect();
        CompilerOptionManager {
            options,
        }
    }

    pub fn get_command(&self) -> Vec<String> {
        return self
            .options
            .iter()
            .filter(|option| option.is_enabled)
            .map(|option| option.to_string())
            .collect();
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