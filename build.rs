//! Packs the question bank into one zstd-compressed blob for the binary
//! (`questions::Bank::embedded`). One stream compresses far better than
//! file-by-file: the questions share most of their structure.
//!
//! Also rebuilds when question folders are added or removed, so the
//! per-question tests (`#[files("questions/*/meta.json")]` in validate.rs)
//! stay current.

use std::path::Path;

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

    // Archive: for each file, u32 path length, path (relative to
    // questions/, `/`-separated), u32 data length, data. Little-endian.
    let mut archive = Vec::new();
    for folder in &folders {
        let mut files = Vec::new();
        collect(
            Path::new("questions"),
            &Path::new("questions").join(folder),
            &mut files,
        );
        files.sort();
        for (path, data) in files {
            archive.extend_from_slice(&(path.len() as u32).to_le_bytes());
            archive.extend_from_slice(path.as_bytes());
            archive.extend_from_slice(&(data.len() as u32).to_le_bytes());
            archive.extend_from_slice(&data);
        }
    }
    // Best compression for releases; fast for everyday builds.
    let level = if std::env::var("PROFILE").as_deref() == Ok("debug") {
        3
    } else {
        19
    };
    let packed = zstd::encode_all(archive.as_slice(), level).expect("compress question bank");
    let out = Path::new(&std::env::var("OUT_DIR").expect("OUT_DIR")).join("bank.zst");
    std::fs::write(out, packed).expect("write bank.zst");
}

fn collect(root: &Path, dir: &Path, out: &mut Vec<(String, Vec<u8>)>) {
    for entry in std::fs::read_dir(dir)
        .expect("read question folder")
        .flatten()
    {
        let path = entry.path();
        let name = entry.file_name();
        // Skip hidden files (.DS_Store) and Python caches.
        if name.to_string_lossy().starts_with('.') || name == "__pycache__" {
            continue;
        }
        if path.is_dir() {
            collect(root, &path, out);
        } else {
            let rel = path
                .strip_prefix(root)
                .expect("inside questions/")
                .components()
                .map(|c| c.as_os_str().to_string_lossy().into_owned())
                .collect::<Vec<_>>()
                .join("/");
            out.push((rel, std::fs::read(&path).expect("read question file")));
        }
    }
}
