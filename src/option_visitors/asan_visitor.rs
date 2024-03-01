use super::OptionVisitor;
use crate::compiler_option::CompilerOption;

pub struct AsanVisitor;

impl OptionVisitor for AsanVisitor {
    fn visit(&self, options: &mut Vec<CompilerOption>) {
        if let Some(sanitize_option) = options.iter_mut().find(|o| o.name == "-fsanitize") {
            sanitize_option.add_or_update_value("address");
        } else {
            options.push(CompilerOption::new("-fsanitize=address", true));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::compiler_option::CompilerOption;
    use crate::option_visitors::OptionVisitor;

    #[test]
    fn test_asan_visitor_adds_address_if_missing() {
        let mut options = vec![];
        let visitor = AsanVisitor;
        visitor.visit(&mut options);

        assert_eq!(options.len(), 1);
        assert_eq!(options[0].name, "-fsanitize");
        assert_eq!(options[0].value, Some("address".to_string()));
    }

    #[test]
    fn test_asan_visitor_updates_existing_fsanitize_without_address() {
        let mut options = vec![CompilerOption::new("-fsanitize=thread", true)];
        let visitor = AsanVisitor;
        visitor.visit(&mut options);

        assert_eq!(options.len(), 1);
        assert_eq!(options[0].name, "-fsanitize");
        assert_eq!(options[0].value, Some("thread,address".to_string()));
    }

    #[test]
    fn test_asan_visitor_does_not_duplicate_address_in_fsanitize() {
        let mut options = vec![CompilerOption::new("-fsanitize=address,thread", true)];
        let visitor = AsanVisitor;
        visitor.visit(&mut options);

        assert_eq!(options.len(), 1);
        assert_eq!(options[0].name, "-fsanitize");
        assert_eq!(options[0].value, Some("address,thread".to_string()));
        // 确认 "address" 只出现一次
        let address_count = options[0].value.as_ref().unwrap().matches("address").count();
        assert_eq!(address_count, 1);
    }
}
