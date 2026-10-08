# Approach: variable-size sliding window with a deficit counter

Keep `need[c]` = how many more copies of `c` the window needs (it may go negative
when the window holds surplus copies), and `missing` = the total number of
characters of `t` not yet matched.

1. Move `right` across `s`. When `need[s[right]] > 0`, that character fills a
   real gap, so decrement `missing`. Always decrement `need[s[right]]`.
2. While `missing == 0` the window `[left, right]` covers `t`. Record it if it is
   strictly shorter than the best so far, then drop `s[left]`: increment its
   `need`, and if that makes it positive the window is short one copy again, so
   increment `missing`. Advance `left`.

Every index enters and leaves the window at most once.

```python
from collections import Counter

def min_window(s, t):
    need = Counter(t)
    missing = len(t)
    best_start, best_len = 0, float("inf")
    left = 0
    for right, ch in enumerate(s):
        if need[ch] > 0:
            missing -= 1
        need[ch] -= 1
        while missing == 0:
            if right - left + 1 < best_len:
                best_start, best_len = left, right - left + 1
            out = s[left]
            need[out] += 1
            if need[out] > 0:
                missing += 1
            left += 1
    return "" if best_len == float("inf") else s[best_start:best_start + best_len]
```

## Complexity

- Time: O(len(s) + len(t)) — each pointer only moves forward.
- Space: O(k) where k is the alphabet size (at most 52 here).

## Pitfalls

- Duplicates in `t` matter: `t = "aa"` needs two `a`s in the window, so count
  characters instead of using a set.
- Track the deficit with one integer; comparing whole count maps on every step
  makes the solution O(n · k).
- Use a strict `<` when recording the best window, otherwise a later window of
  the same length replaces the leftmost one.
- Return `""` (not `None`) when no window exists, including when `t` is longer
  than `s`.
