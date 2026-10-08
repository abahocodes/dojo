# Hints

## Hint 1
Only the minimum and maximum of a subsequence matter, never the order of its
elements. Does sorting `nums` change the answer?

## Hint 2
After sorting, pick the position `i` of the minimum. If `j >= i` is the last
position with `a[i] + a[j] <= target`, how many subsequences have `a[i]` as
their smallest chosen element and stay within `target`?

## Hint 3
Each of the `j - i` elements after `i` (up to `j`) is in or out freely:
`2^(j - i)` subsequences. As `i` grows, `j` only moves left, so walk two
pointers inward and add precomputed powers of two modulo `10^9 + 7`.
