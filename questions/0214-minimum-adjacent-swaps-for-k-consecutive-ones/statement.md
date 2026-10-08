You are given an array `nums` made of `0`s and `1`s, and an integer `k`. The
array contains at least `k` ones.

In one move you may swap two **adjacent** elements. Return the minimum number
of moves needed so that `nums` contains `k` ones in a row (`k` consecutive
positions that all hold `1`).

## Example 1

```
nums   = [1, 0, 0, 1, 0, 1]
k      = 2
output = 1    # swap positions 3 and 4: [1, 0, 0, 0, 1, 1]
```

## Example 2

```
nums   = [1, 0, 0, 0, 0, 0, 1, 1]
k      = 3
output = 5    # walk the first 1 five steps to the right
```

## Constraints

- `1 <= len(nums) <= 10^5`
- `nums[i]` is `0` or `1`
- `1 <= k <=` the number of ones in `nums`
