# Approach: first occurrence of each prefix sum

Let `prefix(i)` be the sum of `nums[0..i]`, with `prefix(-1) = 0`. The subarray
`nums[j+1..i]` sums to `prefix(i) - prefix(j)`, so it equals `k` exactly when
`prefix(j) = prefix(i) - k`. Its length is `i - j`, so for each `i` we want the
smallest such `j`.

Keep a map from every prefix value to the first index where it appeared. Look
up `prefix - k` before inserting the current prefix, and never overwrite an
existing entry.

```python
def max_sub_array_len(nums, k):
    first = {0: -1}
    prefix = 0
    best = 0
    for i, x in enumerate(nums):
        prefix += x
        j = first.get(prefix - k)
        if j is not None:
            best = max(best, i - j)
        if prefix not in first:
            first[prefix] = i
    return best
```

Prefix sums can reach `±2 * 10^9` and `prefix - k` up to about `3 * 10^9`, past
the 32-bit range, so the Java and C++ solutions use 64-bit sums and keys.

## Complexity

- Time: O(n) expected (hash-map operations).
- Space: O(n) for the map.

## Pitfalls

- Overwriting the stored index of a repeated prefix value. That keeps the
  latest index and produces shorter subarrays.
- Forgetting `0 -> -1`, which misses subarrays that start at index `0`.
- Using a sliding window, which only works when every element is positive.
- 32-bit overflow in the prefix sum or in `prefix - k`.
