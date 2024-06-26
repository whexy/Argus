use super::OptionVisitor;
use crate::{
    compiler_option::{CompilerOption, OptionManagement},
    env::{ENABLE_ASAN, ENABLE_COVSAN, ENABLE_MSAN, ENABLE_UBSAN, NOSANITIZER},
};

pub struct SanitizerVisitor {
    use_asan: bool,
    use_msan: bool,
    use_ubsan: bool,
    use_cov: bool,
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
            use_cov: false,
        }
    }

    fn init(&mut self, options: &mut Vec<CompilerOption>) {
        for sanitizer_options in options.get_options("-fsanitize") {
            self.use_asan |= sanitizer_options.contains("address");
            self.use_msan |= sanitizer_options.contains("memory");
            self.use_ubsan |= sanitizer_options.contains("undefined");
        }

        // check environment variables
        if std::env::var(NOSANITIZER).is_ok() {
            self.use_asan = false;
            self.use_msan = false;
            self.use_ubsan = false;
        }
        if std::env::var(ENABLE_ASAN).is_ok() {
            self.use_asan = true;
        }
        if std::env::var(ENABLE_MSAN).is_ok() {
            self.use_msan = true;
        }
        if std::env::var(ENABLE_UBSAN).is_ok() {
            self.use_ubsan = true;
        }
        if std::env::var(ENABLE_COVSAN).is_ok() {
            self.use_cov = true;
        }
    }
}

fn enable_asan(options: &mut Vec<CompilerOption>) {
    options.add_or_mix(&CompilerOption::from_arg("-fsanitize=address"));
}

fn disable_asan(options: &mut Vec<CompilerOption>) {
    for sanitizer_option in options.get_mut_options("-fsanitize") {
        sanitizer_option.remove_value("address");
    }
}

fn enable_msan(options: &mut Vec<CompilerOption>) {
    options.add_or_mix(&CompilerOption::from_arg("-fsanitize=memory"));
}

fn disable_msan(options: &mut Vec<CompilerOption>) {
    for sanitizer_option in options.get_mut_options("-fsanitize") {
        sanitizer_option.remove_value("memory");
    }
}

fn enable_ubsan(options: &mut Vec<CompilerOption>) {
    options.add_or_mix(&CompilerOption::from_arg("-fsanitize=undefined"));

    options.add_or_modify(&CompilerOption::from_arg("-fno-sanitize-recover=all"));
    options.add_or_modify(&CompilerOption::from_arg(
        "-fsanitize-undefined-trap-on-error",
    ));
    options.add_or_modify(&CompilerOption::from_arg("-fno-omit-frame-pointer"));
}

fn disable_ubsan(options: &mut Vec<CompilerOption>) {
    for sanitizer_option in options.get_mut_options("-fsanitize") {
        sanitizer_option.remove_value("undefined");
    }
}

fn enable_cov(options: &mut Vec<CompilerOption>) {
    options.add_or_modify(&CompilerOption::from_arg(
        "-fsanitize-coverage=trace-pc-guard",
    ));
}

fn disable_cov(options: &mut Vec<CompilerOption>) {
    for sanitizer_coverage_option in options.get_mut_options("-fsanitize-coverage") {
        sanitizer_coverage_option.disable();
    }
}

impl OptionVisitor for SanitizerVisitor {
    fn visit(&mut self, options: &mut Vec<CompilerOption>) {
        self.init(options);
        options.add_or_modify(&CompilerOption::from_arg("-U_FORTIFY_SOURCE"));
        if self.use_asan {
            enable_asan(options);
        } else {
            disable_asan(options);
        }

        if self.use_msan {
            enable_msan(options);
        } else {
            disable_msan(options);
        }

        if self.use_ubsan {
            enable_ubsan(options);
        } else {
            disable_ubsan(options);
        }

        if self.use_cov {
            enable_cov(options);
        } else {
            disable_cov(options);
        }
    }
}
