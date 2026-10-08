# Hints

## Hint 1
The outline can only change height at some building's `left` or `right`.
Sweep a vertical line across those x-coordinates in increasing order and ask,
at each one, "what is the tallest building covering this x?"

## Hint 2
Keep the buildings currently under the sweep line in a max-heap keyed by
height. A building is added at its `left`. Instead of deleting it at its
`right`, store its `right` alongside the height and discard the top whenever
the top has already ended (`right <= x`).

## Hint 3
Order events carefully at equal `x`: process starts before ends, and taller
starts first. Then, after the first event at a given `x`, the heap top is
already the true height there, and you emit `[x, top height]` only when it
differs from the last emitted height (an empty heap means height `0`).
