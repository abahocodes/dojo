//! `dojo schema`: JSON Schemas for question assets, generated from the same
//! serde types dojo loads them with. Committed under `schema/` so editors can
//! validate `meta.json` and `tests.json` via their `$schema` key.

use std::path::Path;

use anyhow::Result;

use crate::questions::{Meta, TestsFile};

pub fn files() -> Result<Vec<(&'static str, String)>> {
    Ok(vec![
        (
            "meta.schema.json",
            serde_json::to_string_pretty(&schemars::schema_for!(Meta))? + "\n",
        ),
        (
            "tests.schema.json",
            serde_json::to_string_pretty(&schemars::schema_for!(TestsFile))? + "\n",
        ),
    ])
}

pub fn write(out: &Path) -> Result<Vec<String>> {
    std::fs::create_dir_all(out)?;
    let mut written = vec![];
    for (name, body) in files()? {
        std::fs::write(out.join(name), body)?;
        written.push(out.join(name).display().to_string());
    }
    Ok(written)
}

#[cfg(test)]
mod tests {
    /// The committed schemas must match the types; run `dojo schema` to fix.
    #[test]
    fn committed_schemas_are_current() {
        let dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("schema");
        for (name, body) in super::files().unwrap() {
            let committed = std::fs::read_to_string(dir.join(name)).unwrap_or_default();
            assert_eq!(committed, body, "schema/{name} is stale: run `dojo schema`");
        }
    }

    use std::path::Path;
}
