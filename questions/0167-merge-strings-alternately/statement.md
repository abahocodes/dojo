Given two strings `word1` and `word2`, build a new string by taking
characters from them in turn: the first character of `word1`, then the first
of `word2`, then the second of `word1`, the second of `word2`, and so on.

When one string runs out of characters, append the rest of the other string
unchanged.

## Example 1

```
word1  = "dog"
word2  = "cat"
output = "dcoagt"
```

## Example 2

```
word1  = "ab"
word2  = "wxyz"
output = "awbxyz"   # word1 runs out, then "yz" is appended
```

## Constraints

- `1 <= len(word1), len(word2) <= 100`
- Both strings consist of lowercase English letters.
