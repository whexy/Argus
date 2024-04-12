use std::io::{self, ErrorKind};
use std::path::PathBuf;

/// Get LLVM bindir by running `llvm-config --bindir`
pub fn get_llvm_bindir() -> io::Result<PathBuf> {
    let output = std::process::Command::new("llvm-config")
        .arg("--bindir")
        .output()?;

    let path_str =
        String::from_utf8(output.stdout).map_err(|e| io::Error::new(ErrorKind::InvalidData, e))?;
    Ok(PathBuf::from(path_str.trim()))
}

/// Get LLVM libdir by running `llvm-config --libdir`
pub fn get_llvm_libdir() -> io::Result<PathBuf> {
    let output = std::process::Command::new("llvm-config")
        .arg("--libdir")
        .output()?;

    let path_str =
        String::from_utf8(output.stdout).map_err(|e| io::Error::new(ErrorKind::InvalidData, e))?;
    Ok(PathBuf::from(path_str.trim()))
}

/// Get the path to the clang binary
pub fn get_clang_path() -> io::Result<PathBuf> {
    let bindir = get_llvm_bindir()?;
    let clang_path = bindir.join("clang");
    // Check if clang exists
    if !clang_path.exists() {
        return Err(io::Error::new(
            ErrorKind::NotFound,
            format!("clang not found at {:?}", clang_path),
        ));
    }
    Ok(clang_path)
}

/// Get the path to the clang++ binary
pub fn get_clang_plus_plus_path() -> io::Result<PathBuf> {
    let bindir = get_llvm_bindir()?;
    let clang_plus_plus_path = bindir.join("clang++");
    // Check if clang++ exists
    if !clang_plus_plus_path.exists() {
        return Err(io::Error::new(
            ErrorKind::NotFound,
            format!("clang++ not found at {:?}", clang_plus_plus_path),
        ));
    }
    Ok(clang_plus_plus_path)
}
