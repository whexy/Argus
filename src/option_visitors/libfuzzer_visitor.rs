use std::default;

use crate::compiler_option::{CompilerOption, OptionManagement};
use crate::env::{DRIVER, DRIVER_PASSTHROUGH};
use crate::object::find_object;
use crate::option_visitors::OptionVisitor;

/// Visitor to remove the fuzzer sanitizer from the options. If libFuzzer is used, replace it with the "FUZZER_LIB".
pub struct LibfuzzerVisitor {
    passthrough: bool,
}

impl Default for LibfuzzerVisitor {
    fn default() -> Self {
        Self::new()
    }
}

impl LibfuzzerVisitor {
    pub fn new() -> Self {
        let passthrough = std::env::var(DRIVER_PASSTHROUGH).is_ok();
        LibfuzzerVisitor { passthrough }
    }
}

impl OptionVisitor for LibfuzzerVisitor {
    fn visit(&mut self, options: &mut Vec<CompilerOption>) {
        if self.passthrough {
            return; // leave it as it is
        }

        let mut add_driver = false;

        // Iterate over all sanitizer options and remove the fuzzer sanitizer
        for sanitizer_options in options.get_mut_options("-fsanitize") {
            if sanitizer_options.contains("fuzzer") {
                sanitizer_options.remove_value("fuzzer");
                add_driver = true;
            }
        }

        if !add_driver {
            return;
        }

        let mut nonstd = false;
        for stdlib_options in options.get_options("-stdlib") {
            if stdlib_options.contains("libc++") {
                nonstd = true;
                break;
            }
        }

        let default_driver = if nonstd {
            "bandfuzz-driver-libc++.o"
        } else {
            "bandfuzz-driver.o"
        };

        let driver = std::env::var(DRIVER).unwrap_or(String::from(default_driver));
        if options.is_compiling() || options.is_preprocessor() || options.is_checking() {
            return;
        }
        if let Some(driver_library) = find_object(&driver) {
            options.add_or_modify(&CompilerOption::new(
                driver_library
                    .canonicalize()
                    .unwrap()
                    .to_string_lossy()
                    .as_ref(),
            ));
        } else {
            panic!(
                "Could not find the driver library for the current FUZZER_LIB: {}",
                &driver
            );
        }
    }
}
