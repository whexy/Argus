# ARGUS: clang wrapper for better fuzzing

AFL++ already provided a good compiler wrapper, why bother writing another one?

1. The AFL++ one is hard to understand, with lots of C tricks, and is easy to have OOB.
2. This one uses the visitor mode to modify arguments instead of a FSM, which is clear and less error-prone.
3. Rust provides way better memory safety.

## Argus visitors

- `DefaultParametersVisitor`: Add `-Wl,rpath=<LLVM_DIR>` to avoid linking issue. Add `-Wno-unused-command-line-argument`, `-g`, `-fPIC` to avoid compiling issue. Remove `-Wl,-z defs` to avoid sanitizer issue.
- `DefaultOptimizationVisitor`: Replace optimization level to `-O3`, add `-funroll-loops`.
- `SanitizerVisitor`: Add ASAN, MSAN, UBSAN support.
- `LibfuzzerVisitor`: Remove `-fsanitize=fuzzer` and replace it with libFuzzer driver.
- `RuntimeVisitor`: Add runtime.
- `CMDFuzzVisitor`: Add command line fuzzing support.