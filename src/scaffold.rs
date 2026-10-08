//! `dojo scaffold`: drafts boilerplate files from a question's signature and
//! registers them in `meta.json`. An authoring aid only: at runtime dojo
//! copies the boilerplate exactly as written, so edit what this produces.

use std::path::Path;

use anyhow::{Context, Result, bail};
use serde::Deserialize;
use serde_json::{Value, json};

use crate::lang::{Language, camel};
use crate::questions::{Difficulty, Signature, Type};

/// The parts of `meta.json` a draft needs; works on half-written questions.
#[derive(Deserialize)]
pub struct Meta {
    pub id: u32,
    pub title: String,
    pub difficulty: Difficulty,
    pub signature: Signature,
}

pub fn boilerplate(meta: &Meta, lang: Language) -> String {
    match lang {
        Language::Python => python(meta),
        Language::JavaScript => javascript(meta),
        Language::TypeScript => typescript(meta),
        Language::Go => crate::runner::compiled::go::boilerplate(&meta.signature, &header(meta)),
        Language::Java => {
            crate::runner::compiled::java::boilerplate(&meta.signature, &header(meta))
        }
        Language::Cpp => crate::runner::compiled::cpp::boilerplate(&meta.signature, &header(meta)),
    }
}

/// The two comment lines every boilerplate starts with (without comment
/// markers).
fn header(meta: &Meta) -> String {
    format!(
        "{}. {} ({})\n/show to reread the problem · /test runs visible tests · /submit when done",
        meta.id, meta.title, meta.difficulty
    )
}

fn py_type(t: &Type) -> String {
    match t {
        Type::Int | Type::Long => "int".into(),
        Type::Float => "float".into(),
        Type::Bool => "bool".into(),
        Type::String => "str".into(),
        Type::ListNode => "ListNode | None".into(),
        Type::TreeNode => "TreeNode | None".into(),
        Type::List(inner) => format!("list[{}]", py_type(inner)),
    }
}

fn uses(meta: &Meta, t: &Type) -> bool {
    let sig = &meta.signature;
    sig.returns.uses(t) || sig.params.iter().any(|p| p.ty.uses(t))
}

fn python(meta: &Meta) -> String {
    let sig = &meta.signature;
    let mut s = format!(
        "# {}. {} ({})\n# /show to reread the problem · /test runs visible tests · /submit when done\n\n",
        meta.id, meta.title, meta.difficulty
    );
    if uses(meta, &Type::ListNode) {
        s.push_str(
            "class ListNode:\n    def __init__(self, val=0, next=None):\n        self.val = val\n        self.next = next\n\n\n",
        );
    }
    if uses(meta, &Type::TreeNode) {
        s.push_str(
            "class TreeNode:\n    def __init__(self, val=0, left=None, right=None):\n        self.val = val\n        self.left = left\n        self.right = right\n\n\n",
        );
    }
    let params = sig
        .params
        .iter()
        .map(|p| format!("{}: {}", p.name, py_type(&p.ty)))
        .collect::<Vec<_>>()
        .join(", ");
    s.push_str(&format!(
        "def {}({}) -> {}:\n    pass\n",
        sig.function,
        params,
        py_type(&sig.returns)
    ));
    s
}

fn js_type(t: &Type) -> String {
    match t {
        Type::Int | Type::Long | Type::Float => "number".into(),
        Type::Bool => "boolean".into(),
        Type::String => "string".into(),
        Type::ListNode => "ListNode | null".into(),
        Type::TreeNode => "TreeNode | null".into(),
        Type::List(inner) => match **inner {
            Type::ListNode | Type::TreeNode => format!("Array<{}>", js_type(inner)),
            _ => format!("{}[]", js_type(inner)),
        },
    }
}

