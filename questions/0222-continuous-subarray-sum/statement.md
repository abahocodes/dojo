You are given an array of non-negative integers `nums` and a positive
integer `k`. Decide whether `nums` contains a **contiguous** subarray that

- has **at least two** elements, and
- has a sum that is a multiple of `k`. A sum of `0` counts, since `0 = 0 * k`.

Return `true` if such a subarray exists, otherwise `false`.

## Example 1

```
nums   = [5, 3, 8, 2]
k      = 6
output = true    # the whole array sums to 18 = 3 * 6
```

## Example 2

```
nums   = [7]
k      = 7
output = false   # 7 is a multiple of 7, but a single element is too short
```

## Constraints

- `1 <= len(nums) <= 10^5`
- `0 <= nums[i] <= 10^9`
- `1 <= k <= 2^31 - 1`
