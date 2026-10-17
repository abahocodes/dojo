# Hints

## Hint 1
Store a count per distinct point instead of a list of points: copies matter
only through how many there are, and the number of squares using specific
corners is the product of their counts.

## Hint 2
An axis-aligned square through the query `(qx, qy)` is fixed once you know
one more corner. Pick the corner diagonally opposite, `(px, py)`: it needs
`|px - qx| == |py - qy| != 0`, and then the other two corners must be
`(qx, py)` and `(px, qy)`.

## Hint 3
Loop over the distinct stored points as diagonal candidates. For each valid
one, add `cnt[(px, py)] * cnt[(qx, py)] * cnt[(px, qy)]`. Missing points have
count `0`, so they contribute nothing automatically.
