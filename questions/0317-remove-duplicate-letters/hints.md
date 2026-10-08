# Hints

## Hint 1
Build the answer left to right. You would like small letters as early as
possible, but you may only drop a big letter you already placed if you can
still pick it up later.

## Hint 2
Precompute the last index at which each letter occurs. While scanning, a
letter placed earlier can be "taken back" only if it occurs again further to
the right.

## Hint 3
Keep the answer as a stack plus a "used" flag per letter. For each letter not
already used, pop the stack while the top is larger than the current letter
and the top's last occurrence is after the current index (unmark what you
pop). Then push the current letter. Letters already in the stack are skipped.
