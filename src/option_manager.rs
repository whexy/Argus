use crate::compiler_option::CompilerOption;
use crate::option_visitors::OptionVisitor;

pub struct CompilerOptionManager {
    pub options: Vec<CompilerOption>,
}

impl CompilerOptionManager {
    pub fn new() -> Self {
        CompilerOptionManager {
            options: Vec::new(),
        }
    }

    pub fn apply_visitor(&mut self, visitor: &dyn OptionVisitor) {
        visitor.visit(&mut self.options);
    }

    pub fn add_option(&mut self, name: String, is_enabled: bool) {
        self.options.push(CompilerOption::new(&name, is_enabled));
    }

    pub fn print_command(&self) {
        let command = self
            .options
            .iter()
            .fold(String::from("clang"), |acc, option| {
                acc + " " + &option.to_string()
            });
        println!("{}", command);
    }
}
