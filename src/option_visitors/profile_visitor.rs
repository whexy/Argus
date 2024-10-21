use crate::compiler_option::{CompilerOption, OptionManagement};

use super::OptionVisitor;

#[derive(Default)]
pub struct ProfileVisitor {}

impl ProfileVisitor {
    pub fn new() -> Self {
        Self::default()
    }
}

impl OptionVisitor for ProfileVisitor {
    fn visit(&mut self, options: &mut Vec<CompilerOption>) {
        options.add_option(&CompilerOption::new("-fprofile-instr-generate"));
        options.add_option(&CompilerOption::new("-fcoverage-mapping"));
    }
}
