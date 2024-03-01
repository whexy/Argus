use crate::compiler_option::CompilerOption;

mod asan_visitor;
mod default_optimization_visitor;
mod default_parameters_visitor;
mod libfuzzer_visitor;

pub use asan_visitor::AsanVisitor;
pub use default_optimization_visitor::DefaultOptimizationVisitor;
pub use default_parameters_visitor::DefaultParametersVisitor;
pub use libfuzzer_visitor::LibfuzzerVisitor;

pub trait OptionVisitor {
    fn visit(&self, options: &mut Vec<CompilerOption>);
}
