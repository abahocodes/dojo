# Hints

## Hint 1
Keep a sorted list that starts empty. A dummy head node in front of it means
"insert at the very front" is no different from inserting anywhere else.

## Hint 2
For each input node, save its `next` first, then walk the sorted list from the
dummy until the following node's value is greater than the one you are
inserting, and splice it in there.

## Hint 3
Inputs that are already (mostly) sorted make every walk go to the end. Keep a
pointer to the sorted list's tail: if the new value is at least the tail's
value, append it directly in O(1).