fn javascript(meta: &Meta) -> String {
    let sig = &meta.signature;
    let mut s = format!(
        "// {}. {} ({})\n// /show to reread the problem · /test runs visible tests · /submit when done\n\n",
        meta.id, meta.title, meta.difficulty
    );
    if uses(meta, &Type::ListNode) {
        s.push_str(
            "/**\n * Provided by dojo:\n * function ListNode(val, next) {\n *     this.val = (val === undefined ? 0 : val);\n *     this.next = (next === undefined ? null : next);\n * }\n */\n\n",
        );
    }
    if uses(meta, &Type::TreeNode) {
        s.push_str(
            "/**\n * Provided by dojo:\n * function TreeNode(val, left, right) {\n *     this.val = (val === undefined ? 0 : val);\n *     this.left = (left === undefined ? null : left);\n *     this.right = (right === undefined ? null : right);\n * }\n */\n\n",
        );
    }
    s.push_str("/**\n");
    for p in &sig.params {
        s.push_str(&format!(
            " * @param {{{}}} {}\n",
            js_type(&p.ty),
            camel(&p.name)
        ));
    }
    s.push_str(&format!(" * @return {{{}}}\n */\n", js_type(&sig.returns)));
    let params = sig
        .params
        .iter()
        .map(|p| camel(&p.name))
        .collect::<Vec<_>>()
        .join(", ");
    s.push_str(&format!(
        "function {}({}) {{\n\n}}\n",
        Language::JavaScript.function_name(&sig.function),
        params
    ));
    s
}

fn ts_type(t: &Type) -> String {
    match t {
        Type::List(inner) if matches!(**inner, Type::ListNode | Type::TreeNode) => {
            format!("Array<{}>", ts_type(inner))
        }
        Type::List(inner) => format!("{}[]", ts_type(inner)),
        other => js_type(other),
    }
}

fn typescript(meta: &Meta) -> String {
    let sig = &meta.signature;
    let mut s = String::new();
    for line in header(meta).lines() {
        s.push_str(&format!("// {line}\n"));
    }
    s.push('\n');
    if uses(meta, &Type::ListNode) {
        s.push_str(
            "/**\n * Provided by dojo:\n * class ListNode {\n *     val: number\n *     next: ListNode | null\n *     constructor(val?: number, next?: ListNode | null)\n * }\n */\n\n",
        );
    }
    if uses(meta, &Type::TreeNode) {
        s.push_str(
            "/**\n * Provided by dojo:\n * class TreeNode {\n *     val: number\n *     left: TreeNode | null\n *     right: TreeNode | null\n *     constructor(val?: number, left?: TreeNode | null, right?: TreeNode | null)\n * }\n */\n\n",
        );
    }
    let params = sig
        .params
        .iter()
        .map(|p| format!("{}: {}", camel(&p.name), ts_type(&p.ty)))
        .collect::<Vec<_>>()
        .join(", ");
    s.push_str(&format!(
        "function {}({}): {} {{\n\n}}\n",
        Language::TypeScript.function_name(&sig.function),
        params,
        ts_type(&sig.returns)
    ));
    s
}

/// Drafts boilerplate for `langs` in one question folder and adds them to
/// `meta.json` (`languages.<lang>`). Existing files are kept unless `force`.
/// Returns the files written.
pub fn run(question_dir: &Path, langs: &[Language], force: bool) -> Result<Vec<String>> {
    if langs.is_empty() {
        bail!("no languages given");
    }
    let meta_path = question_dir.join("meta.json");
    let raw = std::fs::read_to_string(&meta_path)
        .with_context(|| format!("reading {}", meta_path.display()))?;
    let mut doc: Value =
        serde_json::from_str(&raw).with_context(|| format!("invalid {}", meta_path.display()))?;
    let meta: Meta = serde_json::from_value(doc.clone()).with_context(|| {
        format!(
            "{} needs id, title, difficulty and signature",
            meta_path.display()
        )
    })?;

    let languages = doc
        .as_object_mut()
        .context("meta.json must be an object")?
        .entry("languages")
        .or_insert_with(|| json!({}));
    let mut written = Vec::new();
    for &lang in langs {
        let entry = languages
            .as_object_mut()
            .context("`languages` must be an object")?
            .entry(lang.name())
            .or_insert_with(|| {
                json!({
                    "boilerplate": format!("boilerplate/{}", lang.asset_file()),
                    "solution": format!("solutions/{}", lang.asset_file()),
                })
            });
        let rel = entry["boilerplate"]
            .as_str()
            .with_context(|| format!("languages.{lang}.boilerplate must be a path"))?
            .to_string();
        let path = question_dir.join(&rel);
        if path.exists() && !force {
            continue;
        }
        if let Some(dir) = path.parent() {
            std::fs::create_dir_all(dir)?;
        }
        std::fs::write(&path, boilerplate(&meta, lang))?;
        written.push(rel);
    }

    let updated = serde_json::to_string_pretty(&doc)? + "\n";
    if updated != raw {
        std::fs::write(&meta_path, updated)?;
        written.push("meta.json".into());
    }
    Ok(written)
}
