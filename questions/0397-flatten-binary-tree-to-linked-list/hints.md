# Hints

## Hint 1
The easy version: record the nodes in preorder in a list, then relink them one
after another. That works. Can you avoid the extra list?

## Hint 2
Look at a node that has a left subtree. In preorder, the whole left subtree
comes right after the node, and the right subtree comes right after the last
node of the left subtree. Which node of the left subtree is visited last in
preorder once everything is a right chain?

## Hint 3
Walk down the right pointers with a cursor. Whenever the cursor has a left
child, find the rightmost node of that left subtree, attach the cursor's right
subtree to it, move the left subtree into the cursor's right slot, and clear
the left pointer. Then step right and repeat.
