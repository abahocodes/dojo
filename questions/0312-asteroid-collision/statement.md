A row of asteroids is drifting along a line. Each one is given by a nonzero
integer: its absolute value is its size, and its sign is its direction
(positive moves right, negative moves left). All asteroids move at the same
speed, so two asteroids moving in the same direction never meet.

When an asteroid moving right runs into one moving left, the smaller of the
two is destroyed. If they have the same size, both are destroyed. The
survivor keeps moving and may collide again.

Given the asteroids in their order along the line, return the ones that are
left once no more collisions can happen, in their original left-to-right
order. If none survive, return an empty array.

## Example 1

```
asteroids = [4, 9, -6, 2]
output    = [4, 9, 2]    # -6 hits 9 and is destroyed
```

## Example 2

```
asteroids = [7, -7, -3, 5]
output    = [-3, 5]      # 7 and -7 destroy each other; -3 and 5 move apart
```

## Constraints

- `2 <= len(asteroids) <= 10^4`
- `1 <= |asteroids[i]| <= 1000` (no value is `0`)
