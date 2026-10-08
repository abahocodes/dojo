You are given an integer array `nums` and an integer `target`. Pick three
elements at **three different positions** of `nums` and add them up. Among all
such triples, find the one whose sum is nearest to `target` (smallest absolute
difference) and return **that sum**.

The inputs are chosen so that the nearest sum is unique: you will never have
to choose between `target - d` and `target + d`.

## Example 1

```
nums   = [-2, 5, 1, -6]
target = 3
output = 4      # -2 + 5 + 1 = 4 is off by 1; the other triples give -3, -7 and 0
```

## Example 2

```
nums   = [1, 1, 1, 0]
target = -100
output = 2      # the smallest possible sum is 0 + 1 + 1
```

## Constraints

- `3 <= len(nums) <= 500`
- `-1000 <= nums[i] <= 1000`
- `-10^4 <= target <= 10^4`
- Exactly one sum is nearest to `target`.
