# Approach: greedy range of open counts

Read the string left to right and think about the number of `'('` that are
still waiting to be closed. Without wildcards this is one number. With them,
different choices give different counts, so track the **range** of counts
that some choice can reach: `lo` (fewest open) to `hi` (most open).

- `'('`: both ends go up by one.
- `')'`: both ends go down by one.
- `'*'`: as `')'` it lowers the count, as `'('` it raises it, as nothing it
  keeps it, so `lo -= 1` and `hi += 1`.

Two checks keep the range honest:

- If `hi < 0`, even treating every wildcard as `'('` leaves a `')'` with
  nothing to close. No choice can recover: return `false`.
- If `lo < 0`, that particular choice closed something that wasn't open.
  Discard it by clamping `lo` to `0`. Every value between `lo` and `hi` is
  reachable (changing one wildcard's choice moves the count by one), so the
  range stays contiguous.

At the end, the string can be balanced exactly when `0` is in the range,
which (since `lo >= 0`) means `lo == 0`.

```python
def check_valid_string(s):
    lo = hi = 0
    for ch in s:
        if ch == "(":
            lo += 1
            hi += 1
        elif ch == ")":
            lo -= 1
            hi -= 1
        else:
            lo -= 1
            hi += 1
        if hi < 0:
            return False
        lo = max(lo, 0)
    return lo == 0
```

**Alternative (DP):** `dp[i][k]` = "can the first `i` characters leave
exactly `k` open?" It is O(n^2) time and makes the reasoning explicit; the
greedy above is that DP with the observation that the true set is always an
interval. A two-stack approach (indices of `'('` and of `'*'`, matching `')'`
greedily, then pairing leftover `'('` with later `'*'`) is also O(n).

## Complexity

- Time: O(n), one pass.
- Space: O(1).

## Pitfalls

- Forgetting to clamp `lo`: for `"(*)"` the unclamped `lo` ends at `-1`, so
  `lo == 0` says false, yet dropping the `'*'` gives `"()"`.
- Clamping only at the end and testing `lo <= 0 <= hi`: for `"*)("` the
  unclamped range ends at `-1..1`, which contains 0, but the last `'('` can
  never be closed. A negative count is an invalid prefix, so it must be
  discarded as soon as it appears.
- Skipping the `hi < 0` check: `"())"` must fail at the third character,
  even though later characters could push the count back up.
- Treating `'*'` as only `'('` or `')'`: it can also be empty, which is why
  odd-length strings can still be valid (`"(*"` is fine).
