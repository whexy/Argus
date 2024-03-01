use crate::compiler_option::CompilerOption;
use crate::option_visitors::OptionVisitor;

pub struct LibfuzzerVisitor;

impl OptionVisitor for LibfuzzerVisitor {
    fn visit(&self, options: &mut Vec<CompilerOption>) {
        for option in options.iter_mut() {
            if option.name == "-fsanitize" {
                option.remove_value("fuzz", true);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::compiler_option::CompilerOption;
    use crate::option_visitors::OptionVisitor;

    #[test]
    fn test_remove_fuzz_from_sanitize() {
        let mut options = vec![CompilerOption::new(
            "-fsanitize=address,fuzz,undefined",
            true,
        )];
        let visitor = LibfuzzerVisitor;
        visitor.visit(&mut options);

        assert_eq!(options[0].value.as_deref(), Some("address,undefined"));
    }

    #[test]
    fn test_disable_sanitize_when_fuzz_is_only_value() {
        let mut options = vec![CompilerOption::new("-fsanitize=fuzz", true)];
        let visitor = LibfuzzerVisitor;
        visitor.visit(&mut options);

        assert!(!options[0].is_enabled);
    }

    #[test]
    fn test_unaffected_sanitize_option() {
        let mut options = vec![CompilerOption::new("-fsanitize=address,undefined", true)];
        let visitor = LibfuzzerVisitor;
        visitor.visit(&mut options);

        assert_eq!(options[0].value.as_deref(), Some("address,undefined"));
    }
}
