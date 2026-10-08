You are given a string `s` of uppercase English letters and an integer `k`.
You may perform at most `k` edits; each edit overwrites one position of `s` with
any uppercase letter you like.

Return the length of the longest contiguous block of identical letters you can
obtain after your edits.

## Example 1

```
s      = "BAAAB"
k      = 2
output = 5          # overwrite both B's with A -> "AAAAA"
```

## Example 2

```
s      = "ABCBBA"
k      = 1
output = 4          # overwrite the C with B -> "ABBBBA", which holds "BBBB"
```

## Constraints

- `1 <= len(s) <= 10^5`
- `s` contains only uppercase English letters.
- `0 <= k <= len(s)`
