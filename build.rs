//! Generates one test per question folder, so `cargo test` names the exact
//! question that's broken (`validate::asset_tests::asset_0001_two_sum`).

use std::path::PathBuf;

fn main() {
    println!("cargo:rerun-if-changed=questions");

    let mut dirs: Vec<String> = std::fs::read_dir("questions")
        .expect("questions/ directory")
        .filter_map(Result::ok)
        .filter(|e| e.file_type().is_ok_and(|t| t.is_dir()))
        .map(|e| e.file_name().to_string_lossy().into_owned())
        .filter(|name| !name.starts_with('.'))
        .collect();
    dirs.sort();

    let mut code = String::new();
    for dir in dirs {
        let name: String = dir
            .chars()
            .map(|c| {
                if c.is_ascii_alphanumeric() {
                    c.to_ascii_lowercase()
                } else {
                    '_'
                }
            })
            .collect();
        code.push_str(&format!(
            "#[test]\nfn asset_{name}() {{\n    check_asset({dir:?});\n}}\n\n"
        ));
    }

    let out = PathBuf::from(std::env::var("OUT_DIR").unwrap()).join("asset_tests.rs");
    std::fs::write(out, code).unwrap();
}
