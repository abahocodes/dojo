use std::fmt;

use serde_json::Value;

/// A language-agnostic type used in question signatures.
///
/// Written in `meta.json` as strings: `int`, `long`, `float`, `bool`,
/// `string`, `ListNode`, `TreeNode`, and any of those followed by one or more
/// `[]`.
///
/// `int` is 32-bit (Java/C++ `int`); `long` is 64-bit for values that don't
/// fit, limited to ±(2^53 − 1) so JavaScript represents them exactly. Node
/// values are `int`s.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(try_from = "String", into = "String")]
pub enum Type {
    Int,
    Long,
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
            "long" => Ok(Type::Long),
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
            Type::Int => is_int(v),
            Type::Long => v
                .as_i64()
                .is_some_and(|n| n.unsigned_abs() <= MAX_SAFE_INTEGER),
            Type::Float => v.is_number(),
            Type::Bool => v.is_boolean(),
            Type::String => v.is_string(),
            Type::ListNode => v.as_array().is_some_and(|a| a.iter().all(is_int)),
            Type::TreeNode => v
                .as_array()
                .is_some_and(|a| a.iter().all(|x| is_int(x) || x.is_null())),
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
        } else if matches!(self, Type::Int | Type::ListNode | Type::TreeNode)
            && v.as_i64().is_some_and(|n| i32::try_from(n).is_err())
        {
            Err(format!("{} doesn't fit a 32-bit int; use `long`", short(v)))
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
            Type::Long => write!(f, "long"),
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
            "description": "int (32-bit), long (64-bit), float, bool, string, ListNode, TreeNode, or any of those followed by one or more []",
            "pattern": "^(int|long|float|bool|string|ListNode|TreeNode)(\\[\\])*$"
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

/// Largest integer JavaScript represents exactly (2^53 − 1).
const MAX_SAFE_INTEGER: u64 = (1 << 53) - 1;

fn is_int(v: &Value) -> bool {
    v.as_i64().is_some_and(|n| i32::try_from(n).is_ok())
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
    use rstest::rstest;
    use serde_json::{Value, json};

    use super::*;

    fn list(t: Type) -> Type {
        Type::List(Box::new(t))
    }

    #[rstest]
    #[case("int", Type::Int)]
    #[case("long", Type::Long)]
    #[case("float", Type::Float)]
    #[case("bool", Type::Bool)]
    #[case("string", Type::String)]
    #[case("ListNode", Type::ListNode)]
    #[case("TreeNode", Type::TreeNode)]
    #[case("int[]", list(Type::Int))]
    #[case("int[][]", list(list(Type::Int)))]
    #[case(" string[] ", list(Type::String))]
    #[case("TreeNode[]", list(Type::TreeNode))]
    fn parses_and_round_trips(#[case] input: &str, #[case] expected: Type) {
        let parsed = Type::parse(input).unwrap();
        assert_eq!(parsed, expected);
        assert_eq!(parsed.to_string(), input.trim());
    }

    #[rstest]
    #[case("map")]
    #[case("Int")]
    #[case("[]")]
    #[case("")]
    fn rejects_unknown_types(#[case] input: &str) {
        assert!(Type::parse(input).is_err());
    }

    #[rstest]
    #[case("int", json!(3), true)]
    #[case("int", json!(3.5), false)]
    #[case("int", json!(2147483647), true)]
    #[case("int", json!(-2147483648), true)]
    #[case("int", json!(2147483648_i64), false)]
    #[case("long", json!(2147483648_i64), true)]
    #[case("long", json!(9007199254740991_i64), true)]
    #[case("long", json!(9007199254740992_i64), false)]
    #[case("long", json!(1.5), false)]
    #[case("int[]", json!([1, 3000000000_i64]), false)]
    #[case("ListNode", json!([1, 3000000000_i64]), false)]
    #[case("float", json!(1), true)]
    #[case("float", json!(1.5), true)]
    #[case("bool", json!(true), true)]
    #[case("bool", json!(1), false)]
    #[case("string", json!("x"), true)]
    #[case("int[]", json!([1, 2, 3]), true)]
    #[case("int[]", json!([1, "x"]), false)]
    #[case("int[]", json!([]), true)]
    #[case("int[][]", json!([[1], [2, 3]]), true)]
    #[case("int[][]", json!([1, 2]), false)]
    #[case("ListNode", json!([1, 2]), true)]
    #[case("ListNode", json!([1, null]), false)]
    #[case("TreeNode", json!([1, null, 2]), true)]
    #[case("TreeNode", json!([]), true)]
    #[case("TreeNode", json!({"val": 1}), false)]
    fn checks_values(#[case] ty: &str, #[case] value: Value, #[case] valid: bool) {
        assert_eq!(Type::parse(ty).unwrap().check(&value).is_ok(), valid);
    }

    #[rstest]
    #[case("ListNode[]", Type::ListNode, true)]
    #[case("int[][]", Type::Int, true)]
    #[case("int[]", Type::TreeNode, false)]
    fn finds_nested_types(#[case] ty: &str, #[case] inner: Type, #[case] uses: bool) {
        assert_eq!(Type::parse(ty).unwrap().uses(&inner), uses);
    }
}
