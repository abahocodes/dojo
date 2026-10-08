Two movers want to share a load of boxes so that each of them carries exactly
the same total weight. The weight of every box is listed in `nums`, and a box
can't be split: each one goes to one mover or the other.

Return `true` if the boxes can be divided into two groups with **equal total
weight**, and `false` otherwise. Every box must be assigned to someone.

## Example 1

```
nums   = [2, 7, 4, 3, 6]
output = true     # 7 + 4 = 11 and 2 + 3 + 6 = 11
```

## Example 2

```
nums   = [2, 2, 10]
output = false    # the total is 14, but no group of boxes weighs 7
```

## Constraints

- `1 <= len(nums) <= 200`
- `1 <= nums[i] <= 100`
