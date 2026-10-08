Wizards stand in a line, and `strength[i]` is the power of the wizard at
position `i`.

Any **group** of one or more wizards standing next to each other (a contiguous
piece of the line) has a *total strength* equal to

```
(weakest strength in the group) * (sum of all strengths in the group)
```

Add up the total strength of **every** contiguous group and return the result
modulo `10^9 + 7`.

## Example 1

```
strength = [2, 3, 1]
output   = 34
# [2]     -> 2 * 2 = 4      [3]     -> 3 * 3 = 9      [1] -> 1 * 1 = 1
# [2,3]   -> 2 * 5 = 10     [3,1]   -> 1 * 4 = 4
# [2,3,1] -> 1 * 6 = 6      total 4 + 9 + 1 + 10 + 4 + 6 = 34
```

## Example 2

```
strength = [4, 4]
output   = 64   # [4] -> 16, [4] -> 16, [4,4] -> 4 * 8 = 32
```

## Constraints

- `1 <= len(strength) <= 10^5`
- `1 <= strength[i] <= 10^9`
