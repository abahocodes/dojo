You stand on index 0 of a list of non-negative integers `nums`. From index `i` you
may jump forward by any distance from 1 up to `nums[i]`.

The input is built so that the last index can always be reached. Return the
**fewest jumps** needed to land on the last index.

## Example 1

```
nums   = [1, 4, 1, 1, 1, 2]
output = 2      # 0 -> 1 -> 5
```

## Example 2

```
nums   = [1, 1, 1, 1]
output = 3      # each step only moves one index forward
```

## Constraints

- `1 <= len(nums) <= 10^4`
- `0 <= nums[i] <= 1000`
- The last index is reachable from index 0.
