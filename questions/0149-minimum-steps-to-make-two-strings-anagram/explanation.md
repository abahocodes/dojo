# Approach: count the shortfall

For each letter, `t` can keep at most `min(count_s, count_t)` of its copies.
Every other character of `t` has to be overwritten, and one overwrite can
supply exactly one letter that `t` is missing. Because both strings have the
same length, the number of surplus characters in `t` equals the number of
missing ones, so the answer is

    sum over letters of max(0, count_s[c] - count_t[c])

A single array of 26 counters suffices: add for `s`, subtract for `t`, then
sum the positive entries.

```python
def min_steps(s, t):
    diff = [0] * 26
    for ch in s:
        diff[ord(ch) - 97] += 1
    for ch in t:
        diff[ord(ch) - 97] -= 1
    return sum(d for d in diff if d > 0)
```

## Complexity

- Time: O(n), one pass over each string.
- Space: O(1), 26 counters.

## Pitfalls

- Summing absolute differences counts every change twice (once as a surplus
  in `t` and once as a shortage). Sum only one side, or halve the total.
- Comparing position by position: `"ab"` and `"ba"` need 0 steps, not 2.
