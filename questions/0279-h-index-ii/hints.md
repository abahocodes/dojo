# Hints

## Hint 1
Since the array is sorted, the papers with the most citations are at the end.
If you count the last `n - i` papers, which of them has the fewest citations?

## Hint 2
The last `n - i` papers all have at least `n - i` citations exactly when
`citations[i] >= n - i`. As `i` moves right, `citations[i]` grows while
`n - i` shrinks.

## Hint 3
So the condition `citations[i] >= n - i` is false, then true. Binary search
for the first index `i` where it holds (or `n` if it never does) and return
`n - i`.
