#![feature(cfg_target_feature)]

use std::env;
use std::path::PathBuf;

fn main() {
    let target = env::var("TARGET").unwrap();
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let mut lib_dir = manifest_dir.clone();
    let mut dll_dir = manifest_dir.clone();
    lib_dir.push("lib");
    dll_dir.push("lib");

    let (mut lib_dir, mut dll_dir) = if target.contains("pc-windows") {
        if target.contains("msvc") {
            lib_dir.push("msvc");
            dll_dir.push("msvc");

            #[cfg(not(target_feature = "crt-static"))]
            {
                lib_dir.push("md");
            }
            #[cfg(target_feature = "crt-static")]
            {
                lib_dir.push("mt");
            }
        } else {
            lib_dir.push("gnu-mingw");
            dll_dir.push("gnu-mingw");
        }

        (lib_dir, dll_dir)
    } else if target.contains("linux") {
        lib_dir.push("linux");
        dll_dir.push("linux");

        (lib_dir, dll_dir)
    } else if target.contains("darwin") {
        lib_dir.push("darwin");
        dll_dir.push("darwin");

        (lib_dir, dll_dir)
    } else {
        return;
    };

    lib_dir.push("lib");
    dll_dir.push("dll");
    if target.contains("x86_64") {
        lib_dir.push("64");
        dll_dir.push("64");
    } else {
        lib_dir.push("32");
        dll_dir.push("32");
    }
    println!("cargo:rustc-link-search=all={}", lib_dir.display());
    if let Ok(read_dir) = std::fs::read_dir(dll_dir) {
        for entry in read_dir {
            let entry_path = entry.expect("Invalid fs entry").path();
            let file_name_result = entry_path.file_name();
            let mut new_file_path = manifest_dir.clone();
            new_file_path.push("target");
            new_file_path.push(env::var("PROFILE").unwrap());

            if let Some(file_name) = file_name_result {
                let file_name = file_name.to_str().unwrap();
                if file_name.ends_with(".dll") || file_name.ends_with(".dylib") ||
                    file_name.contains(".so")
                {
                    new_file_path.push(file_name);
                    std::fs::copy(&entry_path, new_file_path.as_path())
                        .expect("Can't copy from DLL dir");
                }
            }
        }
    }
}
