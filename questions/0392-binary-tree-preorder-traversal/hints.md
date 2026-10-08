# Hints

## Hint 1
The recursive version is three lines: record the node, recurse left, recurse
right. A deep, chain-like tree can make that recursion very deep, though.

## Hint 2
Simulate the recursion with your own stack. Popping a node means "visit it
now"; its children are the work still to be done.

## Hint 3
Pop a node, record its value, then push its **right** child before its
**left** child. The stack is last-in-first-out, so the left subtree is handled
completely before the right one.
