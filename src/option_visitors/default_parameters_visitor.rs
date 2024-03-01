use super::OptionVisitor;
use crate::compiler_option::CompilerOption;

pub struct DefaultParametersVisitor;

impl OptionVisitor for DefaultParametersVisitor {
    fn visit(&self, options: &mut Vec<CompilerOption>) {
        let default_options = vec![
            CompilerOption::new("-g", true),
            CompilerOption::new("-O3", true),
            CompilerOption::new("-funroll-loops", true),
        ];

        for default_option in default_options {
            if !options.iter().any(|o| o.name == default_option.name) {
                options.push(default_option);
            }
        }
    }
}
