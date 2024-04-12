use crate::compiler_option::CompilerOption;
use crate::option_visitors::OptionVisitor;

pub struct LibfuzzerVisitor;

impl Default for LibfuzzerVisitor {
    fn default() -> Self {
        Self::new()
    }
}

impl LibfuzzerVisitor {
    pub fn new() -> Self {
        LibfuzzerVisitor
    }
}

impl OptionVisitor for LibfuzzerVisitor {
    fn visit(&mut self, options: &mut Vec<CompilerOption>) {
        for option in options.iter_mut() {
            if option.name == "-fsanitize" {
                option.remove_value("fuzz");
            }
        }
    }
}
