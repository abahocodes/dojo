# Hints

## Hint 1
Each interval can be handled on its own; the removal never joins two
intervals together.

## Hint 2
An interval that ends at or before `lo`, or starts at or after `hi`, is not
affected at all. What can be left of an interval that is affected?

## Hint 3
An affected interval `[a, b)` keeps `[a, lo)` if `a < lo` and `[hi, b)` if
`b > hi`. Both, one or neither may survive.
