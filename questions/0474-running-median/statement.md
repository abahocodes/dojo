Numbers arrive one at a time, in the order given by `nums`. After each new
number arrives, record the median of every number received so far.

The median of an odd count of numbers is the middle one once they are sorted.
For an even count it is the average of the two middle numbers, which may be a
fraction (for example `2.5`).

Return the list of recorded medians; it has the same length as `nums`.
Answers within `10^-6` of the true values are accepted.

## Example 1

```
nums   = [5, 2, 8, 1]
output = [5.0, 3.5, 5.0, 3.5]
```

After each arrival the sorted prefix is `[5]`, `[2, 5]`, `[2, 5, 8]` and
`[1, 2, 5, 8]`.

## Example 2

```
nums   = [-3, -3, 10]
output = [-3.0, -3.0, -3.0]
```

## Constraints

- `1 <= len(nums) <= 5 * 10^4`
- `-10^5 <= nums[i] <= 10^5`
