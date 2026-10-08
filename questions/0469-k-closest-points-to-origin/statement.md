You are given a list of points on a 2D plane, each written as `[x, y]`, and an
integer `k`. Return the `k` points that lie closest to the origin `(0, 0)`,
measured by ordinary straight-line (Euclidean) distance.

The points may be returned in any order. Each point must be returned exactly
as it appears in the input. The inputs are chosen so that the answer is
unique: the `k`-th smallest distance is strictly smaller than the
`(k + 1)`-th smallest distance.

## Example 1

```
points = [[3, 4], [1, -1], [-2, 2], [5, 0]]
k      = 2
output = [[1, -1], [-2, 2]]
```

The squared distances are 25, 2, 8 and 25, so the two closest points are
`[1, -1]` and `[-2, 2]`.

## Example 2

```
points = [[0, 7], [-6, -1], [2, 2]]
k      = 1
output = [[2, 2]]
```

## Constraints

- `1 <= k <= len(points) <= 10^4`
- `-10^4 <= x, y <= 10^4`
- If `k < len(points)`, the `k`-th and `(k + 1)`-th smallest distances differ.
