# Hints

## Hint 1
In what order does the final chain list the nodes? Which traversal of a BST
visits nodes in that order?

## Hint 2
Walk the tree in order and append each node to the end of a growing chain. A
dummy head node makes the first append no different from the rest.

## Hint 3
Keep `tail`, the last node of the chain. When you visit a node, set
`node.left = None`, `tail.right = node`, `tail = node`. Its left subtree has
already been visited, so clearing the left pointer loses nothing. Return
`dummy.right`.
