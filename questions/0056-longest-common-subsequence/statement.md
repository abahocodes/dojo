A **subsequence** of a string is what remains after deleting any number of its
characters (possibly none) without reordering the rest. For example, `"tnk"`
is a subsequence of `"thinker"`, but `"knt"` is not.

Given two strings `a` and `b`, return the **length of the longest string that
is a subsequence of both**. If they share no characters, return `0`.

## Example 1

```
a      = "thinker"
b      = "tinkle"
output = 5      # "tinke"
```

## Example 2

```
a      = "sky"
b      = "blue"
output = 0      # no character appears in both
```

## Constraints

- `1 <= len(a), len(b) <= 500`
- `a` and `b` contain only lowercase English letters
