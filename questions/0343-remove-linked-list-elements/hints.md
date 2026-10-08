# Hints

## Hint 1
Removing a node from the middle means making its predecessor skip over it. What
is awkward about removing the *first* node?

## Hint 2
Several nodes in a row (including at the very start) may need to go. Make sure
you do not advance past a node before checking its new successor.

## Hint 3
Put a dummy node in front of `head`. Keep a pointer `prev` starting at the
dummy: if `prev.next.val == val`, set `prev.next = prev.next.next`; otherwise
move `prev` forward. Return `dummy.next`.
