This time the houses stand around a **circular** cul-de-sac. `nums[i]` is the
cash in house `i`, and house `0` sits right next to house `len(nums) - 1`.
As before, robbing two neighbouring houses on the same night trips the alarm,
and that includes the first and last house.

Return the **largest total** that can be collected without robbing any two
adjacent houses. A street of a single house has no neighbours, so that house
can always be robbed.

## Example 1

```
nums   = [3, 5, 3]
output = 5      # houses 0 and 2 touch around the circle
```

## Example 2

```
nums   = [2, 7, 3, 1, 8, 4]
output = 15     # houses 1 and 4: 7 + 8
```

## Constraints

- `1 <= len(nums) <= 1000`
- `0 <= nums[i] <= 1000`
