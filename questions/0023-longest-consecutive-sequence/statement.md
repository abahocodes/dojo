You are given an unsorted list of integers `nums`. A **run** is a set of
integers `x, x + 1, x + 2, ..., x + m - 1` that all occur somewhere in `nums`
(in any positions, in any order); its length is `m`.

Return the length of the longest run. Repeated values count only once, and an
empty list has no run, so the answer is `0`.

## Example 1

```
nums   = [42, 5, 3, 41, 4, 40, 6]
output = 4           # 3, 4, 5, 6 are all present (40, 41, 42 is only 3 long)
```

## Example 2

```
nums   = [7, 7, 8, 7]
output = 2           # 7, 8 (duplicates of 7 don't make the run longer)
```

## Constraints

- `0 <= len(nums) <= 10^5`
- `-10^9 <= nums[i] <= 10^9`

**Follow-up:** sorting gives an O(n log n) answer. Can you find it in O(n)?
