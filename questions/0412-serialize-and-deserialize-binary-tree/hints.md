# Hints

## Hint 1
Preorder visits a node, then its whole left subtree, then its whole right
subtree. The `#` markers tell you exactly where each subtree ends, so the
string pins down a single tree.

## Hint 2
A recursive reader is natural: read a token; if it is `#` return nothing,
otherwise make a node, read its left subtree, then its right subtree. With
up to 10^4 nodes the recursion can get deep, so think about doing the same
thing with an explicit stack.

## Hint 3
Keep a stack of nodes whose children are not all assigned, plus a flag for
whether each one's left slot is already filled. Every token after the first
fills the next open slot of the node on top: the left slot first, then the
right slot (after which that node is finished and popped). A real node is
then pushed, since its own children come next.
