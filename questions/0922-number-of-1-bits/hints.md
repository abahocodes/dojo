# Hints

## Hint 1
`n & 1` tells you whether the lowest bit is set, and `n >> 1` drops that bit.
Repeat until `n` is `0`.

## Hint 2
To skip the zero bits entirely, look at what `n - 1` does to the binary form
of `n`: it flips the lowest `1` bit and every `0` below it.

## Hint 3
`n & (n - 1)` is `n` with its lowest `1` bit cleared. Count how many times you
can apply it before `n` becomes `0`.
