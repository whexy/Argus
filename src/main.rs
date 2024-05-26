use std::os::unix::process::ExitStatusExt;

use argus::{
    env::*,
    llvm::{get_clang_path, get_clang_plus_plus_path},
    option_manager::CompilerOptionManager,
    option_visitors::{
        CMDFuzzVisitor, DefaultOptimizationVisitor, DefaultParametersVisitor, LibfuzzerVisitor,
        OptionVisitor, RuntimeVisitor, SanCovPassVisitor, SanitizerVisitor, TTRFuzzVisitor,
        XVisitor,
    },
};

use colored::*;

fn main() {
    let program_name = std::env::args().next().unwrap();
    let args: Vec<String> = std::env::args().skip(1).collect();

    let mut manager = CompilerOptionManager::new(args);

    let visitors: Vec<Box<dyn OptionVisitor>> = vec![
        Box::<DefaultParametersVisitor>::default(), // add -g, -fPIC, remove some -W options
        Box::<DefaultOptimizationVisitor>::default(), // add -O3
        Box::<SanitizerVisitor>::default(),
        Box::<XVisitor>::default(),
        Box::<LibfuzzerVisitor>::default(),
        Box::<RuntimeVisitor>::default(),
        Box::<TTRFuzzVisitor>::default(),
        Box::<CMDFuzzVisitor>::default(),
        Box::<SanCovPassVisitor>::default(),
    ];

    for mut visitor in visitors {
        visitor.visit(&mut manager.options);
        manager.cleanup();
    }

    // For debugging purposes, print the command to the console
    let compiler = if program_name.ends_with("++") {
        get_clang_plus_plus_path()
    } else {
        get_clang_path()
    }
    .expect("Could not find clang or clang++")
    .to_string_lossy()
    .to_string();

    let debug = std::env::var(ARGUS_DEBUG).is_ok();

    if debug {
        eprintln!(
            "[{}::in ] {}",
            "ARGUS".italic().bold(),
            std::env::args()
                .collect::<Vec<_>>()
                .join(" ")
                .to_string()
                .yellow()
        );

        eprintln!(
            "[{}::out] {}",
            "ARGUS".italic().bold(),
            format!("{} {}", compiler, manager).cyan()
        );
        print_envs()
    }

    // Execute the command
    let result = std::process::Command::new(compiler)
        .args(manager.get_command())
        .status()
        .expect("Failed to execute command");

    if debug {
        if result == std::process::ExitStatus::from_raw(0) {
            eprintln!("[{}::exec] {}", "ARGUS".italic().bold(), "Success".green());
        } else {
            eprintln!(
                "[{}::exec] {}",
                "ARGUS".italic().bold(),
                format!("Exit code: {}", result.code().unwrap()).red()
            );
        }
    }

    // return the exit code
    std::process::exit(result.code().unwrap());
}
