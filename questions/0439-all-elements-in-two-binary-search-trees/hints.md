# Hints

## Hint 1
How can you read the values of a single BST in sorted order without sorting?

## Hint 2
An in-order traversal of each tree gives two sorted lists. You have seen a
classic linear-time way to combine two sorted lists.

## Hint 3
Run an iterative in-order traversal on each tree, then merge with two
pointers: repeatedly take the smaller of the two front values (ties can come
from either side), and append whatever remains of the longer list at the end.
