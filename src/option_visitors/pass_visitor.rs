use crate::{
    compiler_option::{CompilerOption, OptionManagement},
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
        let pass_object =
            object::find_object(pass).expect(&format!("Could not find pass object file {}", pass));
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

pub struct CMDFuzzVisitor {
    pass_manager: LLVMPassManager,
    enabled: bool,
}

impl Default for CMDFuzzVisitor {
    fn default() -> Self {
        Self::new()
    }
}

impl CMDFuzzVisitor {
    pub fn new() -> Self {
        CMDFuzzVisitor {
            pass_manager: LLVMPassManager::new(),
            enabled: std::env::var("CMDFUZZ").is_ok(),
        }
    }
}

impl OptionVisitor for CMDFuzzVisitor {
    fn visit(&mut self, options: &mut Vec<CompilerOption>) {
        if !self.enabled {
            return;
        }
        self.pass_manager
            .add_llvm_pass(options, "libArgFuzzPass.so");
    }
}