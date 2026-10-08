# Hints

## Hint 1
Checking only that each node's direct children are on the correct side is not
enough. Look at Example 2: every parent/child pair looks fine on its own.

## Hint 2
Each node must lie inside an allowed range inherited from all of its
ancestors: going left tightens the upper bound, going right tightens the lower
bound. Alternatively: what does an in-order traversal of a valid BST produce?

## Hint 3
Do an iterative in-order traversal (push left children on a stack, pop, then go
right) and remember the previously visited value. The tree is valid exactly
when every visited value is strictly greater than the previous one.
