# Hints

## Hint 1
Picture the path from the root down to `p` and the path from the root down to
`q`. Both start at the root. Where do they stop agreeing?

## Hint 2
If every node knew its parent, you could climb from `p` to the root and
remember every node you pass. Then climb from `q` until you reach a node you
already remembered.

## Hint 3
Walk the tree with an explicit stack, recording `parent[child.val] = node.val`
(stop early once both `p` and `q` are recorded). Put `p` and all of its
ancestors in a set, then move `q` upward until it lands in that set; that value
is the answer. A recursive version ("return the node if it is `p` or `q`,
otherwise whichever side found something, or yourself if both sides did") is
shorter but can overflow the stack on a deep tree.
