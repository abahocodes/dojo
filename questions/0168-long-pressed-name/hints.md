# Hints

## Hint 1
Split both strings into runs of equal characters. What has to be true about
corresponding runs?

## Hint 2
The runs must have the same characters in the same order, and every run in
`typed` must be at least as long as the matching run in `name`. You can check
this without building the runs.

## Hint 3
Walk `typed` with `j` and `name` with `i`. If `typed[j]` matches `name[i]`,
advance `i`. Otherwise `typed[j]` must repeat `typed[j - 1]` (a stuck key);
if not, return `false`. At the end, `i` must have reached the end of `name`.
