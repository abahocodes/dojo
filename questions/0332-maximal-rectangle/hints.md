# Hints

## Hint 1
Fix the bottom row of the rectangle. For every column, how many consecutive
`"1"` cells end at that row? Those counts form a bar chart.

## Hint 2
The best rectangle whose bottom edge lies on a given row is the largest
rectangle inside that bar chart. The bar heights can be updated from the
previous row in O(cols): add one on `"1"`, reset to zero on `"0"`.

## Hint 3
Largest rectangle in a bar chart: keep a stack of column indices with
increasing heights. When a shorter bar arrives, pop; the popped bar's
rectangle spans from just after the new stack top to just before the current
column. A sentinel bar of height 0 at the end flushes the stack.
