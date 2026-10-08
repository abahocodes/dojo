Given an integer array `nums` and a non-negative integer `k`, rotate the
array to the right by `k` positions and return the result.

One rotation to the right moves the last element to the front and shifts
every other element one place to the right. After rotating by `k`, the
element at index `i` ends up at index `(i + k) mod n`, where `n` is the
length of `nums`. `k` may be larger than `n`.

Try to do it in place with O(1) extra memory.

## Example 1

```
nums   = [10, 20, 30, 40, 50, 60]
k      = 2
output = [50, 60, 10, 20, 30, 40]
```

## Example 2

```
nums   = [7, -3, 4]
k      = 4
output = [4, 7, -3]    # rotating by 4 is the same as rotating by 1
```

## Constraints

- `1 <= len(nums) <= 10^5`
- `-2^31 <= nums[i] <= 2^31 - 1`
- `0 <= k <= 10^9`
