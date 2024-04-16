use crate::compiler_option::CompilerOption;

use super::OptionVisitor;

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
    if !options
        .iter()
        .any(|opt| opt.name.starts_with("-O") && opt.is_enabled)
    {
        options.push(CompilerOption::new("-O3"));
        options.push(CompilerOption::new("-funroll-loops"));
    }
}

impl OptionVisitor for DefaultOptimizationVisitor {
    fn visit(&mut self, options: &mut Vec<CompilerOption>) {
        modify_optimization_option(options);
    }
}
