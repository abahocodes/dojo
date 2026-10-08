You are given an integer array `nums` and a list of `queries`. Each query is a
pair `[l, r]` with `0 <= l <= r < len(nums)` and asks for

```
nums[l] + nums[l + 1] + ... + nums[r]
```

Return the answers as a list, in the same order as the queries. There can be
many queries, so aim for O(1) work per query after some preprocessing.

## Example 1

```
nums    = [4, -1, 6, 2, -5]
queries = [[0, 2], [1, 4], [3, 3]]
output  = [9, 2, 2]
```

## Example 2

```
nums    = [100000, 100000, 100000]
queries = [[0, 2], [2, 2]]
output  = [300000, 100000]
```

## Constraints

- `1 <= len(nums) <= 10^5`
- `1 <= len(queries) <= 10^5`
- `-10^5 <= nums[i] <= 10^5`
- `0 <= l <= r < len(nums)` for every query `[l, r]`
- Answers can exceed 32 bits (up to `10^10` in absolute value).
