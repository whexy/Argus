# ARGUS

ARGUS is a compiler wrapper designed to enhance the experience of argument modification by providing a more understandable and memory-safe alternative. Built with Rust, ARGUS utilizes a visitor pattern to modify compiler arguments, resulting in a clear and less error-prone approach compared to traditional finite state machines. This project aims to simplify the process of setting up fuzzing environments and improve the overall reliability of the compilation process.

## Getting Started

To start using ARGUS, follow these simple steps:

1. **Installation**: Clone the repository and build the project using Cargo, Rust's package manager.
   ```bash
   git clone https://github.com/yourusername/argus.git
   cd argus
   cargo build --release
   ```

2. **Usage**: Replace your existing compiler calls with ARGUS to automatically apply the visitor modifications.
   ```bash
   ./argus <original-compiler-command>
   ```

3. **Configuration**: Customize the behavior of ARGUS by modifying the visitor settings with environment variables.

## ARGUS Visitors

ARGUS uses a series of visitors to modify and enhance the compilation process. Each visitor serves a specific purpose:

### Default Enabled Visitors

- **DefaultParametersVisitor**: 
  - Adds `-Wl,rpath=<LLVM_DIR>` to resolve linking issues.
  - Includes `-Wno-unused-command-line-argument`, `-g`, and `-fPIC` to address common compilation issues.
  - Removes `-Wl,-z defs` to prevent sanitizer-related problems.

- **DefaultOptimizationVisitor**: 
  - If no optimization level is specified in the compiler command, `-O0` is added by default to ensure no optimization is applied.
  - If an optimization level is already specified in the compiler command, it will be retained as is.
  - The environment variable `BANDFUZZ_OPT` can be used to override the optimization level. For instance, setting `export BANDFUZZ_OPT=2` will apply `-O2` to the compilation command.
  - When `-O3` is used, the flag `-funroll-loops` is also added to improve loop performance.

- **SanitizerVisitor**: 
  - Incorporates support for AddressSanitizer (ASAN), MemorySanitizer (MSAN), and UndefinedBehaviorSanitizer (UBSAN) to improve code safety and detect potential issues.
  - The environment variables `ENABLE_ASAN`, `ENABLE_MSAN`, `ENABLE_UBSAN`, and `ENABLE_COVSAN` can be used to override the default behavior.

- **XVisitor**:
  - Adds `-x none` to the compilation command if the `-x` flag is present in the original command. This is useful when compiling with mixed C and C++ sources.

### Optional Visitors

- **LibfuzzerVisitor**: 
  - Enabled by setting the `HARNESS_MODE` environment variable.
  - Removes `-fsanitize=fuzzer` and substitutes it with a driver for AFL++ fuzzing support.
  - The default driver is `bandfuzz-driver.o` for standard C libraries, and `bandfuzz-driver-libc++.o` for libc++ libraries.
  - The environment variable `BANDFUZZ_DRIVER` can be used to override the default driver.

- **RuntimeVisitor**: 
  - Enabled by setting the `ADD_RUNTIME` environment variable.
  - Adds runtime components necessary for the execution of the compiled program. In most cases, the runtime is an object file containing a group of functions to work with instrumentations.
  - The default runtime is `bandfuzz-rt.o`.
  - The environment variable `BANDFUZZ_RUNTIME` can be used to override the default runtime.

- **ProfileVisitor**:
  - Enabled by setting the `BANDFUZZ_PROFILE` environment variable.
  - Adds `-fprofile-instr-generate` and `-fcoverage-mapping` to the compilation command. By adding these two flags, you can use `llvm-cov` tools to show the coverage of the compiled program. See the LLVM documentation [here](https://llvm.org/docs/CoverageMappingFormat.html).

- **AddAdditionalPassVisitor**:
  - Enabled by setting the `ADD_ADDITIONAL_PASSES` environment variable.
  - Adds additional LLVM passes to the compilation process.

- **AddAdditionalObjectVisitor**:
  - Enabled by setting the `ADD_ADDITIONAL_OBJECTS` environment variable.
  - Adds additional object files to the compilation process.

## Environment Variables

- `ARGUS_DEBUG`: If this environment variable is set, ARGUS will print debug information to stderr.
- `BANDFUZZ_OPT`: If this environment variable is set, it will be used to set the optimization level. The value must be an integer between 0 and 3, i.e. `export BANDFUZZ_OPT=2` will apply `-O2` to the compilation command.
- `NOSANITIZER`: If this environment variable is set, all sanitizers will be disabled.
- `ENABLE_ASAN`, `ENABLE_MSAN`, `ENABLE_UBSAN`, `ENABLE_COVSAN`: If these environment variables are set, the corresponding sanitizers will be enabled (overrides `NOSANITIZER`).
- `HARNESS_MODE`: Enables the LibfuzzerVisitor for AFL++ fuzzing support.
- `BANDFUZZ_DRIVER`: Overrides the default fuzzing driver.
- `ADD_RUNTIME`: Enables the RuntimeVisitor to add runtime components.
- `BANDFUZZ_RUNTIME`: Overrides the default runtime component.
- `BANDFUZZ_PROFILE`: Enables the ProfileVisitor for coverage mapping.
- `ADD_ADDITIONAL_PASSES`: Enables the addition of extra LLVM passes.
- `ADD_ADDITIONAL_OBJECTS`: Enables the addition of extra object files.

## Contributing

We welcome contributions from the community! If you have suggestions for improvements or new features, feel free to open an issue or submit a pull request. Please ensure that your contributions adhere to the project's coding standards and include appropriate tests.

## License

ARGUS is licensed under the MIT License. See the [LICENSE](LICENSE) file for more details.

## Why ARGUS?

While AFL++ already offers a compiler wrapper, ARGUS brings several improvements:

1. **Clarity and Simplicity**: The AFL++ wrapper can be complex, employing numerous C tricks that may lead to out-of-bounds (OOB) issues. ARGUS, on the other hand, leverages Rust's visitor pattern for a more straightforward and intuitive modification of compiler arguments.

2. **Memory Safety**: Rust's strong emphasis on memory safety reduces the risk of common programming errors, making ARGUS a more robust solution for handling compiler arguments.

By providing a more user-friendly and memory-safe experience, ARGUS aims to be the go-to solution for developers looking to harness the power of AFL++ while minimizing complexity and errors.