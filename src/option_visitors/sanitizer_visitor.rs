use super::OptionVisitor;
use crate::{
    compiler_option::{CompilerOption, OptionManagement},
    env::{ENABLE_ASAN, ENABLE_COVSAN, ENABLE_MSAN, ENABLE_UBSAN, NOSANITIZER},
};

#[derive(Default)]
pub struct SanitizerVisitor {
    use_asan: bool,
    use_msan: bool,
    use_ubsan: bool,
    use_cov: bool,
}

impl SanitizerVisitor {
    pub fn new() -> Self {
        Self::default()
    }

    fn init(&mut self, options: &mut Vec<CompilerOption>) {
        self.set_sanitizers_from_options(options);
        self.override_sanitizers_from_env();
    }

    fn set_sanitizers_from_options(&mut self, options: &mut Vec<CompilerOption>) {
        for sanitizer_options in options.get_options("-fsanitize") {
            self.use_asan |= sanitizer_options.contains("address");
            self.use_msan |= sanitizer_options.contains("memory");
            self.use_ubsan |= sanitizer_options.contains("undefined");
            self.use_cov |= sanitizer_options.contains("trace-pc-guard");
        }
    }

    fn override_sanitizers_from_env(&mut self) {
        if std::env::var(NOSANITIZER).is_ok() {
            self.use_asan = false;
            self.use_msan = false;
            self.use_ubsan = false;
        }

        self.use_asan |= std::env::var(ENABLE_ASAN).is_ok();
        self.use_msan |= std::env::var(ENABLE_MSAN).is_ok();
        self.use_ubsan |= std::env::var(ENABLE_UBSAN).is_ok();
        self.use_cov |= std::env::var(ENABLE_COVSAN).is_ok();
    }
}

fn toggle_sanitizer(
    options: &mut Vec<CompilerOption>,
    enable: bool,
    flag: &str,
    additional_flags: &[&str],
) {
    if enable {
        options.add_or_mix(&CompilerOption::from_arg(flag));
        for &additional_flag in additional_flags {
            options.add_or_mix(&CompilerOption::from_arg(additional_flag));
        }
    } else {
        for sanitizer_option in options.get_mut_options("-fsanitize") {
            sanitizer_option.remove_value(flag.split('=').last().unwrap());
        }
    }
}

impl OptionVisitor for SanitizerVisitor {
    fn visit(&mut self, options: &mut Vec<CompilerOption>) {
        self.init(options);

        toggle_sanitizer(
            options,
            self.use_asan,
            "-fsanitize=address",
            &["-U_FORTIFY_SOURCE"],
        );
        toggle_sanitizer(options, self.use_msan, "-fsanitize=memory", &[]);
        toggle_sanitizer(
            options,
            self.use_ubsan,
            "-fsanitize=undefined",
            &[
                "-fno-sanitize-recover=all",
                "-fsanitize-undefined-trap-on-error",
                "-fno-omit-frame-pointer",
            ],
        );
        toggle_sanitizer(
            options,
            self.use_cov,
            "-fsanitize-coverage=trace-pc-guard",
            &[],
        );
    }
}
