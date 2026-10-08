A burglar is working down a straight street of houses. `nums[i]` is the amount
of cash hidden in house `i`. Neighbouring houses share an alarm system: breaking
into two houses that sit next to each other on the same night sets it off.

Return the **largest total** the burglar can collect in one night without
breaking into any two adjacent houses.

## Example 1

```
nums   = [6, 1, 2, 9, 4]
output = 15     # houses 0 and 3: 6 + 9
```

## Example 2

```
nums   = [5, 10, 4]
output = 10     # house 1 alone beats 5 + 4
```

## Constraints

- `1 <= len(nums) <= 1000`
- `0 <= nums[i] <= 1000`
