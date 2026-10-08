# Approach: last occurrence of each letter

Fix the right end `j`. A substring `s[i..j]` contains every letter exactly when
each letter's most recent occurrence at or before `j` is at index `i` or later.
So the valid starts are `0, 1, ..., m` where `m` is the smallest of the three
last-seen indices, giving `m + 1` substrings. Initialise every last index to
`-1` so that the count is `0` until all three letters have been seen.

```python
def number_of_substrings(s):
    last = [-1, -1, -1]
    total = 0
    for i, ch in enumerate(s):
        last[ord(ch) - ord("a")] = i
        total += min(last) + 1
    return total
```

The same count also falls out of a classic two-pointer window: for each left
end, find the first right end that completes the set, and every longer
substring also counts.

## Complexity

- Time: O(n), one pass.
- Space: O(1).

## Pitfalls

- Counting distinct substrings instead of positions: `"abcabc"` contains `"abc"`
  twice and both count.
- Enumerating all substrings is O(n^2), too slow for `5 * 10^4`.
- The answer can reach about `n^2 / 2 = 1.25 * 10^9`, close to the 32-bit
  limit but still within it.
