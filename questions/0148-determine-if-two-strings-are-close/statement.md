Call two strings **close** if you can turn one into the other using the
following moves, as many times as you like and in any order:

- **Reposition:** exchange the characters at any two positions
  (`"bca"` can become `"acb"`).
- **Relabel:** pick two letters `x` and `y` that both appear in the string
  and turn every `x` into `y` and every `y` into `x` at the same time
  (`"xxyz"` can become `"yyxz"`).

Moves are applied to one string only. Given `word1` and `word2`, return
`true` if they are close, otherwise `false`.

## Example 1

```
word1  = "bccab"
word2  = "aabcb"
output = true    # relabel a<->c gives "baacb", then reposition
```

## Example 2

```
word1  = "aa"
word2  = "ab"
output = false   # 'b' never appears in word1, and no move can create it
```

## Constraints

- `1 <= len(word1), len(word2) <= 10^5`
- Both strings consist of lowercase English letters.
