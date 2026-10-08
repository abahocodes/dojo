You are given a list of strings `words`. Count the index pairs `(i, j)` with
`i < j` such that `words[i]` is **both** a prefix and a suffix of `words[j]`:
`words[j]` starts with `words[i]` and also ends with it. A string is a prefix
and a suffix of itself, so two equal words at `i < j` form a pair.

## Example 1

```
words  = ["ab", "abab", "aba", "ab"]
output = 2
```

The pairs are `(0, 1)` (`"abab"` starts and ends with `"ab"`) and `(0, 3)`.
`"aba"` starts with `"ab"` but ends with `"ba"`, so `(0, 2)` does not count.

## Example 2

```
words  = ["xy", "xyx", "yx", "x"]
output = 0
```

`"x"` is a prefix and a suffix of `"xyx"`, but it comes later in the list
(`i` must be less than `j`).

## Constraints

- `1 <= len(words) <= 50`
- `1 <= len(words[i]) <= 10`
- All strings consist of lowercase English letters.
