Points on a grid arrive one at a time. At any moment you may be handed a
query point and asked: in how many ways can three of the stored points be
picked so that, together with the query point, they form the corners of a
square?

Only **axis-aligned** squares count (every side is horizontal or vertical),
and the square must have **positive area**. Points can be added more than
once; copies of the same point are distinct points, so each copy gives its
own choices.

Implement the class `DetectSquares`:

- `DetectSquares()` creates an empty store.
- `add(point)` stores `point = [x, y]`.
- `count(point)` returns the number of ways to choose three stored points
  that, with `point = [qx, qy]`, form an axis-aligned square of positive
  area. The query point itself is not stored by this call.

## How the tests call your class

Each test is a sequence of operations on one fresh `DetectSquares`. `ops[i]`
is `"add"` or `"count"` and `points[i]` is its `[x, y]` argument. The driver
at the bottom of the starter code runs them in order and returns the result
of every `count` call; `add` returns nothing and adds nothing to the output.
You only need to fill in the class.

## Example 1

```
ops    = ["add", "add", "add", "count", "count", "add", "count"]
points = [[2, 6], [5, 6], [5, 9], [2, 9], [3, 9], [5, 9], [2, 9]]
output = [1, 0, 2]
```

`[2, 9]` with `[2, 6]`, `[5, 6]` and `[5, 9]` is a 3-by-3 square. Nothing
forms a square with `[3, 9]`. After a second copy of `[5, 9]` arrives, either
copy can be the fourth corner, so there are `2` ways.

## Example 2

```
ops    = ["add", "add", "add", "add", "count"]
points = [[0, 0], [0, 2], [2, 0], [0, 4], [0, 0]]
output = [0]
```

Two points share the query's column and one shares its row, but the corner
`[2, 2]` is missing. A stored copy of the query point `[0, 0]` itself never
helps: that square would have zero area.

## Constraints

- `1 <= len(ops) == len(points) <= 3000`
- `0 <= x, y <= 1000`
- Every answer fits in a 32-bit signed integer.
