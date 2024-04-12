use crate::compiler_option::CompilerOption;

mod sanitizer_visitor;
mod default_optimization_visitor;
mod default_parameters_visitor;
mod libfuzzer_visitor;

pub use sanitizer_visitor::SanitizerVisitor;
pub use default_optimization_visitor::DefaultOptimizationVisitor;
pub use default_parameters_visitor::DefaultParametersVisitor;
pub use libfuzzer_visitor::LibfuzzerVisitor;

pub trait OptionVisitor {
    fn visit(&mut self, options: &mut Vec<CompilerOption>);
}
