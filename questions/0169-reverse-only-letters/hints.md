# Hints

## Hint 1
The non-letters act as fixed walls; only the letters move.

## Hint 2
The first letter must swap with the last letter, the second with the
second-to-last, and so on.

## Hint 3
Use two pointers at both ends of a mutable copy. Skip non-letters on either
side; when both point at letters, swap them and move both inward.
