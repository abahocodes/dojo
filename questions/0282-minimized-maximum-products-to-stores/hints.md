# Hints

## Hint 1
Suppose every store may hold at most `x` units. How many stores does product
type `i` need then?

## Hint 2
Type `i` needs `ceil(quantities[i] / x)` stores, and types cannot share a
store, so the plan works exactly when the sum of those counts is at most `n`.

## Hint 3
That sum only shrinks as `x` grows. Binary search the smallest `x` in
`[1, max(quantities)]` whose total is at most `n`.
