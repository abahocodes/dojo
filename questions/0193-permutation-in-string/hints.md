# Hints

## Hint 1
A rearrangement of `s1` has the same length as `s1`. Only substrings of `s2` of
length `len(s1)` are candidates.

## Hint 2
Two strings are rearrangements of each other exactly when their letter counts
match. Compare a 26-entry count array of `s1` with the counts of each window.

## Hint 3
Slide the window across `s2`, adding the entering letter and removing the
leaving one. Instead of comparing all 26 counts each step, keep a number
`matches` of letters whose counts currently agree, and update it only for the
two letters that changed. The answer is `true` as soon as `matches == 26`.
