Given two lowercase strings `s` and `p`, find every position where a
rearrangement of `p` appears in `s`. Formally, return every index `i` such that
the substring of `s` of length `len(p)` starting at `i` contains exactly the
same letters as `p`, each the same number of times.

Return the indices in **increasing order**. If there are none (including when
`p` is longer than `s`), return an empty array.

## Example 1

```
s      = "tacocatcat"
p      = "cat"
output = [0, 4, 5, 6, 7]    # "tac", "cat", "atc", "tca", "cat"
```

## Example 2

```
s      = "aaa"
p      = "aaaa"
output = []
```

## Constraints

- `1 <= len(s), len(p) <= 3 * 10^4`
- `s` and `p` consist of lowercase English letters
