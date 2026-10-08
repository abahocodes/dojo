# Hints

## Hint 1
Treat each word as a bag of letters. How many copies of `'a'` can the
answer hold, given every word's count of `'a'`?

## Hint 2
It is the minimum of `'a'`'s count over all words. The same holds for each
of the 26 letters independently.

## Hint 3
Keep a 26-entry array of running minimums. For each word, count its letters
and lower each minimum. Finally emit each letter `i` exactly `min[i]` times,
from `'a'` to `'z'`.
