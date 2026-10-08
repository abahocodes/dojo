# Hints

## Hint 1
Looking at a node alone, you cannot tell whether it is a left child. Which node
does know that?

## Hint 2
Decide at the parent: a node's left child is a left leaf exactly when that
child exists and has no children of its own.

## Hint 3
Traverse with a stack of `(node, is_left)` pairs, starting with
`(root, False)`. When you pop a leaf with `is_left` set, add its value. Push the
left child with `True` and the right child with `False`.
