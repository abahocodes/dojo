# Hints

## Hint 1
Copying the values into an array and comparing it with its reverse is easy,
but uses O(n) extra memory. What could you change in the list itself instead?

## Hint 2
A palindrome's first half mirrors its second half. If the second half were
reversed, you could walk both halves forward in lockstep.

## Hint 3
Find the middle with slow/fast pointers, reverse the list from the middle
onward by re-linking `next` pointers, then compare the first half with the
reversed half node by node. (Reversing it back afterwards restores the input.)
