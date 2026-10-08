Given a list of non-negative integers `nums`, return the largest value of
`nums[i] XOR nums[j]` over all index pairs with `0 <= i <= j < len(nums)`.
Since `i` may equal `j`, the answer is at least `0` (a number XOR itself).

## Example 1

```
nums   = [5, 9, 12, 3, 25]
output = 28
```

`5 XOR 25 = 00101 XOR 11001 = 11100 = 28`.

## Example 2

```
nums   = [14, 70, 53, 83, 49, 91, 36, 80, 92, 51, 66, 70]
output = 127
```

`36 XOR 91 = 0100100 XOR 1011011 = 1111111 = 127`.

## Constraints

- `1 <= len(nums) <= 2 * 10^4`
- `0 <= nums[i] <= 2^31 - 1`
