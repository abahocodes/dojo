The string `s` uses only the letters `a`, `b` and `c`. Count its substrings
(contiguous, non-empty) that contain **each** of the three letters at least
once. Substrings at different positions are counted separately even if their
text is the same.

## Example 1

```
s      = "abcabc"
output = 10   # e.g. "abc" (3 times), "abca", "bcab", "abcabc", ...
```

## Example 2

```
s      = "aaacb"
output = 3    # "aaacb", "aacb", "acb"
```

## Constraints

- `3 <= len(s) <= 5 * 10^4`
- `s` consists only of the characters `'a'`, `'b'` and `'c'`.
