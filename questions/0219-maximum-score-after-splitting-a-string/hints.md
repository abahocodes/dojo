# Hints

## Hint 1
There are only `len(s) - 1` possible cuts. Can you get the score of each cut
from the previous one in O(1)?

## Hint 2
Moving the cut one character to the right moves that character from the
right part to the left part. A `'0'` adds one point; a `'1'` loses one.

## Hint 3
Start with the score of an empty left part: the total number of `'1'`s. Walk
the cut positions `1 .. len(s) - 1`, updating the score as in Hint 2, and keep
the maximum. Do not consider the cut after the last character.
