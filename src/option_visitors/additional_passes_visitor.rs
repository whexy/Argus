use crate::{
    compiler_option::{CompilerOption, OptionManagement},
    env::ADD_ADDITIONAL_PASSES,
    llvm, object,
};

use super::OptionVisitor;

/// Add LLVM pass
pub struct LLVMPassManager {
    llvm_version: u32,
}

impl LLVMPassManager {
    pub fn new() -> Self {
        let llvm_version = llvm::get_llvm_major_version().expect("Could not get LLVM version");
        LLVMPassManager { llvm_version }
    }

    pub fn add_llvm_pass(&self, options: &mut Vec<CompilerOption>, pass: &str) {
        let pass_object = object::find_object(pass)
            .unwrap_or_else(|| panic!("Could not find pass object file {}", pass));
        let pass_path = pass_object
            .canonicalize()
            .unwrap()
            .to_string_lossy()
            .to_string();
        match self.llvm_version {
            1..=10 => {
                options.add_option(&CompilerOption::new("-Xclang"));
                options.add_option(&CompilerOption::new("-load"));
                options.add_option(&CompilerOption::new("-Xclang"));
                options.add_option(&CompilerOption::new(&pass_path));
            }
            11..=15 => {
                options.add_option(&CompilerOption::new("-fexperimental-new-pass-manager"));
                options.add_option(&CompilerOption::new_with_value(
                    "-fpass-plugin",
                    vec![&pass_path],
                ));
            }
            16..=18 => {
                options.add_option(&CompilerOption::new_with_value(
                    "-fpass-plugin",
                    vec![&pass_path],
                ));
            }
            _ => {
                panic!("Unsupported LLVM version");
            }
        }
    }
}

// Additional passes visitor
#[derive(Default)]
pub struct AdditionalPassesVisitor {}

impl AdditionalPassesVisitor {
    pub fn new() -> Self {
        Self::default()
    }
}

impl OptionVisitor for AdditionalPassesVisitor {
    fn visit(&mut self, options: &mut Vec<CompilerOption>) {
        let pass_manager = LLVMPassManager::new();

        // read pass list from the environment variable
        let pass_list =
            std::env::var(ADD_ADDITIONAL_PASSES).unwrap_or_else(|_| String::from("SanCovPass.so"));
        for pass in pass_list.split(',') {
            pass_manager.add_llvm_pass(options, pass);
        }
    }
}
