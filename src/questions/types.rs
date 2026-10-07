use std::fmt;

use serde_json::Value;

/// A language-agnostic type used in question signatures.
///
/// Written in `meta.json` as strings: `int`, `float`, `bool`, `string`,
/// `ListNode`, `TreeNode`, and any of those followed by one or more `[]`.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(try_from = "String", into = "String")]
pub enum Type {
    Int,
    Float,
    Bool,
    String,
    ListNode,
    TreeNode,
    List(Box<Type>),
}

impl Type {
    pub fn parse(s: &str) -> Result<Type, String> {
        let s = s.trim();
        if let Some(inner) = s.strip_suffix("[]") {
            return Ok(Type::List(Box::new(Type::parse(inner)?)));
        }
        match s {
            "int" => Ok(Type::Int),
            "float" => Ok(Type::Float),
            "bool" => Ok(Type::Bool),
            "string" => Ok(Type::String),
            "ListNode" => Ok(Type::ListNode),
            "TreeNode" => Ok(Type::TreeNode),
            other => Err(format!("unknown type `{other}`")),
        }
    }

    /// Checks that a JSON value is a valid encoding of this type.
    /// `ListNode` is encoded as an int array, `TreeNode` as a level-order
    /// array of ints and nulls (LeetCode style).
    pub fn check(&self, v: &Value) -> Result<(), String> {
        let ok = match self {
            Type::Int => v.is_i64() || v.is_u64(),
            Type::Float => v.is_number(),
            Type::Bool => v.is_boolean(),
            Type::String => v.is_string(),
            Type::ListNode => v.as_array().is_some_and(|a| a.iter().all(|x| x.is_i64())),
            Type::TreeNode => v
                .as_array()
                .is_some_and(|a| a.iter().all(|x| x.is_i64() || x.is_null())),
            Type::List(inner) => {
                let Some(items) = v.as_array() else {
                    return Err(format!("expected {self}, got {}", short(v)));
                };
                for (i, item) in items.iter().enumerate() {
                    inner.check(item).map_err(|e| format!("[{i}]: {e}"))?;
                }
                true
            }
        };
        if ok {
            Ok(())
        } else {
            Err(format!("expected {self}, got {}", short(v)))
        }
    }

    pub fn uses(&self, t: &Type) -> bool {
        match self {
            Type::List(inner) => inner.uses(t),
            other => other == t,
        }
    }
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Type::Int => write!(f, "int"),
            Type::Float => write!(f, "float"),
            Type::Bool => write!(f, "bool"),
            Type::String => write!(f, "string"),
            Type::ListNode => write!(f, "ListNode"),
            Type::TreeNode => write!(f, "TreeNode"),
            Type::List(inner) => write!(f, "{inner}[]"),
        }
    }
}

impl schemars::JsonSchema for Type {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "Type".into()
    }

    fn json_schema(_: &mut schemars::SchemaGenerator) -> schemars::Schema {
        schemars::json_schema!({
            "type": "string",
            "description": "int, float, bool, string, ListNode, TreeNode, or any of those followed by one or more []",
            "pattern": "^(int|float|bool|string|ListNode|TreeNode)(\\[\\])*$"
        })
    }
}

impl TryFrom<String> for Type {
    type Error = String;
    fn try_from(s: String) -> Result<Self, Self::Error> {
        Type::parse(&s)
    }
}

impl From<Type> for String {
    fn from(t: Type) -> String {
        t.to_string()
    }
}

fn short(v: &Value) -> String {
    let s = v.to_string();
    if s.chars().count() > 40 {
        format!("{}…", s.chars().take(40).collect::<String>())
    } else {
        s
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn parses_nested_lists() {
        assert_eq!(
            Type::parse("int[][]").unwrap(),
            Type::List(Box::new(Type::List(Box::new(Type::Int))))
        );
        assert_eq!(Type::parse("int[][]").unwrap().to_string(), "int[][]");
        assert!(Type::parse("map").is_err());
    }

    #[test]
    fn checks_values() {
        let t = Type::parse("int[]").unwrap();
        assert!(t.check(&json!([1, 2, 3])).is_ok());
        assert!(t.check(&json!([1, "x"])).is_err());
        assert!(Type::TreeNode.check(&json!([1, null, 2])).is_ok());
        assert!(Type::ListNode.check(&json!([1, null])).is_err());
        assert!(Type::Float.check(&json!(1)).is_ok());
    }
}
