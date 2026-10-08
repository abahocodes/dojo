# Hints

## Hint 1
Exploring every route one by one takes as long as the number of routes, which
can be in the quadrillions here. How can you reach a cell in the first place?

## Hint 2
The robot enters cell `(r, c)` either from the cell above or from the cell to
its left. So the routes to `(r, c)` are the routes to `(r-1, c)` plus the
routes to `(r, c-1)`.

## Hint 3
Every cell in the first row or first column has exactly one route. Fill the
grid row by row; a single array of length `n` is enough, updated with
`row[c] += row[c - 1]`.
