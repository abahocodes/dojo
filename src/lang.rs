//! Languages a question can be solved in.

use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Language {
    Python,
    JavaScript,
    TypeScript,
    Java,
    Cpp,
    Go,
}

impl Language {
    pub const ALL: &[Language] = &[
        Language::Python,
        Language::JavaScript,
        Language::TypeScript,
        Language::Java,
        Language::Cpp,
        Language::Go,
    ];

    pub fn parse(s: &str) -> Option<Language> {
        match s.to_ascii_lowercase().as_str() {
            "python" | "python3" | "py" => Some(Language::Python),
            "javascript" | "js" | "node" => Some(Language::JavaScript),
            "typescript" | "ts" => Some(Language::TypeScript),
            "java" => Some(Language::Java),
            "cpp" | "c++" | "cxx" => Some(Language::Cpp),
            "go" | "golang" => Some(Language::Go),
            _ => None,
        }
    }

    /// Languages every question in the bank must have. The rest are
    /// optional until the bank is ported to them.
    pub const REQUIRED: &[Language] = &[Language::Python, Language::JavaScript];

    /// Compiled before running (a driver is generated around the solution).
    pub fn compiled(self) -> bool {
        matches!(self, Language::Java | Language::Cpp | Language::Go)
    }

    /// Stable identifier used in config, the database and asset file names.
    pub fn name(self) -> &'static str {
        match self {
            Language::Python => "python",
            Language::JavaScript => "javascript",
            Language::TypeScript => "typescript",
            Language::Java => "java",
            Language::Cpp => "cpp",
            Language::Go => "go",
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            Language::Python => "Python 3",
            Language::JavaScript => "JavaScript",
            Language::TypeScript => "TypeScript",
            Language::Java => "Java",
            Language::Cpp => "C++",
            Language::Go => "Go",
        }
    }

    pub fn ext(self) -> &'static str {
        match self {
            Language::Python => "py",
            Language::JavaScript => "js",
            Language::TypeScript => "ts",
            Language::Java => "java",
            Language::Cpp => "cpp",
            Language::Go => "go",
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

    /// The signature's snake_case name in this language's convention:
    /// snake_case in Python, camelCase everywhere else (as on LeetCode).
    pub fn function_name(self, snake: &str) -> String {
        match self {
            Language::Python => snake.to_string(),
            _ => camel(snake),
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
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case("two_sum", "twoSum")]
    #[case("product_except_self", "productExceptSelf")]
    #[case("trap", "trap")]
    #[case("_private", "private")]
    #[case("a_b_c", "aBC")]
    fn camel_cases(#[case] snake: &str, #[case] expected: &str) {
        assert_eq!(camel(snake), expected);
    }

    #[rstest]
    #[case("python", Some(Language::Python))]
    #[case("py", Some(Language::Python))]
    #[case("Python3", Some(Language::Python))]
    #[case("js", Some(Language::JavaScript))]
    #[case("node", Some(Language::JavaScript))]
    #[case("JavaScript", Some(Language::JavaScript))]
    #[case("ts", Some(Language::TypeScript))]
    #[case("Java", Some(Language::Java))]
    #[case("c++", Some(Language::Cpp))]
    #[case("cpp", Some(Language::Cpp))]
    #[case("golang", Some(Language::Go))]
    #[case("rust", None)]
    #[case("", None)]
    fn parses(#[case] input: &str, #[case] expected: Option<Language>) {
        assert_eq!(Language::parse(input), expected);
    }

    #[rstest]
    #[case(Language::Python, "python.py", "solution.py", "two_sum")]
    #[case(Language::JavaScript, "javascript.js", "solution.js", "twoSum")]
    #[case(Language::TypeScript, "typescript.ts", "solution.ts", "twoSum")]
    #[case(Language::Java, "java.java", "solution.java", "twoSum")]
    #[case(Language::Cpp, "cpp.cpp", "solution.cpp", "twoSum")]
    #[case(Language::Go, "go.go", "solution.go", "twoSum")]
    fn file_and_function_names(
        #[case] lang: Language,
        #[case] asset: &str,
        #[case] solution: &str,
        #[case] function: &str,
    ) {
        assert_eq!(lang.asset_file(), asset);
        assert_eq!(lang.solution_file(), solution);
        assert_eq!(lang.function_name("two_sum"), function);
    }
}
