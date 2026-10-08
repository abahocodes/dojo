# Approach: count the complement

Rearranging `j - i == nums[j] - nums[i]` gives `nums[i] - i == nums[j] - j`.
So a pair is good exactly when both indices share the key `nums[x] - x`.
Scan the array, and for every index `j` add the number of earlier indices
with the same key to the count of good pairs. Every pair is either good or
bad, so the answer is `n * (n - 1) / 2 - good`.

```python
def count_bad_pairs(nums):
    seen = {}
    good = 0
    for j, x in enumerate(nums):
        key = x - j
        good += seen.get(key, 0)
        seen[key] = seen.get(key, 0) + 1
    n = len(nums)
    return n * (n - 1) // 2 - good
```

## Complexity

- Time: O(n) expected.
- Space: O(n) for the map of keys.

## Pitfalls

- 32-bit overflow: with `n = 10^5` there are almost `5 * 10^9` pairs. Use
  `long` / `long long` / 64-bit for the total and the good-pair count, and
  compute `n * (n - 1)` in 64 bits.
- Counting bad pairs directly: a bad pair has no simple key, so the
  subtraction is what makes this linear.
- Using `nums[i] + i` instead of `nums[i] - i`: the rearrangement must keep
  each index with its own value.
