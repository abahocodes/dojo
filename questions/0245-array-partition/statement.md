You are given an even-length list `nums` of `2n` integers. Split them into
`n` pairs, using every element in exactly one pair. Each pair scores the
**smaller** of its two numbers, and the total score is the sum over all pairs.

Return the largest total score any pairing can achieve.

## Example 1

```
nums   = [4, 1, 3, 2]
output = 4       # pairs (1, 2) and (3, 4): 1 + 3
```

## Example 2

```
nums   = [5, -2, 5, 8, 0, 1]
output = 4       # pairs (-2, 0), (1, 5), (5, 8): -2 + 1 + 5
```

## Constraints

- `1 <= n <= 10^4`, so `len(nums) == 2n` is between `2` and `2 * 10^4`
- `-10^4 <= nums[i] <= 10^4`
