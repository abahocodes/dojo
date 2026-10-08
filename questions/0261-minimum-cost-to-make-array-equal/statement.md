You are given two integer arrays `nums` and `cost` of the same length `n`.
You may repeatedly pick an index `i` and either increase or decrease
`nums[i]` by `1`; each such step on index `i` costs `cost[i]`.

Return the minimum total cost needed to make every element of `nums` equal.

## Example 1

```
nums   = [2, 6, 3]
cost   = [3, 1, 2]
output = 6    # make everything 3: |2-3|*3 + |6-3|*1 + |3-3|*2 = 3 + 3 + 0
```

## Example 2

```
nums   = [4, 4, 4]
cost   = [5, 9, 1]
output = 0    # already equal
```

## Constraints

- `1 <= n <= 10^5`, `len(cost) == n`
- `1 <= nums[i], cost[i] <= 10^6`
- The answer is guaranteed to be at most `2^53`. Intermediate products such as
  `cost[i] * |nums[i] - x|` can exceed 32 bits, so use 64-bit arithmetic.
