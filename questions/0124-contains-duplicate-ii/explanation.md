# Approach: remember the last index of each value

For each position `i`, the only earlier occurrence of `nums[i]` that matters
is the latest one: it is the closest, so if it is more than `k` positions
back, so is every other. Keep a map `last` from value to the most recent
index where it appeared, check the distance, then overwrite the entry.

```python
def contains_nearby_duplicate(nums, k):
    last = {}
    for i, x in enumerate(nums):
        if x in last and i - last[x] <= k:
            return True
        last[x] = i
    return False
```

An equivalent alternative keeps a sliding set of the last `k` values: add
`nums[i]`, and remove `nums[i - k]` once the window grows past `k`
elements. A value already in the set is a nearby duplicate.

## Complexity

- Time: O(n) expected, one pass with hash-map operations.
- Space: O(n) for the map (O(min(n, k)) with the sliding-set variant).

## Pitfalls

- Storing the first index instead of the last: `[1, 0, 1, 1]` with `k = 1`
  has the 1s at positions 2 and 3, but comparing against position 0 misses
  them.
- `k = 0`: two different positions are never 0 apart, so the answer is
  always `false`.
- Treating `|i - j| < k` as the condition. The distance may equal `k`.
