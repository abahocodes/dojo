# Hints

## Hint 1
Every valid substring straddles exactly one boundary where the character
changes, with `k` equal characters on each side of it.

## Hint 2
Compress the string into run lengths: `"0110011"` becomes `1, 2, 2, 2`. How
many valid substrings straddle the boundary between two adjacent runs?

## Hint 3
Between runs of lengths `a` and `b` you can take `k` characters from each side
for `k = 1 .. min(a, b)`, so that boundary contributes `min(a, b)`. Sum this
over all adjacent pairs, keeping only the previous and current run lengths.
