# Hints

## Hint 1
There are 2^n subsequences of `a`, far too many to check. Instead, compare
the strings from one end and ask what happens to the first (or last)
characters.

## Hint 2
Let `L(i, j)` be the answer for the prefixes `a[:i]` and `b[:j]`. If
`a[i-1] == b[j-1]`, that character can end the common subsequence. If not, one
of the two last characters is unused.

## Hint 3
`L(i, j) = L(i-1, j-1) + 1` when the last characters match, otherwise
`max(L(i-1, j), L(i, j-1))`. Fill the table row by row; each row only needs
the previous one.
