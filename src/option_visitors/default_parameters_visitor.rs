use super::OptionVisitor;
use crate::compiler_option::CompilerOption;

pub struct DefaultParametersVisitor;

impl Default for DefaultParametersVisitor {
    fn default() -> Self {
        Self::new()
    }
}

impl DefaultParametersVisitor {
    pub fn new() -> Self {
        DefaultParametersVisitor
    }
}

impl OptionVisitor for DefaultParametersVisitor {
    fn visit(&mut self, options: &mut Vec<CompilerOption>) {
        // do nothing
    }
}
