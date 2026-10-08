# Hints

## Hint 1
The game is deterministic: from index `i`, the odd jump always lands on the
same `oddNext[i]` and the even jump on the same `evenNext[i]`. Precompute both,
then work backwards from the end: `goodOdd[i] = goodEven[oddNext[i]]` and
`goodEven[i] = goodOdd[evenNext[i]]`.

## Hint 2
To find `oddNext` for every index at once, sort the indices by
`(value, index)`. In that order, `oddNext[i]` is the first index after `i` in
the list that is also to the right of `i` in the array.

## Hint 3
"The next larger index in a list" is a monotonic-stack job: walk the sorted
order, and while the stack's top index is smaller than the current index, pop
it and set its target to the current index; then push. For `evenNext`, do the
same with indices sorted by `(-value, index)`.
