You have `n` bags; bag `i` contains `nums[i]` balls. In one operation you may
take any bag and split it into **two** new bags, each holding a positive
number of balls (the two counts add up to the original count).

You may perform at most `max_operations` operations. Your penalty is the
number of balls in the **fullest** bag at the end. Return the smallest
penalty you can achieve.

## Example 1

```
nums           = [10]
max_operations = 1
output         = 5     # split 10 into 5 + 5
```

## Example 2

```
nums           = [7, 17]
max_operations = 3
output         = 6     # 7 -> 6 + 1, 17 -> 6 + 6 + 5 (two splits)
                       # a limit of 5 would need 1 + 3 = 4 splits
```

## Constraints

- `1 <= n <= 10^5`
- `1 <= nums[i] <= 10^9`
- `1 <= max_operations <= 10^9`
