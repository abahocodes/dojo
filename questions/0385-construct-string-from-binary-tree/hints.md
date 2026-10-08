# Hints

## Hint 1
Write the rule as a recursive definition: the string of a node is its value,
then something for the left child, then something for the right child.

## Hint 2
There are three cases for the parentheses. A leaf gets nothing. A node with a
right child always writes both groups, `(left)(right)`, even if `left` is
empty. A node with only a left child writes just `(left)`.

## Hint 3
`s = str(val)`; if the node has any child, append `"(" + f(left) + ")"`
(where `f(None)` is `""`); if it has a right child, also append
`"(" + f(right) + ")"`. Build the pieces in a list and join once at the end
to avoid repeated string copying; for very deep trees use an explicit stack
that holds both nodes and literal `"("`, `")"`, `"()"` tokens.
