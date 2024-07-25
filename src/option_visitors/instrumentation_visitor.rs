use crate::{
    compiler_option::{CompilerOption, OptionManagement},
    env::FUNCTION_INSTRUMENTATION,
};

use super::OptionVisitor;

pub struct InstrumentationVisitor {
    enabled: bool,
}

impl Default for InstrumentationVisitor {
    fn default() -> Self {
        Self::new()
    }
}

impl InstrumentationVisitor {
    pub fn new() -> Self {
        let enabled = std::env::var(FUNCTION_INSTRUMENTATION).is_ok();
        InstrumentationVisitor { enabled }
    }
}

impl OptionVisitor for InstrumentationVisitor {
    fn visit(&mut self, options: &mut Vec<CompilerOption>) {
        if !self.enabled {
            return;
        }
        options.add_option(&CompilerOption::new("-finstrument-functions"));
    }
}
