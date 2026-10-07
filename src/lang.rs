//! Languages a question can be solved in.

use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Language {
    Python,
    JavaScript,
}

impl Language {
    pub const ALL: &[Language] = &[Language::Python, Language::JavaScript];

    pub fn parse(s: &str) -> Option<Language> {
        match s.to_ascii_lowercase().as_str() {
            "python" | "python3" | "py" => Some(Language::Python),
            "javascript" | "js" | "node" => Some(Language::JavaScript),
            _ => None,
        }
    }

    /// Stable identifier used in config, the database and asset file names.
    pub fn name(self) -> &'static str {
        match self {
            Language::Python => "python",
            Language::JavaScript => "javascript",
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            Language::Python => "Python 3",
            Language::JavaScript => "JavaScript",
        }
    }

    pub fn ext(self) -> &'static str {
        match self {
            Language::Python => "py",
            Language::JavaScript => "js",
        }
    }

    /// Asset file name inside `boilerplate/` and `solutions/`, e.g. `python.py`.
    pub fn asset_file(self) -> String {
        format!("{}.{}", self.name(), self.ext())
    }

    /// The user's working file, e.g. `solution.py`.
    pub fn solution_file(self) -> String {
        format!("solution.{}", self.ext())
    }

    /// The signature's snake_case name in this language's convention.
    pub fn function_name(self, snake: &str) -> String {
        match self {
            Language::Python => snake.to_string(),
            Language::JavaScript => camel(snake),
        }
    }
}

impl serde::Serialize for Language {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.name())
    }
}

impl<'de> serde::Deserialize<'de> for Language {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let s = String::deserialize(d)?;
        Language::ALL
            .iter()
            .copied()
            .find(|l| l.name() == s)
            .ok_or_else(|| {
                let names: Vec<&str> = Language::ALL.iter().map(|l| l.name()).collect();
                serde::de::Error::custom(format!(
                    "unknown language `{s}` (expected one of {})",
                    names.join(", ")
                ))
            })
    }
}

impl fmt::Display for Language {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

pub fn camel(snake: &str) -> String {
    let mut out = String::with_capacity(snake.len());
    let mut upper = false;
    for ch in snake.chars() {
        if ch == '_' {
            upper = !out.is_empty();
        } else if upper {
            out.extend(ch.to_uppercase());
            upper = false;
        } else {
            out.push(ch);
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn names() {
        assert_eq!(camel("two_sum"), "twoSum");
        assert_eq!(camel("product_except_self"), "productExceptSelf");
        assert_eq!(camel("trap"), "trap");
        assert_eq!(Language::parse("js"), Some(Language::JavaScript));
        assert_eq!(Language::JavaScript.asset_file(), "javascript.js");
    }
}
