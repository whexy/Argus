use super::OptionVisitor;
use crate::{
    compiler_option::{CompilerOption, OptionManagement},
    env::RUNTIME,
    object::find_object,
};

#[derive(Default)]
pub struct RuntimeVisitor {}

impl RuntimeVisitor {
    pub fn new() -> Self {
        Self::default()
    }
}

impl OptionVisitor for RuntimeVisitor {
    fn visit(&mut self, options: &mut Vec<CompilerOption>) {
        if options.is_compiling() || options.is_preprocessor() || options.is_checking() {
            return;
        }

        let runtime_path = std::env::var(RUNTIME).unwrap_or_else(|_| "bandfuzz-rt.o".to_string());
        let runtime = find_object(&runtime_path).unwrap_or_else(|| {
            panic!(
                "Could not find runtime object file {}",
                runtime_path.as_str()
            )
        });

        options.add_or_modify(&CompilerOption::new(
            runtime.canonicalize().unwrap().to_string_lossy().as_ref(),
        ));
        options.add_or_mix(&CompilerOption::new("-lpthread"));
        options.add_or_mix(&CompilerOption::new("-ldl"));
        options.add_or_mix(&CompilerOption::new("-lgcc"));
    }
}
