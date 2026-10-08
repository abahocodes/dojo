A compressed string uses the notation `k[body]` to stand for `body` written
out `k` times in a row, where `k` is a positive integer written in decimal.
Bodies may themselves contain further `k[...]` groups, nested to any depth.
Letters that are not inside any group are copied unchanged.

Given a well-formed compressed string `s`, return the fully expanded string.

## Example 1

```
s      = "2[ab]c"
output = "ababc"
```

## Example 2

```
s      = "x3[y2[z]]w"
output = "xyzzyzzyzzw"   # "2[z]" -> "zz", then "3[yzz]" -> "yzzyzzyzz"
```

## Constraints

- `1 <= len(s) <= 100`
- `s` consists of lowercase English letters, digits, `[` and `]`.
- `s` is well formed: every number is immediately followed by `[`, brackets
  are matched, every `[` is preceded by a number, and digits appear only as
  repeat counts (never as text to copy).
- Every repeat count `k` satisfies `1 <= k <= 300` and has no leading zeros.
- The expanded string has length at most `10^5`.
