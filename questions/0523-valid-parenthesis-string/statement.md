You are given a string `s` made only of the characters `'('`, `')'` and
`'*'`. Each `'*'` is a wildcard: you may replace it with `'('`, with `')'`, or
with nothing at all, and each `'*'` is decided independently.

Return `true` if there is some way to replace the wildcards so that the result
is a balanced parenthesis string, and `false` otherwise. A string is balanced
when every `'('` is closed by a later `')'` and every `')'` closes an earlier
`'('`. The empty string is balanced.

## Example 1

```
s      = "(*))"
output = true
```

Turn the `'*'` into `'('` to get `"(())"`.

## Example 2

```
s      = "*)(("
output = false
```

The `'*'` can close the first `')'`, but nothing can close the two trailing
`'('`.

## Example 3

```
s      = "(**"
output = true
```

One `'*'` becomes `')'` and the other is dropped: `"()"`.

## Constraints

- `1 <= len(s) <= 100`
- `s[i]` is `'('`, `')'` or `'*'`.
