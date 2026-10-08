# Hints

## Hint 1
Recursion is easy (left, right, then the node) but can overflow the call stack
on a deep, chain-shaped tree. Try to simulate it with a stack.

## Hint 2
Look at postorder backwards: it is node, right subtree, left subtree. That is
a preorder walk with the children visited in the opposite order, which is easy
to do with a stack.

## Hint 3
Push `root`. Repeatedly pop a node, append its value, and push its **left**
child and then its **right** child (so the right one is popped first). When the
stack is empty, reverse the collected values.
