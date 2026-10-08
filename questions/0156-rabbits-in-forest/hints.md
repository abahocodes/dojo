# Hints

## Hint 1
A rabbit that answers `x` belongs to a color group of exactly `x + 1`
rabbits. Rabbits giving different answers can never share a color.

## Hint 2
Handle each distinct answer separately. Up to `x + 1` rabbits that all say
`x` can be packed into one group; how many groups do `c` of them need?

## Hint 3
With `c` rabbits answering `x`, you need `ceil(c / (x + 1))` groups, each
contributing `x + 1` rabbits. Sum that over every distinct answer.
