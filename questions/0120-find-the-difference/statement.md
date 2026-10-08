String `t` was made by taking string `s`, adding **one** extra lowercase
letter, and then shuffling the result. Return the added letter as a
one-character string.

The extra letter may be a letter that already appears in `s`.

## Example 1

```
s      = "map"
t      = "pmax"
output = "x"
```

## Example 2

```
s      = ""
t      = "q"
output = "q"
```

## Constraints

- `0 <= len(s) <= 1000`
- `len(t) == len(s) + 1`
- Both strings contain only lowercase English letters, and `t` is a shuffle of
  `s` plus one letter.
