Call a string **wonderful** if at most one letter occurs an odd number of
times in it. For example, `"abab"` (every count even) and `"abcba"` (only `c`
is odd) are wonderful, while `"ab"` is not.

Given a string `word` made only of the letters `'a'` through `'j'`, return the
number of non-empty substrings of `word` that are wonderful. Substrings are
counted by position: equal substrings at different positions count
separately.

## Example 1

```
word   = "aab"
output = 5   # "a", "a", "b", "aa", "aab"; "ab" has two odd letters
```

## Example 2

```
word   = "jj"
output = 3   # "j", "j", "jj"
```

## Constraints

- `1 <= len(word) <= 10^5`
- `word` consists of lowercase letters from `'a'` to `'j'`.
- The answer can exceed 2^31 - 1; return it as a 64-bit integer.
