use std::path::PathBuf;

use super::OptionVisitor;
use crate::{
    compiler_option::{CompilerOption, OptionManagement},
    object::find_object,
};

pub struct RuntimeVisitor {
    use_runtime: bool,
    runtime: PathBuf,
}

impl Default for RuntimeVisitor {
    fn default() -> Self {
        Self::new()
    }
}

impl RuntimeVisitor {
    pub fn new() -> Self {
        let use_runtime = std::env::var("BANDFUZZ_NORUNTIME").is_err();
        let runtime_path =
            std::env::var("BANDFUZZ_RUNTIME").unwrap_or_else(|_| "bf-rt.o".to_string());
        let runtime = find_object(&runtime_path).expect(&format!(
            "Could not find runtime object file {}",
            runtime_path.as_str()
        ));
        RuntimeVisitor {
            use_runtime,
            runtime,
        }
    }
}

impl OptionVisitor for RuntimeVisitor {
    fn visit(&mut self, options: &mut Vec<CompilerOption>) {
        if !self.use_runtime {
            return;
        }
        if options.is_compiling() || options.is_preprocessor() || options.is_checking() {
            return;
        }
        options.add_or_modify(&&CompilerOption::new(
            self.runtime
                .canonicalize()
                .unwrap()
                .to_string_lossy()
                .as_ref(),
        ));
    }
}
