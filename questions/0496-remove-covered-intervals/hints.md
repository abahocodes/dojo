# Hints

## Hint 1
Comparing every pair works for small inputs but is quadratic. What order would
let you decide whether an interval is covered by looking only at intervals you
have already seen?

## Hint 2
Sort by start. Any interval that could cover the current one then comes before
it, as long as ties are broken well. Which interval should come first when two
share a start?

## Hint 3
Sort by start ascending and, for equal starts, by end descending. Scan while
tracking the largest end seen so far: the current interval is covered exactly
when its end is at most that maximum.
