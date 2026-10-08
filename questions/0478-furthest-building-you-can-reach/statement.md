You stand on building `0` of a row of buildings with heights `heights`, and
you walk to the right one building at a time. You carry `bricks` bricks and
`ladders` ladders.

Stepping from building `i` to building `i + 1`:

- costs nothing if `heights[i + 1] <= heights[i]`;
- otherwise needs **either** one ladder **or** exactly
  `heights[i + 1] - heights[i]` bricks. Ladders and bricks are used up.

Choosing when to spend ladders and when to spend bricks as well as possible,
return the largest index (0-based) of a building you can reach.

## Example 1

```
heights = [4, 2, 7, 6, 9, 14, 12]
bricks  = 5
ladders = 1
output  = 4
```

Go 0 → 1 for free, 1 → 2 with the ladder, 2 → 3 for free, 3 → 4 with 3
bricks. Climbing to building 5 needs 5 more bricks (only 2 left) or a ladder
(none left).

## Example 2

```
heights = [4, 12, 2, 7, 3, 18, 20, 3, 19]
bricks  = 10
ladders = 2
output  = 7
```

## Constraints

- `1 <= len(heights) <= 10^5`
- `1 <= heights[i] <= 10^6`
- `0 <= bricks <= 10^9`
- `0 <= ladders <= len(heights)`
