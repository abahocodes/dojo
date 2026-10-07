You are given a string `s` made up only of the six characters `(`, `)`, `[`, `]`,
`{` and `}`.

Decide whether the brackets are **balanced**: every opening bracket must be
closed by a bracket of the same kind, closings must happen in the reverse order
of the openings (inner pairs close before outer ones), and no closing bracket
may appear without a matching opener before it.

Return `true` if `s` is balanced, otherwise `false`.

## Example 1

```
s      = "{[()()]}"
output = true
```

## Example 2

```
s      = "[(])"
output = false     # the ( is still open when ] arrives
```

## Constraints

- `1 <= len(s) <= 10^4`
- `s` contains only the characters `()[]{}`.
