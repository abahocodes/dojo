# Approach: pigeonhole buckets

Let `lo` and `hi` be the minimum and maximum. The `n - 1` gaps of the sorted
order add up to `hi - lo`, so the largest gap is at least
`ceil((hi - lo) / (n - 1))`.

Choose a bucket width `size = max(1, (hi - lo) // (n - 1))`, which is no
larger than that bound, and put each value `v` into bucket
`(v - lo) // size`. Two values in the same bucket differ by less than `size`,
so they cannot form the largest gap (if they did, it would be smaller than the
lower bound). Hence the largest gap is always between the maximum of one
non-empty bucket and the minimum of the next non-empty bucket. Each bucket
only needs its minimum and maximum.

```python
def maximum_gap(nums):
    n = len(nums)
    if n < 2:
        return 0
    lo, hi = min(nums), max(nums)
    if lo == hi:
        return 0
    size = max(1, (hi - lo) // (n - 1))
    count = (hi - lo) // size + 1
    bucket_min = [None] * count
    bucket_max = [None] * count
    for v in nums:
        b = (v - lo) // size
        if bucket_min[b] is None or v < bucket_min[b]:
            bucket_min[b] = v
        if bucket_max[b] is None or v > bucket_max[b]:
            bucket_max[b] = v
    best = 0
    prev = lo
    for b in range(count):
        if bucket_min[b] is not None:
            best = max(best, bucket_min[b] - prev)
            prev = bucket_max[b]
    return best
```

Radix sort (base 256 or base 10, a fixed number of passes because values fit
in 30 bits) followed by a linear scan is another linear-time solution.

## Complexity

- Time: O(n). There are at most about `2n` buckets and each value is touched
  a constant number of times.
- Space: O(n) for the bucket bounds.

## Pitfalls

- A bucket width of 0 when `hi - lo < n - 1`: clamp it to at least 1.
- Comparing gaps inside a bucket, or skipping empty buckets incorrectly: the
  gap is from the previous *non-empty* bucket's maximum.
- All values equal (`lo == hi`): the answer is 0 and the width formula would
  divide by zero-sized ranges.
