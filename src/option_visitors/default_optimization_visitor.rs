use super::OptionVisitor;
use crate::compiler_option::CompilerOption;

pub struct DefaultOptimizationVisitor;

impl OptionVisitor for DefaultOptimizationVisitor {
    fn visit(&self, options: &mut Vec<CompilerOption>) {
        let required_options = vec!["-g", "-O3", "-funroll-loops"];
        let existing_options = {
            options
                .iter()
                .map(|opt| opt.name.as_str())
                .collect::<Vec<_>>()
        };

        let missing_options: Vec<&str> = required_options
            .iter()
            .filter(|&&opt| !existing_options.contains(&opt))
            .cloned()
            .collect();

        for opt in missing_options {
            options.push(CompilerOption::new(opt, true));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_visit() {
        let mut options = vec![
            CompilerOption::new("-g", true),
            CompilerOption::new("-O2", true),
        ];
        let visitor = DefaultOptimizationVisitor;
        visitor.visit(&mut options);

        let expected_options = vec!["-g", "-O2", "-O3", "-funroll-loops"];

        let actual_options: Vec<_> = options.iter().map(|opt| opt.name.as_str()).collect();

        assert_eq!(expected_options, actual_options);
    }
}
