# Hints

## Hint 1
Which surviving nodes become roots of the forest? Think about what must be
true of a node's parent for the node to start a new tree.

## Hint 2
A surviving node is a root exactly when it is the original root or its parent
was deleted. Put `to_delete` into a set so each check is O(1).

## Hint 3
Traverse the tree carrying a flag "my parent is gone" (true for the original
root). At each node: if it survives and the flag is set, add it to the answer.
Pass its own "deleted" status down as the children's flag, and set a child
pointer to `null` whenever that child is going to be deleted, so each surviving
tree is cut cleanly.
