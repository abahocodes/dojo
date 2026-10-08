# Hints

## Hint 1
Two closed intervals `[a, b]` and `[c, d]` intersect in
`[max(a, c), min(b, d)]` whenever that is a valid interval.

## Hint 2
Comparing every pair is quadratic. Since both lists are sorted, use one
pointer into each and move them forward together, like merging two sorted
lists.

## Hint 3
After comparing `first[i]` and `second[j]`, drop the one with the smaller end:
it cannot reach any later interval of the other list.
