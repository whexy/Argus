use crate::compiler_option::CompilerOption;

mod additional_objects_visitor;
mod additional_passes_visitor;
mod default_optimization_visitor;
mod default_parameters_visitor;
mod libfuzzer_visitor;
mod profile_visitor;
mod runtime_visitor;
mod sanitizer_visitor;
mod x_visitor;

pub use additional_objects_visitor::AdditionalObjectsVisitor;
pub use additional_passes_visitor::AdditionalPassesVisitor;
pub use default_optimization_visitor::DefaultOptimizationVisitor;
pub use default_parameters_visitor::DefaultParametersVisitor;
pub use libfuzzer_visitor::LibfuzzerVisitor;
pub use profile_visitor::ProfileVisitor;
pub use runtime_visitor::RuntimeVisitor;
pub use sanitizer_visitor::SanitizerVisitor;
pub use x_visitor::XVisitor;

pub trait OptionVisitor {
    fn visit(&mut self, options: &mut Vec<CompilerOption>);
}
