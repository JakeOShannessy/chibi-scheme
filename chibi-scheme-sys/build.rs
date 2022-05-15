use std::collections::HashSet;
use std::env;

// Taken from https://github.com/rust-lang/rust-bindgen/issues/687
#[derive(Debug)]
struct IgnoreMacros(HashSet<String>);

impl bindgen::callbacks::ParseCallbacks for IgnoreMacros {
    fn will_parse_macro(&self, name: &str) -> bindgen::callbacks::MacroParsingBehavior {
        if self.0.contains(name) {
            bindgen::callbacks::MacroParsingBehavior::Ignore
        } else {
            bindgen::callbacks::MacroParsingBehavior::Default
        }
    }
}

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let dst = cmake::Config::new("chibi-scheme")
        .define("BUILD_SHARED_LIBS", "OFF")
        .build();
    let ignored_macros = IgnoreMacros(
        vec![
            "FP_INFINITE".into(),
            "FP_NAN".into(),
            "FP_NORMAL".into(),
            "FP_SUBNORMAL".into(),
            "FP_ZERO".into(),
            "IPPORT_RESERVED".into(),
            "FP_INT_UPWARD".into(),
            "FP_INT_DOWNWARD".into(),
            "FP_INT_TOWARDZERO".into(),
            "FP_INT_TONEARESTFROMZERO".into(),
            "FP_INT_TONEAREST".into(),
        ]
        .into_iter()
        .collect(),
    );
    let bindings = bindgen::Builder::default()
        .clang_arg(format!("-I{}", dst.join("include").display()))
        .header(format!("{}/include/chibi/eval.h", dst.display()))
        .parse_callbacks(Box::new(ignored_macros))
        .blocklist_type("_?P?IMAGE_TLS_DIRECTORY.*")
        .generate()
        .expect("Unable to generate bindings");
    bindings
        .write_to_file(format!("{}/bindings.rs", out_dir))
        .expect("Could not write bindings");

    println!(
        "cargo:rustc-link-search=native={}",
        dst.join("lib").display()
    );
    #[cfg(windows)]
    let libname = "libchibi-scheme";
    #[cfg(not(windows))]
    let libname = "chibi-scheme";
    println!("cargo:rustc-link-lib=static={}", libname);
}
