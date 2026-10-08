# Approach: sliding window with a running best frequency

A window can be turned into one repeated letter using
`length - max_count` edits, where `max_count` is the frequency of its most common
letter. So we want the longest window with `length - max_count <= k`.

Extend the window to the right one letter at a time and update `max_freq`, the
highest count any letter has reached. If the window is now too expensive, slide
the left edge forward by one. The window length therefore never shrinks: it either
grows or shifts, and its final size is the answer.

`max_freq` is allowed to be stale (higher than the true maximum of the current
window). That is safe: the window only grows when some letter's count exceeds the
stale value, which is exactly when a longer valid window exists.

```python
def character_replacement(s, k):
    counts = [0] * 26
    max_freq = 0
    left = 0
    for right, ch in enumerate(s):
        idx = ord(ch) - 65
        counts[idx] += 1
        max_freq = max(max_freq, counts[idx])
        if right - left + 1 - max_freq > k:
            counts[ord(s[left]) - 65] -= 1
            left += 1
    return len(s) - left
```

## Complexity

- Time: O(n) — each character is added once and removed at most once.
- Space: O(1) — 26 counters.

## Pitfalls

- Recomputing the true maximum count on every shrink is correct but costs an
  extra factor of 26; it is not needed.
- The answer is the final window size `len(s) - left`, not a value you must
  re-derive from the counts.
- With `k >= len(s)` the whole string works; make sure your loop still returns
  `len(s)`.
