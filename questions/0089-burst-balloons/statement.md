A row of balloons is painted with the numbers in `nums`. You pop them one at a
time, in any order you like, until none are left. Popping a balloon earns
`left × value × right` points, where `value` is that balloon's number and
`left` / `right` are the numbers on its **current** neighbours, the closest
balloons still unpopped on each side. If there is no balloon on a side, that
side counts as `1`.

After a pop, the balloons on either side of the gap become neighbours.
Return the **maximum** total number of points you can earn.

## Example 1

```
nums   = [2, 4, 3, 5]
output = 115
# pop 3: 4*3*5 = 60   -> [2, 4, 5]
# pop 4: 2*4*5 = 40   -> [2, 5]
# pop 2: 1*2*5 = 10   -> [5]
# pop 5: 1*5*1 = 5    -> total 115
```

## Example 2

```
nums   = [6, 1]
output = 12     # pop 1 (6*1*1 = 6), then 6 (1*6*1 = 6)
```

## Constraints

- `1 <= len(nums) <= 100`
- `0 <= nums[i] <= 100`
