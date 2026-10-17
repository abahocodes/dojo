# Hints

## Hint 1
Do what you would on paper: add one to the last digit. When does that last
digit affect anything to its left?

## Hint 2
Only a `9` produces a carry: it turns into `0` and the `1` moves one place
left. Any other digit just grows by one, and then nothing further changes.

## Hint 3
Scan from the right. Replace each `9` by `0` and keep going; at the first digit
below `9`, add one and return. If you run off the front, every digit was `9`:
the answer is a `1` followed by `len(digits)` zeros.
