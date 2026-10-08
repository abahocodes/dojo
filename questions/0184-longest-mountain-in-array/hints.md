# Hints

## Hint 1
Every mountain has exactly one peak. For a fixed peak `p`, how far can the
mountain extend to the left, and to the right?

## Hint 2
The left part is the strictly increasing run ending at `p`; the right part is
the strictly decreasing run starting at `p`. Both must have at least two
elements. Two arrays of run lengths give an O(n) answer.

## Hint 3
You can do it in one pass with O(1) memory: keep `up` and `down` step counts
for the mountain you are currently in. Reset both on an equal pair, or when
the values start rising again after a descent. Whenever both are positive,
`up + down + 1` is a candidate.
