You are given an array of positive integers `nums` and an integer
`threshold`. Pick a positive integer divisor `d`, divide every element of
`nums` by `d`, round each quotient **up** to the nearest integer, and add the
rounded quotients together.

Return the smallest divisor `d` for which this sum is at most `threshold`.
Since `threshold` is at least the length of `nums`, such a divisor always
exists (a large enough `d` makes every rounded quotient equal to `1`).

## Example 1

```
nums      = [2, 6, 9, 13]
threshold = 8
output    = 5   # d = 5: 1 + 2 + 2 + 3 = 8, but d = 4: 1 + 2 + 3 + 4 = 10
```

## Example 2

```
nums      = [7, 7, 7]
threshold = 3
output    = 7   # any smaller d rounds at least one 7 up to 2 or more
```

## Constraints

- `1 <= len(nums) <= 5 * 10^4`
- `1 <= nums[i] <= 10^6`
- `len(nums) <= threshold <= 10^6`
