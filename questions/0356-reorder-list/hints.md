# Hints

## Hint 1
Copying the nodes into an array and weaving from both ends works, but uses O(n)
extra space. Try to do it in O(1) extra space.

## Hint 2
The result interleaves the first half of the list with the second half read
backwards. Can you get at "the second half, reversed" directly?

## Hint 3
Find the middle with slow/fast pointers, cut the list there, reverse the second
half in place, then merge the two halves by alternately taking one node from
each.
