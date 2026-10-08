You are given a list of integers `nums` (which may include negatives and zeros)
and an integer `k`. Count the **contiguous, non-empty** stretches of `nums`
whose elements add up to exactly `k`.

Stretches are told apart by where they start and end, so two stretches with the
same values at different positions are both counted.

## Example 1

```
nums   = [1, 2, 1, 2, 1]
k      = 3
output = 4      # [1, 2] twice and [2, 1] twice
```

## Example 2

```
nums   = [3, -3, 3, 0]
k      = 0
output = 4      # [3, -3], [-3, 3], [-3, 3, 0] and [0]
```

## Constraints

- `1 <= len(nums) <= 2 * 10^4`
- `-1000 <= nums[i] <= 1000`
- `-10^7 <= k <= 10^7`

**Follow-up:** checking every start and end is O(n²). Can you count them in one pass?
