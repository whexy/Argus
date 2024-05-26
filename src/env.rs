use colored::*;

// Define a macro to create environment variables and their descriptions
macro_rules! define_env_vars {
    ($($name:ident: $value:expr => $description:expr),* $(,)?) => {
        $(
            pub const $name: &str = $value;
        )*

        pub const ARGUS_ENVS: &[(&str, &str)] = &[
            $(
                ($value, $description),
            )*
        ];
    };
}

// Use the macro to define environment variables and their descriptions
define_env_vars! {
    ARGUS_DEBUG: "ARGUS_DEBUG" => "Enable debugging for Argus.",
    CMDFUZZ: "CMDFUZZ" => "Command (argument) fuzzing mode.",
    DISABLE_COVSAN: "BANDFUZZ_NOCOV" => "Disable coverage sanitizer.",
    DRIVER: "FUZZER_LIB" => "Library used as the fuzzer driver (libFuzzer mode).",
    ENABLE_ASAN: "AFL_USE_ASAN" => "Enable AddressSanitizer.",
    ENABLE_MSAN: "AFL_USE_MSAN" => "Enable MemorySanitizer.",
    ENABLE_UBSAN: "AFL_USE_UBSAN" => "Enable UndefinedBehaviorSanitizer.",
    NORUNTIME: "BANDFUZZ_NORUNTIME" => "Disable linking runtime to the target.",
    NOSANITIZER: "BANDFUZZ_NOSAN" => "Disable all sanitizers.",
    OPT_LEVEL: "BANDFUZZ_OPT" => "Optimization level for the target.",
    RUNTIME: "BANDFUZZ_RUNTIME" => "File path to runtime linked to the target.",
    TTRFUZZ: "TTRFUZZ" => "Calculate Time-To-Reach.",
    NATIVE_SANCOV: "BANDFUZZ_NATIVESANCOV" => "Use native coverage sanitizer.",
}

pub fn print_envs() {
    for (env, description) in ARGUS_ENVS.iter() {
        println!(
            "{}: {}",
            env.bold().cyan(),
            std::env::var(env).unwrap_or_else(|_| String::from("-")),
        );
        println!("  {}", description.italic().dimmed());
    }
}
