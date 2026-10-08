An `n × n` grid holds the heights of a square of terrain. Every value from `0`
to `n² - 1` appears exactly once. It starts raining, and at time `t` the water
everywhere stands at level `t`.

You start in the top-left cell `(0, 0)`. At time `t` you may swim between two
cells that share a side, any distance and in no time at all, as long as both
cells have height at most `t`.

Return the earliest time at which you can reach the bottom-right cell
`(n - 1, n - 1)`.

## Example 1

```
grid = [
  [0, 2],
  [1, 3]
]
output = 3      # the target itself has height 3, so you must wait until t = 3
```

## Example 2

```
grid = [
  [ 0,  1,  2,  3,  4],
  [24, 23, 22, 21,  5],
  [12, 13, 14, 15, 16],
  [11, 17, 18, 19, 20],
  [10,  9,  8,  7,  6]
]
output = 16
# 0 1 2 3 4 5 16 15 14 13 12 11 10 9 8 7 6: the highest cell on the way is 16.
# Going straight down the right edge would need the 20.
```

## Constraints

- `1 <= n <= 50`
- The values of `grid` are a permutation of `0 .. n² - 1`.
