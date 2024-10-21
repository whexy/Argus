use crate::compiler_option::{CompilerOption, OptionManagement};
use crate::env::DRIVER;
use crate::object::find_object;
use crate::option_visitors::OptionVisitor;

/// Visitor to remove the fuzzer sanitizer from the options. If libFuzzer is used, replace it with the "FUZZER_LIB".
#[derive(Default)]
pub struct LibfuzzerVisitor;

impl LibfuzzerVisitor {
    pub fn new() -> Self {
        Self::default()
    }
}

impl OptionVisitor for LibfuzzerVisitor {
    fn visit(&mut self, options: &mut Vec<CompilerOption>) {
        if self.remove_fuzzer_sanitizer(options) {
            self.add_driver_library(options);
        }
    }
}

impl LibfuzzerVisitor {
    /// Removes the fuzzer sanitizer from the options if present.
    /// Returns true if the fuzzer sanitizer was removed, false otherwise.
    fn remove_fuzzer_sanitizer(&self, options: &mut Vec<CompilerOption>) -> bool {
        let mut fuzzer_found = false;

        for sanitizer_options in options.get_mut_options("-fsanitize") {
            if sanitizer_options.contains("fuzzer") {
                sanitizer_options.remove_value("fuzzer");
                fuzzer_found = true;
            }
        }

        fuzzer_found
    }

    /// Adds the appropriate driver library based on the environment and options.
    fn add_driver_library(&self, options: &mut Vec<CompilerOption>) {
        if options.is_compiling() || options.is_preprocessor() || options.is_checking() {
            return;
        }

        let nonstd = options
            .get_options("-stdlib")
            .iter()
            .any(|stdlib_options| stdlib_options.contains("libc++"));

        let default_driver = if nonstd {
            "bandfuzz-driver-libc++.o"
        } else {
            "bandfuzz-driver.o"
        };

        let driver = std::env::var(DRIVER).unwrap_or_else(|_| default_driver.to_string());

        if let Some(driver_library) = find_object(&driver) {
            options.add_or_modify(&CompilerOption::new(
                driver_library
                    .canonicalize()
                    .unwrap()
                    .to_string_lossy()
                    .as_ref(),
            ));

            if !nonstd {
                options.add_or_modify(&CompilerOption::new("-lstdc++"));
            }
        } else {
            panic!(
                "Could not find the driver library for the current FUZZER_LIB: {}",
                driver
            );
        }
    }
}
