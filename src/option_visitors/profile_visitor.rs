use crate::{
    compiler_option::{CompilerOption, OptionManagement},
    env::PROFILING,
};

use super::OptionVisitor;

pub struct ProfileVisitor {
    enabled: bool,
}

impl Default for ProfileVisitor {
    fn default() -> Self {
        Self::new()
    }
}

impl ProfileVisitor {
    pub fn new() -> Self {
        let enabled = std::env::var(PROFILING).is_ok();
        ProfileVisitor { enabled }
    }
}

impl OptionVisitor for ProfileVisitor {
    fn visit(&mut self, options: &mut Vec<CompilerOption>) {
        if !self.enabled {
            return;
        }
        options.add_option(&CompilerOption::new("-fprofile-instr-generate"));
        options.add_option(&CompilerOption::new("-fcoverage-mapping"));
    }
}
