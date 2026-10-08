# Hints

## Hint 1
Comparing just the lists of values is not enough: the same values can be
arranged in different shapes. Compare node by node, position by position.

## Hint 2
Two trees are identical when their roots match and their left subtrees are
identical and their right subtrees are identical. What are the cases at a
single pair of positions (both missing, one missing, both present)?

## Hint 3
Push the pair `(p, q)` on a stack. Pop a pair: if both are `None`, continue; if
exactly one is `None` or the values differ, return `False`; otherwise push
`(p.left, q.left)` and `(p.right, q.right)`. If the stack empties, return `True`.
