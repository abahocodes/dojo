# Approach: adjacent run lengths

A valid substring is `k` copies of one character followed by `k` copies of the
other, so it sits across exactly one change point. If the run ending at that
point has length `a` and the run starting there has length `b`, the valid
substrings centred on it are the ones with `k = 1, ..., min(a, b)`.

So the answer is the sum of `min(a, b)` over all pairs of adjacent runs. Track
`prev_run` and `cur_run` while scanning; every time the character changes,
add `min(prev_run, cur_run)` and shift. Add the final pair after the loop.

```python
def count_binary_substrings(s: str) -> int:
    total = 0
    prev_run, cur_run = 0, 1
    for i in range(1, len(s)):
        if s[i] == s[i - 1]:
            cur_run += 1
        else:
            total += min(prev_run, cur_run)
            prev_run, cur_run = cur_run, 1
    return total + min(prev_run, cur_run)
```

## Complexity

- Time: O(n).
- Space: O(1).

## Pitfalls

- Counting distinct substrings instead of occurrences: `"1010"` has answer 3,
  not 2.
- Forgetting the last pair of runs after the loop ends.
- Brute-force checking every substring is O(n^2) or worse and is too slow for
  `n = 10^5`.
