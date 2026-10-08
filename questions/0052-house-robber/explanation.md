# Approach: DP over prefixes with two variables

Let `best(i)` be the most cash obtainable from the first `i` houses. For house
`i - 1` there are two choices:

- skip it: `best(i - 1)`
- rob it: the house before it must be skipped, so `best(i - 2) + nums[i - 1]`

```
best(0) = 0,   best(1) = nums[0]
best(i) = max(best(i - 1), best(i - 2) + nums[i - 1])
```

Walk left to right and keep only the last two values:

```python
def rob(nums):
    prev, curr = 0, 0  # best for the prefix ending two houses back, one house back
    for x in nums:
        prev, curr = curr, max(curr, prev + x)
    return curr
```

## Complexity

- Time: O(n).
- Space: O(1).

## Pitfalls

- Comparing "all even indices" with "all odd indices" is wrong: in
  `[6, 1, 2, 9, 4]` the best set is houses 0 and 3, which mixes parities.
- Plain recursion that tries both choices at every house is exponential.
- With one house, the answer is that house; with all zeros, the answer is `0`.
- Update both variables at once (or via a temporary); overwriting `prev` before
  using it double-counts.
