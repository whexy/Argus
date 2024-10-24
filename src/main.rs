use std::fs::OpenOptions;
use std::io::Write;
use std::os::unix::process::ExitStatusExt;

use argus::{
    env::*,
    llvm::{get_clang_path, get_clang_plus_plus_path},
    option_manager::CompilerOptionManager,
    option_visitors::{
        AdditionalObjectsVisitor, AdditionalPassesVisitor, DefaultOptimizationVisitor,
        DefaultParametersVisitor, LibfuzzerVisitor, OptionVisitor, ProfileVisitor, RuntimeVisitor,
        SanitizerVisitor, XVisitor,
    },
};

use colored::*;

fn main() {
    let program_name = std::env::args().next().unwrap();
    let args: Vec<String> = std::env::args().skip(1).collect();

    let mut manager = CompilerOptionManager::new(args);

    let mut visitors: Vec<Box<dyn OptionVisitor>> = vec![
        Box::<DefaultParametersVisitor>::default(),
        Box::<DefaultOptimizationVisitor>::default(),
        Box::<SanitizerVisitor>::default(),
        Box::<XVisitor>::default(),
    ];

    // Optional visitors
    if std::env::var(ADD_DRIVER).is_ok() {
        visitors.push(Box::<LibfuzzerVisitor>::default());
    }
    if std::env::var(ADD_RUNTIME).is_ok() {
        visitors.push(Box::<RuntimeVisitor>::default());
    }
    if std::env::var(PROFILING).is_ok() {
        visitors.push(Box::<ProfileVisitor>::default());
    }
    if std::env::var(ADD_ADDITIONAL_PASSES).is_ok() {
        visitors.push(Box::<AdditionalPassesVisitor>::default());
    }
    if std::env::var(ADD_ADDITIONAL_OBJECTS).is_ok() {
        visitors.push(Box::<AdditionalObjectsVisitor>::default());
    }

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

        // also, append the command to the end of /tmp/argus.log file, if it exists. Otherwise, create it first.
        let log_path = std::path::Path::new("/tmp/argus.log");
        let log_entry = format!(
            "{} \n \t ==> {} {}\n",
            std::env::args().collect::<Vec<_>>().join(" "),
            compiler,
            manager
        );

        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(log_path)
            .expect("Could not open log file");

        writeln!(file, "{}", log_entry).expect("Could not write to log file");
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
