# Approach: extend to the last occurrence, cut as early as possible

A piece that contains letter `c` must reach at least `last[c]`, the final index
of `c`. So while scanning, the current piece must extend to the maximum
`last[...]` over all letters seen in it. As soon as the scan reaches that
point, no letter in the piece appears again, and cutting right there is both
legal and the earliest possible cut, which is what maximises the number of
pieces.

```python
def partition_labels(s):
    last = {ch: i for i, ch in enumerate(s)}
    sizes = []
    start = end = 0
    for i, ch in enumerate(s):
        end = max(end, last[ch])
        if i == end:
            sizes.append(i - start + 1)
            start = i + 1
    return sizes
```

## Complexity

- Time: O(n): two passes over the string.
- Space: O(1): at most 26 entries in `last`.

## Pitfalls

- Cutting at the last occurrence of the *first* letter only. A letter inside
  the piece can push the end further (`"abab"` is one piece, not `"aba"`).
- Returning the pieces themselves or their end indices instead of their
  lengths.
- Forgetting to reset `start` after each cut, which makes later lengths too
  long.
