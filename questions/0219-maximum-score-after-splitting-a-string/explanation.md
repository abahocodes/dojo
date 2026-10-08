# Approach: one pass, updating the score

Start with the cut before the first character: the score would be the number
of `'1'`s in `s`. Moving the cut past character `c` changes the score by
`+1` if `c == '0'` (one more zero on the left) and by `-1` if `c == '1'` (one
fewer one on the right). Both parts must be non-empty, so only the cuts after
characters `0 .. len(s) - 2` count.

```python
def max_score_split(s):
    score = s.count("1")
    best = 0
    for c in s[:-1]:
        score += 1 if c == "0" else -1
        best = max(best, score)
    return best
```

## Complexity

- Time: O(n).
- Space: O(1).

## Pitfalls

- Allowing an empty part. For `"1111"`, taking all four ones on the right
  would give `4`, but the left part must contain at least one character, so
  the answer is `3`.
- Similarly, for `"0000"` the right part must be non-empty: the answer is
  `3`, not `4`.
- Recounting both parts for every cut is O(n²); fine at these bounds, but the
  running update is just as easy.
