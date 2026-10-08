You are given a list of integers `nums` of length `n`. One value is
guaranteed to occur **more than `n / 2` times**, more often than all other
values put together.

Return that value.

## Example 1

```
nums   = [6, 1, 6, 6, 2]
output = 6           # 6 occurs 3 times out of 5
```

## Example 2

```
nums   = [-4, -4, 9, -4]
output = -4
```

## Constraints

- `1 <= len(nums) <= 5 * 10^4`
- `-10^9 <= nums[i] <= 10^9`
- A value occurring more than `n / 2` times always exists.

**Follow-up:** counting with a hash map uses O(n) extra memory. Can you solve
it in O(n) time and O(1) memory?
