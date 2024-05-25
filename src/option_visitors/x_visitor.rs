use crate::compiler_option::{CompilerOption, OptionManagement};

use super::OptionVisitor;

pub struct XVisitor {}

impl Default for XVisitor {
    fn default() -> Self {
        Self::new()
    }
}

impl XVisitor {
    pub fn new() -> Self {
        XVisitor {}
    }
}

impl OptionVisitor for XVisitor {
    fn visit(&mut self, options: &mut Vec<CompilerOption>) {
        // if options contains -x, then apply this visitor
        if !options.get_options("-x").is_empty() {
            options.add_option(&CompilerOption::from_arg("-x"));
            options.add_option(&CompilerOption::from_arg("none"));
        }
    }
}
