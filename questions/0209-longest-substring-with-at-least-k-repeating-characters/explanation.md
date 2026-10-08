# Approach: sliding window per number of distinct letters

A window being balanced is not monotone in its length, so a single sliding
window does not work directly. The trick is to fix one more parameter: `t`,
the exact number of distinct letters we allow in the window. For a fixed `t`
the rule "shrink while the window has more than `t` distinct letters" is
monotone, so the usual two pointers apply.

For each `t` in `1..26`, maintain letter counts, `unique` (distinct letters in
the window) and `at_least` (letters whose count is at least `k`). Whenever
`unique == at_least`, every letter present reaches `k` and the window is a
candidate. The longest balanced substring has some number `t` of distinct
letters, and the pass for that `t` finds it.

```python
def longest_substring_k_repeating(s, k):
    best = 0
    for limit in range(1, 27):
        count = [0] * 26
        left = 0
        unique = 0
        at_least = 0
        for right, ch in enumerate(s):
            c = ord(ch) - 97
            if count[c] == 0:
                unique += 1
            count[c] += 1
            if count[c] == k:
                at_least += 1
            while unique > limit:
                d = ord(s[left]) - 97
                if count[d] == k:
                    at_least -= 1
                count[d] -= 1
                if count[d] == 0:
                    unique -= 1
                left += 1
            if unique == at_least:
                best = max(best, right - left + 1)
    return best
```

An equally good alternative is divide and conquer: any letter with fewer than
`k` occurrences in the current range splits it into independent pieces;
a range with no such letter is itself balanced.

## Complexity

- Time: O(26 * n).
- Space: O(26).

## Pitfalls

- A single unconstrained sliding window: shrinking can make an unbalanced
  window balanced and vice versa, so greedy moves miss answers.
- `k` larger than `len(s)`: no letter can qualify and the answer is `0`.
- Recursive divide and conquer on long strings can recurse deeply; splitting
  on *all* rare letters at once keeps the depth at most 26.
