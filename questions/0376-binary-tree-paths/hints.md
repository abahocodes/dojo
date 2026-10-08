# Hints

## Hint 1
Each answer string is fixed once you reach a leaf. What do you need to carry
down the tree so that the leaf can produce its string immediately?

## Hint 2
Carry the path text built so far. A child's text is its parent's text followed
by `"->"` and the child's value.

## Hint 3
Use a stack of `(node, path)` pairs, starting with `(root, str(root.val))`.
When you pop a leaf, append `path` to the result; otherwise push each existing
child with `path + "->" + str(child.val)`.
