# Hints

## Hint 1
An inorder traversal of a valid BST produces a strictly increasing sequence.
What does that sequence look like after two of its values are exchanged?

## Hint 2
Look for places where a value is larger than the one after it. If the swapped
values were neighbors in inorder there is one such drop; otherwise there are
two.

## Hint 3
During an inorder traversal keep the previous node. At the first drop, the
first misplaced node is `prev`; at every drop, record the current node as the
second one. After the traversal, swap the two values.
