# Hints

## Hint 1
This is the classic two-sum problem, with the values stored in a tree instead
of a list. What would you do with a list?

## Hint 2
A hash set of the values seen so far works on any tree. A BST gives you
something more: an in-order traversal lists its values in sorted order.

## Hint 3
Collect the values in order, then use two pointers, `i` at the start and `j` at
the end. If the sum is too small, advance `i`. If it is too large, move `j`
back. Stop when `i` meets `j`, which also guarantees the two nodes are
different.
