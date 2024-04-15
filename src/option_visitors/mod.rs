use crate::compiler_option::CompilerOption;

mod default_optimization_visitor;
mod default_parameters_visitor;
mod libfuzzer_visitor;
mod sanitizer_visitor;
mod runtime_visitor;
mod pass_visitor;

pub use default_optimization_visitor::DefaultOptimizationVisitor;
pub use default_parameters_visitor::DefaultParametersVisitor;
pub use libfuzzer_visitor::LibfuzzerVisitor;
pub use sanitizer_visitor::SanitizerVisitor;
pub use runtime_visitor::RuntimeVisitor;
pub use pass_visitor::CMDFuzzVisitor;

pub trait OptionVisitor {
    fn visit(&mut self, options: &mut Vec<CompilerOption>);
}
