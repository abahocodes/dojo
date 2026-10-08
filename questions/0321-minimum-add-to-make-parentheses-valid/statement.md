A string of parentheses is **valid** when it is empty, or is `(A)` for a valid
`A`, or is `AB` for valid `A` and `B`. In other words, every `(` has a
matching `)` after it and every `)` has a matching `(` before it.

You are given a string `s` made only of `(` and `)`. You may insert extra
parentheses anywhere in `s` (at the start, the end, or between any two
characters). Return the minimum number of insertions needed to make `s`
valid.

## Example 1

```
s      = "())"
output = 1     # insert "(" at the front: "(())"
```

## Example 2

```
s      = ")(("
output = 3     # one "(" in front, two ")" at the end: "()(())"
```

## Constraints

- `1 <= len(s) <= 10^5`
- `s` consists only of `(` and `)`.
