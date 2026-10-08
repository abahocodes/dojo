# Hints

## Hint 1
If `len(s1) + len(s2) != len(s3)` the answer is `false`. Otherwise, after
taking `i` letters from `s1` and `j` from `s2`, the next letter of `s3` is
always `s3[i + j]`, so the state is just the pair `(i, j)`.

## Hint 2
Greedily preferring `s1` whenever its letter matches fails: with
`s1 = "ab"`, `s2 = "ac"`, `s3 = "acab"` the first `a` must come from `s2`. You
need to explore both choices, but there are only
`(len(s1) + 1) × (len(s2) + 1)` states.

## Hint 3
`ok[i][j]` is true when the first `i` letters of `s1` and first `j` of `s2` can
form `s3[:i + j]`: `ok[i][j] = (ok[i-1][j] and s1[i-1] == s3[i+j-1]) or
(ok[i][j-1] and s2[j-1] == s3[i+j-1])`, with `ok[0][0] = true`. One row of the
table is enough.
