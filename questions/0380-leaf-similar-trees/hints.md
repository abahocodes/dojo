# Hints

## Hint 1
Collect each tree's leaves into a list, then compare the two lists. Which
traversal visits leaves from left to right?

## Hint 2
Any depth-first traversal that explores the left subtree before the right one
(pre-order, in-order or post-order) meets the leaves in left-to-right order.
A node is a leaf when both of its children are missing.

## Hint 3
With an explicit stack: pop a node; if it is a leaf, record its value;
otherwise push its right child and then its left child, so the left side is
popped first. Do this for both trees and compare the two value lists.
