# Hints

## Hint 1
The smallest value overall must be the head of one of the two lists. Which one?

## Hint 2
Repeatedly compare the two current heads, take the smaller node, and advance
in that list. A placeholder "dummy" node in front of the result saves you from
special-casing the very first node.

## Hint 3
Keep `tail = dummy`. While both lists are non-empty, attach the smaller head to
`tail.next` and move forward. When one list runs out, attach the rest of the
other in one step: it is already sorted. Return `dummy.next`.
