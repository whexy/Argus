use crate::compiler_option::CompilerOption;

mod default_optimization_visitor;
mod default_parameters_visitor;
mod instrumentation_visitor;
mod libfuzzer_visitor;
mod pass_visitor;
mod profile_visitor;
mod runtime_visitor;
mod sanitizer_visitor;
mod x_visitor;

pub use default_optimization_visitor::DefaultOptimizationVisitor;
pub use default_parameters_visitor::DefaultParametersVisitor;
pub use instrumentation_visitor::InstrumentationVisitor;
pub use libfuzzer_visitor::LibfuzzerVisitor;
pub use pass_visitor::{CMDFuzzVisitor, SanCovPassVisitor, TTRFuzzVisitor, FunctionCallVisitor};
pub use profile_visitor::ProfileVisitor;
pub use runtime_visitor::RuntimeVisitor;
pub use sanitizer_visitor::SanitizerVisitor;
pub use x_visitor::XVisitor;

pub trait OptionVisitor {
    fn visit(&mut self, options: &mut Vec<CompilerOption>);
}
