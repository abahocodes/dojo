//! Rebuilds when question folders are added or removed, so the per-question
//! tests (`#[files("questions/*/meta.json")]` in validate.rs) stay current.

fn main() {
    println!("cargo:rerun-if-changed=questions");
    let mut folders: Vec<String> = std::fs::read_dir("questions")
        .expect("questions/ directory")
        .filter_map(Result::ok)
        .filter(|e| e.file_type().is_ok_and(|t| t.is_dir()))
        .map(|e| e.file_name().to_string_lossy().into_owned())
        .collect();
    folders.sort();
    // A changed value forces the crate (and its test list) to recompile.
    println!(
        "cargo:rustc-env=DOJO_QUESTION_FOLDERS={}",
        folders.join(",")
    );
}
