# Hints

## Hint 1
Every match has length `len(p)`, so you only need to examine windows of `s` of
that fixed length. Two strings are rearrangements exactly when their letter
counts agree.

## Hint 2
Comparing a fresh 26-letter count for every window is O(26 * n), which is
already fine. Building each window's counts from scratch, however, is
O(len(p)) per window. Slide the counts instead.

## Hint 3
Maintain counts for the current window and a counter `matches` of letters whose
window count equals their count in `p`. Each slide changes two letters; update
`matches` for just those. Record `i` whenever `matches == 26`.
