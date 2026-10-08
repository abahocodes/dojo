# Hints

## Hint 1
Walk through `word` and `abbr` at the same time with one index into each.

## Hint 2
A letter in `abbr` must equal the letter at the current position of `word`.
A run of digits tells you how many letters of `word` to skip.

## Hint 3
When you meet a digit, reject it immediately if it is `0` (that is a leading
zero). Otherwise read the whole number and advance the `word` index by it. At
the end, both indices must sit exactly at the ends of their strings.
