You are given a string `s` of `0`s and `1`s. Count the non-empty substrings
(contiguous pieces) of `s` that

- contain the same number of `0`s and `1`s, and
- have all of their `0`s next to each other and all of their `1`s next to each
  other, like `"01"`, `"1100"` or `"000111"`.

Substrings that are equal but start at different positions are counted
separately.

## Example 1

```
s      = "0110011"
output = 5   # "01", "10", "1100", "01", "0011"
```

## Example 2

```
s      = "1010"
output = 3   # "10", "01", "10"
```

## Constraints

- `1 <= len(s) <= 10^5`
- `s[i]` is `'0'` or `'1'`.
