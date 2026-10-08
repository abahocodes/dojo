# Hints

## Hint 1
Thinking about which balloon to pop **first** is a dead end: after the pop the
two sides are still neighbours, so the subproblems aren't independent. Try
thinking about which balloon is popped **last** instead.

## Hint 2
Pad the row with a `1` on each end. For an open interval `(i, j)` of balloons
strictly between `i` and `j`, suppose `k` is the last one popped in it. When
it pops, its neighbours are exactly `i` and `j`, worth `v[i] * v[k] * v[j]`.

## Hint 3
Balloons left of `k` and right of `k` never interact (`k` separates them until
the end), so `best(i, j) = max over k of best(i, k) + v[i]*v[k]*v[j] + best(k, j)`.
Fill the table by increasing interval length; the answer is `best(0, n + 1)`.
