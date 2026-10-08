There are `n` baskets along a line; basket `i` sits at integer coordinate
`position[i]`, and no two baskets share a coordinate. The array is **not**
necessarily sorted.

You must place `m` balls, each in a different basket. Two balls at
coordinates `a` and `b` repel each other with force `|a - b|`, and the
arrangement's weakest link is the smallest force between any two balls.

Return the largest possible value of that minimum distance over all ways to
place the `m` balls.

## Example 1

```
position = [1, 2, 3, 4, 7]
m        = 3
output   = 3   # balls at 1, 4 and 7
```

## Example 2

```
position = [10, 1, 30, 22]
m        = 2
output   = 29   # balls at 1 and 30
```

## Constraints

- `2 <= m <= n <= 10^5`
- `1 <= position[i] <= 10^9`
- all values in `position` are distinct
