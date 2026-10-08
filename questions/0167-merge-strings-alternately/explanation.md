# Approach: one index, two optional appends

Walk an index `i` from 0 while it is inside either string. At each step append
`word1[i]` (if `i < len(word1)`) and then `word2[i]` (if `i < len(word2)`).
Once the shorter string is exhausted only the longer one contributes, which is
exactly "append the rest".

```python
def merge_alternately(word1: str, word2: str) -> str:
    out = []
    i = 0
    while i < len(word1) or i < len(word2):
        if i < len(word1):
            out.append(word1[i])
        if i < len(word2):
            out.append(word2[i])
        i += 1
    return "".join(out)
```

## Complexity

- Time: O(len(word1) + len(word2)).
- Space: O(len(word1) + len(word2)) for the result.

## Pitfalls

- Starting with `word2`. The first character always comes from `word1`.
- Stopping at the shorter length and forgetting the tail of the longer string.
