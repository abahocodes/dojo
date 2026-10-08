# Approach: count by right end with three "last seen" indices

Fix the right end `i` of a subarray. A start `s <= i` gives a fixed-bound
subarray exactly when:

1. every element in `nums[s..i]` lies in `[min_k, max_k]`, i.e. `s` is after
   the last out-of-range index `bad`;
2. some element in `nums[s..i]` equals `min_k`, i.e. `s <= last_min`;
3. some element equals `max_k`, i.e. `s <= last_max`.

Those starts form the range `bad + 1 .. min(last_min, last_max)`, which has
`max(0, min(last_min, last_max) - bad)` elements. Initialising all three
indices to `-1` makes the formula give `0` until both bounds have been seen.

```python
def count_subarrays_fixed_bounds(nums, min_k, max_k):
    total = 0
    bad = last_min = last_max = -1
    for i, v in enumerate(nums):
        if v < min_k or v > max_k:
            bad = i
        if v == min_k:
            last_min = i
        if v == max_k:
            last_max = i
        total += max(0, min(last_min, last_max) - bad)
    return total
```

If `min_k > max_k`, every value is out of range, `bad` always equals `i`,
and the total stays 0 without a special case.

## Complexity

- Time: O(n), one pass.
- Space: O(1).

## Pitfalls

- Overflow: with `n = 10^5` the answer can reach about 5 * 10^9. Use a 64-bit
  accumulator in Java, C++ and Go.
- Using `if / elif` for the `min_k` and `max_k` checks: when
  `min_k == max_k` one element updates both indices.
- Forgetting to reset via `bad`: an out-of-range element makes the earlier
  `last_min` / `last_max` useless, which the `- bad` term handles because
  those indices are then `<= bad`.
- Counting only one subarray per right end. Every start in
  `bad+1 .. min(last_min, last_max)` gives a different valid subarray.
