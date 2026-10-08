# Hints

## Hint 1
Instead of counting what to change, count what to **keep**. The kept elements
must be distinct and must all fit in some range `[x, x + n - 1]`.

## Hint 2
Duplicates can never both be kept, so work with the sorted distinct values.
Is it enough to try ranges whose left end `x` is one of those values?

## Hint 3
Yes: shifting a range right until its left end hits a kept value never loses
anything. For each distinct value `v` as the left end, count the distinct
values in `[v, v + n - 1]` with two pointers (or binary search). The answer is
`n` minus the largest count.
