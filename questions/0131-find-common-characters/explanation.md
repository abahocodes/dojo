# Approach: per-letter minimum counts

A letter can appear in the answer as many times as it appears in the word
that has the fewest copies of it. So for each of the 26 letters, take the
minimum of its count over all words. Emitting the letters from `'a'` to
`'z'` gives alphabetical order for free.

```python
def common_chars(words):
    common = [float("inf")] * 26
    for w in words:
        freq = [0] * 26
        for c in w:
            freq[ord(c) - 97] += 1
        common = [min(a, b) for a, b in zip(common, freq)]
    return [chr(97 + i) for i in range(26) for _ in range(common[i])]
```

## Complexity

- Time: O(total length of all words + 26 * number of words).
- Space: O(1): two arrays of 26 counts (plus the output).

## Pitfalls

- Intersecting sets of letters loses multiplicity: `"moon"` and `"noon"`
  share two `'o'`s, not one.
- Initializing the minimums to 0 instead of "infinity" (or to the first
  word's counts) makes every result empty.
- Returning letters in order of first appearance instead of alphabetical
  order.
