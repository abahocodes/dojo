# Approach: prefix sums + hash map

Let `running` be the sum of everything up to and including the current element.
A stretch that ends here sums to `k` exactly when the prefix just before its start
equals `running - k`. So if we know how many earlier prefixes had each value, we
can count all stretches ending at the current position in O(1).

Seed the map with `{0: 1}` — the empty prefix — so stretches starting at index 0
are counted. Look up before inserting, so a stretch is never empty.

```python
def subarray_sum(nums, k):
    seen = {0: 1}
    running = 0
    count = 0
    for x in nums:
        running += x
        count += seen.get(running - k, 0)
        seen[running] = seen.get(running, 0) + 1
    return count
```

## Complexity

- Time: O(n) — one pass, O(1) average per map operation.
- Space: O(n) for the map of prefix sums.

## Pitfalls

- A sliding window only works when every value is positive; zeros and negatives
  break the "grow right, shrink left" logic.
- Forgetting the `{0: 1}` seed misses every stretch that starts at index 0.
- Store **counts**, not a set: the same prefix sum can appear many times (think of
  runs of zeros), and each occurrence is a different starting point.
- Inserting `running` before the lookup would count an empty stretch when `k == 0`.
