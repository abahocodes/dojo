# Hints

## Hint 1
A gap can only appear between two neighbouring present values, before the
first one, or after the last one.

## Hint 2
Pretend there is an extra present value at `lower - 1` and another at
`upper + 1`. Now every gap sits between two neighbours of one list.

## Hint 3
Walk the extended list keeping the previous value `prev`. If the next value
`x` satisfies `x - prev >= 2`, the block `[prev + 1, x - 1]` is missing.
