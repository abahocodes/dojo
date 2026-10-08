You are given a list `words` of lowercase strings (it may contain repeats).

For any string `t`, define its **score** as the number of entries of `words`
that start with `t` (a word counts as starting with itself). Repeated entries
are counted once per occurrence.

For each `words[i]`, add up the scores of all of its **non-empty prefixes**
(including `words[i]` itself). Return these totals as a list in the same order
as `words`.

## Example 1

```
words  = ["car", "cat", "dog", "ca"]
output = [7, 7, 3, 6]
```

For `"car"`: `"c"` scores 3 (`car`, `cat`, `ca`), `"ca"` scores 3, `"car"`
scores 1, for a total of 7. For `"dog"` every prefix scores 1.

## Example 2

```
words  = ["ab", "ab", "b"]
output = [4, 4, 1]
```

## Constraints

- `1 <= len(words) <= 1000`
- `1 <= len(words[i]) <= 1000`
- `words[i]` consists of lowercase English letters.
