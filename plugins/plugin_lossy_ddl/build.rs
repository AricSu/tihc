use std::env;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    // Check for feature flags
    let build_tidb = env::var("CARGO_FEATURE_TIDB_ENGINE").is_ok();
    let build_rust_fallback = env::var("CARGO_FEATURE_RUST_FALLBACK").is_ok();
    
    println!("cargo:rerun-if-env-changed=CARGO_FEATURE_TIDB_ENGINE");
    println!("cargo:rerun-if-env-changed=CARGO_FEATURE_RUST_FALLBACK");
    println!("cargo:rerun-if-changed=go/");
    
    if build_tidb {
        println!("cargo:warning=Building with TiDB engine support");
        build_tidb_engine();
    }
    
    if build_rust_fallback {
        println!("cargo:warning=Rust fallback engine enabled");
    }
    
    if !build_tidb && !build_rust_fallback {
        println!("cargo:warning=No engines explicitly enabled, building with default configuration");
    }
    
    // Always link system frameworks on macOS for potential Go runtime
    #[cfg(target_os = "macos")]
    {
        println!("cargo:rustc-link-lib=framework=CoreFoundation");
        println!("cargo:rustc-link-lib=framework=Security");
        println!("cargo:rustc-link-lib=framework=IOKit");
        println!("cargo:rustc-link-lib=resolv");
    }
}

fn build_tidb_engine() {
    let go_dir = PathBuf::from("go");
    let out_dir = env::var("OUT_DIR").unwrap();
    let out_path = PathBuf::from(&out_dir);
    
    if !go_dir.join("libschematracker.go").exists() {
        println!("cargo:warning=Go source not found, TiDB engine will not be available");
        return;
    }
    
    // Build TiDB integration library
    let output = Command::new("go")
        .current_dir(&go_dir)
        .args(&["build", "-buildmode=c-archive", "-o"])
        .arg(out_path.join("libschematracker.a"))
        .arg("libschematracker.go")
        .output()
        .expect("Failed to execute Go build command");
        
    if !output.status.success() {
        println!("cargo:warning=TiDB Go build failed: {}", String::from_utf8_lossy(&output.stderr));
        println!("cargo:warning=TiDB engine will not be available");
        return;
    }
    
    // Copy header file
    if go_dir.join("libschematracker.h").exists() {
        std::fs::copy(
            go_dir.join("libschematracker.h"), 
            out_path.join("libschematracker.h")
        ).expect("Failed to copy header file");
    }
    
    // Configure linking
    println!("cargo:rustc-link-search=native={}", out_dir);
    println!("cargo:rustc-link-lib=static=schematracker");
    
    // Platform-specific Go runtime dependencies
    #[cfg(target_os = "macos")]
    {
        println!("cargo:rustc-link-lib=framework=CoreFoundation");
        println!("cargo:rustc-link-lib=framework=Security");  
        println!("cargo:rustc-link-lib=framework=IOKit");
        println!("cargo:rustc-link-lib=resolv");
    }
    
    #[cfg(target_os = "linux")]
    {
        println!("cargo:rustc-link-lib=pthread");
        println!("cargo:rustc-link-lib=dl");
        println!("cargo:rustc-link-lib=m");
    }
    
    // Generate bindings
    let bindings_path = out_path.join("bindings.rs");
    std::fs::write(&bindings_path, r#"
extern "C" {
    pub fn precheck_sql(sql_ptr: *const std::os::raw::c_char) -> std::os::raw::c_int;
}
"#).expect("Failed to write bindings");
    
    println!("cargo:warning=TiDB engine build completed successfully");
}
