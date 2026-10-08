You are given an integer array `nums` and a budget `k`. One operation picks
any element and increases it by `1`. You may perform **at most** `k`
operations in total (spread over any elements you like, or none at all).

The frequency of a value is how many elements equal it. Return the largest
frequency any value can reach after your operations.

## Example 1

```
nums   = [3, 1, 6, 4]
k      = 5
output = 3    # raise 3 -> 4 (1 op) and 1 -> 4 (3 ops): [4, 4, 6, 4]
```

## Example 2

```
nums   = [10, 20, 30]
k      = 1
output = 1    # one increment cannot make any two values equal
```

## Constraints

- `1 <= len(nums) <= 10^5`
- `1 <= nums[i] <= 10^5`
- `1 <= k <= 10^5`
