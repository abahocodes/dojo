# Hints

## Hint 1
When you move from a node to its child, how does the number spelled so far
change? Think about appending a digit to the right of an integer.

## Hint 2
Carry the number for the path so far down the tree. At a child,
`value = parent_value * 10 + child.val`.

## Hint 3
Do a DFS (recursive or with a stack of `(node, prefix)` pairs). When you reach
a node with no children, add its `value` to the total; otherwise push its
children with the new prefix.
