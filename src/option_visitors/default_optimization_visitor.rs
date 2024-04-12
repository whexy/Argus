use super::OptionVisitor;
use crate::compiler_option::{CompilerOption, OptionManagement};

pub struct DefaultOptimizationVisitor;

impl Default for DefaultOptimizationVisitor {
    fn default() -> Self {
        Self::new()
    }
}

impl DefaultOptimizationVisitor {
    pub fn new() -> Self {
        DefaultOptimizationVisitor
    }
}

/// Modify the optimization options. If -Ox is already present, disable it.
fn modify_optimization_option(options: &mut Vec<CompilerOption>) {
    // disable all options that start with -O
    let optimization_options = options
        .iter_mut()
        .filter(|opt| opt.name.starts_with("-O") && opt.is_enabled)
        .collect::<Vec<_>>();
    for opt in optimization_options {
        opt.disable();
    }

    options.push(CompilerOption::from_arg("-O3"));
    options.push(CompilerOption::from_arg("-funroll-loops"));
}

fn modify_debug_option(options: &mut Vec<CompilerOption>) {
    // disable all options that start with -g
    if options.get_option("-g").is_none() {
        options.push(CompilerOption::from_arg("-g"));
    }
}

impl OptionVisitor for DefaultOptimizationVisitor {
    fn visit(&mut self, options: &mut Vec<CompilerOption>) {
        modify_optimization_option(options);
        modify_debug_option(options);
    }
}

#[cfg(test)]
mod tests {
    use crate::option_manager::CompilerOptionManager;

    use super::*;

    #[test]
    fn test_visit() {
        let args = vec![String::from("-O2"), String::from("-g")];
        let mut manager = CompilerOptionManager::new(args);
        let mut visitor = DefaultOptimizationVisitor;
        visitor.visit(&mut manager.options);
        let expected_options = vec!["-g", "-O3", "-funroll-loops"];
        assert_eq!(expected_options, manager.get_command());
    }
}
