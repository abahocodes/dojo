# Approach: sliding window with last-seen positions

Maintain a window `s[left..i]` that never contains a repeat. A map remembers the
most recent index of each character. When the new character `s[i]` was last
seen inside the window, move `left` just past that occurrence — every window
starting at or before it would contain the duplicate.

```python
def length_of_longest_substring(s):
    last = {}
    left = 0
    best = 0
    for i, ch in enumerate(s):
        if ch in last and last[ch] >= left:
            left = last[ch] + 1
        last[ch] = i
        best = max(best, i - left + 1)
    return best
```

## Complexity

- Time: O(n) — each index is processed once, and `left` only moves forward.
- Space: O(k) where `k` is the alphabet size (at most ~95 printable characters).

## Pitfalls

- Moving `left` backwards: the `last[ch] >= left` check matters for inputs like
  `"abba"` — when the final `a` arrives, its earlier copy is already outside the
  window.
- Empty string should return 0.
- Alternative: a set-based window that shrinks one step at a time from the left
  while `s[i]` is in the set. Also O(n), just with more steps.
