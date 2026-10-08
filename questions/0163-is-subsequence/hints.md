# Hints

## Hint 1
Characters of `s` must be found in `t` in the same order. Which occurrence
in `t` should the first character of `s` be matched to?

## Hint 2
Matching each character of `s` to the earliest possible position in `t`
never hurts: it leaves the most room for the characters that follow.

## Hint 3
Use two pointers `i` (in `s`) and `j` (in `t`). Advance `j` every step and
advance `i` only when `s[i] == t[j]`. The answer is whether `i` reached
`len(s)`.
