use super::OptionVisitor;
use crate::{
    clang,
    compiler_option::{CompilerOption, OptionManagement},
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
    let llvm_dir = clang::get_llvm_libdir().expect("Could not find LLVM lib directory");
    // check if llvm_dir is in /usr or /lib
    if !llvm_dir.starts_with("/usr") && !llvm_dir.starts_with("/lib") {
        options.push(CompilerOption::from_arg(&format!(
            "-Wl,-rpath={}",
            llvm_dir.to_string_lossy()
        )));
    }
}

fn add_debug_option(options: &mut Vec<CompilerOption>) {
    // disable all options that start with -g
    options.add_or_modify(&CompilerOption::new("-g"));
}

fn add_pic(options: &mut Vec<CompilerOption>) {
    options.add_or_modify(&CompilerOption::new("-fPIC"));
}

impl OptionVisitor for DefaultParametersVisitor {
    fn visit(&mut self, options: &mut Vec<CompilerOption>) {
        add_llvm_lib(options);
        add_debug_option(options);
        add_pic(options);
    }
}
