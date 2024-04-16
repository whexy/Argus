use crate::compiler_option::{CompilerOption, OptionManagement};

use super::OptionVisitor;

pub struct DefaultOptimizationVisitor {
    optimization_level: Option<u8>,
}

impl Default for DefaultOptimizationVisitor {
    fn default() -> Self {
        Self::new()
    }
}

impl DefaultOptimizationVisitor {
    pub fn new() -> Self {
        DefaultOptimizationVisitor {
            optimization_level: None,
        }
    }

    pub fn init(&mut self, options: &Vec<CompilerOption>) {
        let mut option_level = None;
        if options.get_option("-O3").is_some() {
            option_level = Some(3);
        } else if options.get_option("-O2").is_some() {
            option_level = Some(2);
        } else if options.get_option("-O1").is_some() {
            option_level = Some(1);
        } else if options.get_option("-O0").is_some() {
            option_level = Some(0);
        }

        // Override by environment variable
        if let Ok(level) = std::env::var("BANDFUZZ_OPT") {
            if let Ok(level) = level.parse::<u8>() {
                if (0..=3).contains(&level) {
                    option_level = Some(level);
                }
            }
        }
        self.optimization_level = option_level;
    }
}

fn disable_optimization_options(options: &mut Vec<CompilerOption>) {
    options.retain(|opt| !opt.name.starts_with("-O"));
}

fn enable_o0(options: &mut Vec<CompilerOption>) {
    options.push(CompilerOption::new("-O0"));
}

fn enable_o1(options: &mut Vec<CompilerOption>) {
    options.push(CompilerOption::new("-O1"));
}

fn enable_o2(options: &mut Vec<CompilerOption>) {
    options.push(CompilerOption::new("-O2"));
}

fn enable_o3(options: &mut Vec<CompilerOption>) {
    options.push(CompilerOption::new("-O3"));
    options.push(CompilerOption::new("-funroll-loops"));
}

impl OptionVisitor for DefaultOptimizationVisitor {
    fn visit(&mut self, options: &mut Vec<CompilerOption>) {
        self.init(options);
        disable_optimization_options(options);
        match self.optimization_level {
            Some(0) => enable_o0(options),
            Some(1) => enable_o1(options),
            Some(2) => enable_o2(options),
            Some(3) => enable_o3(options),
            _ => enable_o0(options), // by default enable O0
        }
    }
}
