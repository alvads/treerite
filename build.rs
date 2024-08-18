fn main() {
    add_search_path();
    add_llvm_path();
    build_lib();
}

fn build_lib() {
    let dst = cmake::Config::new("treelite")
        .define("Treelite_BUILD_STATIC_LIBS", "ON")
        .define("Treelite_USE_DYNAMIC_MSVC_RUNTIME", "ON")
        .build();
    println!("cargo:rustc-link-search={}/lib", dst.display());
    println!("cargo:rustc-link-lib=static=treelite_runtime_static");
    #[cfg(target_os = "linux")]
    {
        println!("cargo:rustc-link-lib=stdc++");
    }
    #[cfg(target_os = "macos")]
    {
        println!("cargo:rustc-link-lib=c++");
    }
}

#[cfg(not(target_os = "windows"))]
fn add_search_path() {
    for path in std::env::var("LD_LIBRARY_PATH")
        .unwrap_or_else(|_| "".to_string())
        .split(":")
    {
        if path.trim().len() == 0 {
            continue;
        }
        println!("cargo:rustc-link-search={}", path);
    }
}

#[cfg(target_os = "windows")]
fn add_search_path() {
    for path in std::env::var("PATH")
        .unwrap_or_else(|_| "".to_string())
        .split(";")
    {
        if path.trim().len() == 0 {
            continue;
        }
        println!("cargo:rustc-link-search={}", path);
    }
}

fn add_llvm_path() {
    if let Some(llvm_config_path) = option_env!("LLVM_CONFIG_PATH") {
        println!("cargo:rustc-env=LLVM_CONFIG_PATH={}", llvm_config_path);
    }
}
