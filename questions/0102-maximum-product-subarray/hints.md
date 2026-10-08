# Hints

## Hint 1
Try adapting the "best stretch ending here" idea from the maximum *sum* problem.
What goes wrong when the next value is negative?

## Hint 2
Multiplying by a negative number turns the smallest product into the largest one.
So the most negative product ending at `i - 1` matters as much as the largest.

## Hint 3
Track both `hi` and `lo`, the largest and smallest products of stretches ending at
the current element. For a new value `x`, the candidates are `x`, `hi * x` and
`lo * x` — swap `hi` and `lo` first when `x` is negative.
