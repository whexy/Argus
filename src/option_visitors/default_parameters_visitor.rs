use std::option;

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

fn disable_unfriendly_options(options: &mut Vec<CompilerOption>) {
    let unfriendly_options = vec![
        "-Wl,-z,defs",
        "-Wl,--no-undefined",
        "-Wl,-no-undefined",
        "--no-undefined",
    ];

    for option in unfriendly_options {
        if let Some(option) = options.get_mut_option(option) {
            option.disable();
        }
    }

    // remove the -z defs options. This requires some hacky code.
    let mut i = 0;
    while i < options.len() {
        if options[i].name == "-z" || options[i].name == "-Wl,-z" {
            // check the next option
            if i + 1 < options.len() {
                if options[i + 1].name == "defs" || options[i + 1].name == "-Wl,defs" {
                    options[i].disable();
                    options[i + 1].disable();
                    i += 1;
                }
            }
        }
        i += 1;
    }
}

impl OptionVisitor for DefaultParametersVisitor {
    fn visit(&mut self, options: &mut Vec<CompilerOption>) {
        disable_unfriendly_options(options);
        options.add_or_modify(&CompilerOption::new("-Wno-unused-command-line-argument"));
        add_llvm_lib(options);
        options.add_or_modify(&CompilerOption::new("-g"));
        options.add_or_modify(&CompilerOption::new("-fPIC"));
    }
}
