A teacher has the test scores of a class in the array `nums`. She wants to
pick exactly `k` of the scores (any `k` positions, not necessarily adjacent)
so that the picked scores are as close together as possible.

Return the smallest possible value of `(highest picked score) - (lowest picked
score)`. When `k == 1` the answer is `0`.

## Example 1

```
nums   = [31, 4, 18, 27, 9, 22]
k      = 3
output = 9    # pick 18, 22, 27
```

## Example 2

```
nums   = [100, 7, 100, 52]
k      = 2
output = 0    # pick both 100s
```

## Constraints

- `1 <= k <= len(nums) <= 1000`
- `0 <= nums[i] <= 10^5`
