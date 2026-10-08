# Hints

## Hint 1
A node survives exactly when its value is at least the maximum of everything
after it. Thinking from the right end makes that easy to check.

## Hint 2
Scanning left to right instead, keep the survivors so far in a stack. A new
value removes every earlier survivor that is smaller than it.

## Hint 3
For each node: while the stack is non-empty and its top value is `< node.val`,
pop. Then push the node. At the end, link the stack from bottom to top (and end
the last node with `None`). That is a monotonic (non-increasing) stack.
