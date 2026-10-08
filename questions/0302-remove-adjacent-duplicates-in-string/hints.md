# Hints

## Hint 1
Repeatedly scanning the string and deleting pairs works, but each pass is
O(n) and you may need O(n) passes. Can one left-to-right pass do it?

## Hint 2
While scanning, keep the characters that have survived so far. A new
character can only cancel with the most recent survivor.

## Hint 3
Use a stack: if the top equals the current character, pop it; otherwise push
the character. The stack, read from bottom to top, is the answer.
