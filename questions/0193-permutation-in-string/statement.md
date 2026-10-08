Given two lowercase strings `s1` and `s2`, decide whether `s2` contains a
contiguous substring that uses exactly the same letters as `s1`, each the same
number of times, in any order (a rearrangement of `s1`).

Return `true` if such a substring exists, otherwise `false`. If `s1` is longer
than `s2` the answer is `false`.

## Example 1

```
s1     = "tea"
s2     = "sweatier"
output = true     # "eat" starts at index 2
```

## Example 2

```
s1     = "loop"
s2     = "lopsolo"
output = false    # no window of length 4 has two o's, one l and one p
```

## Constraints

- `1 <= len(s1), len(s2) <= 10^4`
- `s1` and `s2` consist of lowercase English letters
