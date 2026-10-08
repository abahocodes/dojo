# Approach: sliding window with character counts

A substring with at most `k` distinct characters stays valid if you shrink it,
so the longest valid substring ending at each position can be found with a
window. Grow the window to the right. When the window holds more than `k`
distinct characters, move its left edge until it holds `k` again. Since the
alphabet is printable ASCII, a 128-entry array of counts is enough.

```python
def length_of_longest_substring_k_distinct(s, k):
    count = [0] * 128
    distinct = left = best = 0
    for right, ch in enumerate(s):
        c = ord(ch)
        if count[c] == 0:
            distinct += 1
        count[c] += 1
        while distinct > k:
            d = ord(s[left])
            count[d] -= 1
            if count[d] == 0:
                distinct -= 1
            left += 1
        best = max(best, right - left + 1)
    return best
```

## Complexity

- Time: O(n). Each index enters and leaves the window once.
- Space: O(1): a fixed array of 128 counts.

## Pitfalls

- `k = 0`: the window shrinks to empty every time and the answer is 0. Make
  sure the loop does not read past `right`.
- Keeping `distinct` in sync: increment only when a count goes 0 -> 1 and
  decrement only when it goes 1 -> 0.
- `k` larger than the number of distinct characters in `s`: the answer is
  `len(s)`.
