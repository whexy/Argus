use super::OptionVisitor;
use crate::compiler_option::{CompilerOption, OptionManagement};

pub struct SanitizerVisitor {
    use_asan: bool,
    use_msan: bool,
    use_ubsan: bool,
}

impl Default for SanitizerVisitor {
    fn default() -> Self {
        Self::new()
    }
}

impl SanitizerVisitor {
    pub fn new() -> Self {
        SanitizerVisitor {
            use_asan: false,
            use_msan: false,
            use_ubsan: false,
        }
    }

    fn init(&mut self, options: &mut Vec<CompilerOption>) {
        if let Some(sanitizer_options) = options.get_option("-fsanitize") {
            self.use_asan = sanitizer_options.contains("address");
            self.use_msan = sanitizer_options.contains("memory");
            self.use_ubsan = sanitizer_options.contains("undefined");
        }

        // check environment variables
        if std::env::var("AFL_USE_ASAN").is_ok() {
            self.use_asan = true;
        }
        if std::env::var("AFL_USE_MSAN").is_ok() {
            self.use_msan = true;
        }
        if std::env::var("AFL_USE_UBSAN").is_ok() {
            self.use_ubsan = true;
        }
    }
}

fn enable_asan(options: &mut Vec<CompilerOption>) {
    options.add_or_mix(&CompilerOption::from_arg("-fsanitize=address"));
}

fn enable_msan(options: &mut Vec<CompilerOption>) {
    options.add_or_mix(&CompilerOption::from_arg("-fsanitize=memory"));
}

fn enable_ubsan(options: &mut Vec<CompilerOption>) {
    options.add_or_mix(&CompilerOption::from_arg("-fsanitize=undefined"));

    options.add_or_modify(&CompilerOption::from_arg("-fno-sanitize-recover=all"));
    options.add_or_modify(&CompilerOption::from_arg(
        "-fsanitize-undefined-trap-on-error",
    ));
    options.add_or_modify(&CompilerOption::from_arg("-fno-omit-frame-pointer"));
}

impl OptionVisitor for SanitizerVisitor {
    fn visit(&mut self, options: &mut Vec<CompilerOption>) {
        self.init(options);
        if self.use_asan {
            enable_asan(options);
        }
        if self.use_msan {
            enable_msan(options);
        }
        if self.use_ubsan {
            enable_ubsan(options);
        }
    }
}
