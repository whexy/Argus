use crate::compiler_option::{CompilerOption, OptionManagement};
use crate::object::find_object;
use crate::option_visitors::OptionVisitor;

/// Visitor to remove the fuzzer sanitizer from the options. If libFuzzer is used, replace it with the "FUZZER_LIB".
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
        if let Some(sanitizer_options) = options.get_mut_option("-fsanitize") {
            if sanitizer_options.contains("fuzzer") {
                sanitizer_options.remove_value("fuzzer");

                // replace libFuzzer with the "FUZZER_LIB"
                let fuzzer_lib =
                    std::env::var("FUZZER_LIB").unwrap_or(String::from("bf-llvm_mode.o"));
                if let Some(runtime_library) = find_object(&fuzzer_lib) {
                    options.add_or_modify(&CompilerOption::new(
                        runtime_library.canonicalize().unwrap().to_string_lossy().as_ref(),
                    ));
                } else {
                    panic!(
                        "Could not find the runtime library for the current FUZZER_LIB: {}",
                        fuzzer_lib
                    );
                }
            }
        }
    }
}
