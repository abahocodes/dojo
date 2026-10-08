A row of vertical walls stands at positions `0, 1, ..., n - 1`; the wall at
position `i` has height `heights[i]`. Choose **two different walls**. Together
with the ground they form a container, and the water it can hold is

```
(distance between the walls) * (height of the shorter wall)
```

The walls in between do not get in the way. Return the largest amount of water
any pair of walls can hold.

## Example 1

```
heights = [2, 7, 3, 6, 1, 5]
output  = 20         # walls 1 and 5: distance 4, shorter wall 5
```

## Example 2

```
heights = [4, 4]
output  = 4          # distance 1, shorter wall 4
```

## Constraints

- `2 <= len(heights) <= 10^5`
- `0 <= heights[i] <= 10^4`

**Follow-up:** checking all pairs is O(n²). Can you do it in one pass?
