# Approach: binary search on k with a two-pointer check

Deleting more characters can only break the subsequence, never repair it, so
the good values of `k` form a prefix `0, 1, ..., answer`. `k = 0` is good
because `p` is a subsequence of `s`. Binary search the largest good `k`.

To test a `k` quickly, precompute `removed_at[i]`: the step at which
position `i` is removed (`len(removable)` or more if never). After `k` steps
position `i` is gone exactly when `removed_at[i] < k`. The usual greedy
subsequence check then walks `s` once, skipping deleted positions and
advancing a pointer into `p` on every match.

```python
def maximum_removals(s, p, removable):
    removed_at = [len(removable)] * len(s)
    for step, i in enumerate(removable):
        removed_at[i] = step

    def survives(k):
        j = 0
        for i, ch in enumerate(s):
            if j < len(p) and removed_at[i] >= k and ch == p[j]:
                j += 1
        return j == len(p)

    lo, hi = 0, len(removable)
    while lo < hi:
        mid = (lo + hi + 1) // 2
        if survives(mid):
            lo = mid
        else:
            hi = mid - 1
    return lo
```

## Complexity

- Time: O(len(s) * log(len(removable))).
- Space: O(len(s)) for `removed_at`.

## Pitfalls

- Re-indexing after deletions: the indices in `removable` always refer to
  the original string. Marking positions as deleted (rather than actually
  erasing characters) keeps them stable.
- Building a fresh set of removed indices for every `mid` costs extra time
  and memory; the `removed_at` array answers "is it removed after `k` steps"
  in O(1).
- Searching `k` only up to `len(removable) - 1`: all removals can be
  possible.
