You are given a string `text` and a list of distinct strings `words`. Find
every pair of indices `[i, j]` such that the slice of `text` from index `i` to
index `j`, **inclusive** of both ends, is exactly one of the words.

Return all such pairs sorted by `i`, and pairs with the same `i` by `j`, both
ascending. Words may overlap each other in `text`, and one word may appear
several times. Return `[]` when there are no matches.

## Example 1

```
text   = "thecatsatonthemat"
words  = ["cat", "at", "the", "mat", "dog"]
output = [[0, 2], [3, 5], [4, 5], [7, 8], [11, 13], [14, 16], [15, 16]]
```

`"cat"` is `text[3..5]` and `"at"` inside it is `text[4..5]`.

## Example 2

```
text   = "aaab"
words  = ["a", "aa", "ab"]
output = [[0, 0], [0, 1], [1, 1], [1, 2], [2, 2], [2, 3]]
```

## Constraints

- `1 <= len(text) <= 100`
- `1 <= len(words) <= 20`
- `1 <= len(words[i]) <= 50`
- All strings consist of lowercase English letters, and the words are
  distinct.
