use super::OptionVisitor;
use crate::{
    compiler_option::{CompilerOption, OptionManagement},
    llvm,
};

pub struct DefaultParametersVisitor;

impl Default for DefaultParametersVisitor {
    fn default() -> Self {
        Self::new()
    }
}

impl DefaultParametersVisitor {
    pub fn new() -> Self {
        DefaultParametersVisitor
    }
}

fn add_llvm_lib(options: &mut Vec<CompilerOption>) {
    let llvm_dir = llvm::get_llvm_libdir().expect("Could not find LLVM lib directory");
    // check if llvm_dir is in /usr or /lib
    if !llvm_dir.starts_with("/usr") && !llvm_dir.starts_with("/lib") {
        options.push(CompilerOption::from_arg(&format!(
            "-Wl,-rpath={}",
            llvm_dir.to_string_lossy()
        )));
    }
}

impl OptionVisitor for DefaultParametersVisitor {
    fn visit(&mut self, options: &mut Vec<CompilerOption>) {
        options.add_or_modify(&CompilerOption::new("-Wno-unused-command-line-argument"));
        add_llvm_lib(options);
        options.add_or_modify(&CompilerOption::new("-g"));
        options.add_or_modify(&CompilerOption::new("-fPIC"));
    }
}
