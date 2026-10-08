# Hints

## Hint 1
Because every step goes strictly uphill, a route can never revisit a cell, so
you don't need a visited set. The longest route starting at a cell depends only
on that cell.

## Hint 2
Let `best(cell) = 1 + max(best(n))` over the strictly higher neighbours `n`
(or `1` if there are none). Memoising this gives each cell's answer once.

## Hint 3
A route can be 40,000 cells long, too deep for naive recursion. Instead,
process cells from highest to lowest elevation: when you reach a cell, all of
its higher neighbours already have their final `best`.
