You are given a list of positive integers `nums`, each one a candidate side
length. Pick three of them, at three **different positions** in the list, that
can be the sides of a real triangle: one with positive area, so the longest
side must be strictly shorter than the sum of the other two.

Return the largest perimeter (sum of the three sides) any such choice can
reach. If no three values form a triangle with positive area, return `0`.

## Example 1

```
nums   = [3, 9, 4, 6, 2]
output = 19      # sides 9, 6, 4: 6 + 4 = 10 > 9
```

## Example 2

```
nums   = [1, 5, 2, 10]
output = 0       # every triple is too flat, e.g. 1 + 2 is not > 5
```

## Constraints

- `3 <= len(nums) <= 10^4`
- `1 <= nums[i] <= 10^6`
- Equal values at different positions may both be used.
