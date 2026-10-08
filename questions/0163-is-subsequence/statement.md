Given two strings `s` and `t`, return `true` if `s` is a **subsequence** of
`t`, and `false` otherwise.

`s` is a subsequence of `t` if you can delete some characters of `t` (any
number, including none) without changing the order of the remaining ones
and obtain exactly `s`. The empty string is a subsequence of every string.

## Example 1

```
s      = "ace"
t      = "abcde"
output = true    # keep a, c, e
```

## Example 2

```
s      = "aec"
t      = "abcde"
output = false   # e comes after c in t
```

## Constraints

- `0 <= len(s) <= 100`
- `0 <= len(t) <= 10^4`
- `s` and `t` consist of lowercase English letters.
