use std::env;
use std::fs;
use std::path::{Path, PathBuf};

/// Try our best to find the object file.
/// We'll try the following methods:
/// 0. Check the object file path directly.
/// 1. Check the argv[0] directory.
/// 2. Check /proc/self/exe (Linux specific).
/// 3. Use a predefined, hardcoded path.
/// 4. Try the current directory.
/// 5. Try the / (root) directory.
pub fn find_object(obj: &str) -> Option<PathBuf> {
    // Check the object file path directly
    if Path::new(obj).exists() {
        return Some(PathBuf::from(obj));
    }

    // Get the object file name
    let obj = &Path::new(obj)
        .file_name()
        .unwrap()
        .to_string_lossy()
        .to_string();

    // Check the argv[0] directory
    if let Some(argv0) = env::args().next() {
        if let Some(parent) = Path::new(&argv0).parent() {
            let path = parent.join(obj);
            if path.exists() {
                return Some(path);
            }
        }
    }

    // Check the /proc/self/exe path
    let proc_path = PathBuf::from("/proc/self/exe");
    if let Ok(exe_path) = fs::read_link(proc_path) {
        if let Some(parent) = exe_path.parent() {
            let path = parent.join(obj);
            if path.exists() {
                return Some(path);
            }

            let path = parent.join("../bin").join(obj);
            if path.exists() {
                return Some(path);
            }
        }
    }

    // Check the predefined path
    let predefined_bandfuzz_path = "/workspaces/BANDFUZZplusplus/bin";
    let path = Path::new(predefined_bandfuzz_path).join(obj);
    if path.exists() {
        return Some(path);
    }

    // Try the current directory
    let path = Path::new(".").join(obj);
    if path.exists() {
        return Some(path);
    }

    // Try the / (root) directory
    let path = Path::new("/").join(obj);
    if path.exists() {
        return Some(path);
    }

    None
}
