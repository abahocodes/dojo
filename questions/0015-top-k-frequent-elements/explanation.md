# Approach: count, then bucket by frequency

1. Count each value with a hash map.
2. Every count is between `1` and `n = len(nums)`, so make `n + 1` buckets and
   drop each value into `buckets[count]`.
3. Walk the buckets from `n` down to `1`, collecting values until you have `k`.

```python
def top_k_frequent(nums, k):
    counts = {}
    for x in nums:
        counts[x] = counts.get(x, 0) + 1

    buckets = [[] for _ in range(len(nums) + 1)]
    for value, freq in counts.items():
        buckets[freq].append(value)

    result = []
    for freq in range(len(nums), 0, -1):
        for value in buckets[freq]:
            result.append(value)
            if len(result) == k:
                return result
    return result
```

**Alternatives:**

- Sort the distinct values by count: O(m log m) for `m` distinct values, and
  the simplest to write (`Counter(nums).most_common(k)`).
- A min-heap of size `k` keyed on count: O(n + m log k). Push each
  `(count, value)` and pop whenever the heap grows past `k`. This is the
  natural choice when `k` is small or the data is streamed.
- Quickselect on the counts: O(m) on average.

## Complexity

- Time: O(n) for counting and the bucket walk.
- Space: O(n) for the counts and buckets.

## Pitfalls

- Return the **values**, not their counts.
- Size the buckets as `len(nums) + 1`: a single value can appear `n` times.
- With a heap, use a **min**-heap of size `k`. You evict the smallest count,
  not the largest.
- Values can be negative, so don't use them as list indices without shifting.
