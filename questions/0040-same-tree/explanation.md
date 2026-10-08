# Approach: walk both trees in lockstep

Visit the two trees simultaneously, always looking at a pair of nodes that sit
in the same position. A pair is fine if both nodes are missing. It proves the
trees differ if exactly one is missing or the values disagree. Otherwise the
pair's children must be compared too. An explicit stack keeps this safe on very
deep trees.

```python
def is_same_tree(p, q):
    stack = [(p, q)]
    while stack:
        a, b = stack.pop()
        if a is None and b is None:
            continue
        if a is None or b is None or a.val != b.val:
            return False
        stack.append((a.left, b.left))
        stack.append((a.right, b.right))
    return True
```

The recursive form reads almost like the definition:
`return p.val == q.val and same(p.left, q.left) and same(p.right, q.right)`,
after handling the `None` cases first.

## Complexity

- Time: O(min(n, m)): we stop at the first difference, and never visit more
  nodes than the smaller tree has (plus its missing children).
- Space: O(h) for the stack, O(n) in the worst case.

## Pitfalls

- Comparing traversal value lists without the `null` markers: `[3, 7]` and
  `[3, null, 7]` have the same in-order values but different shapes.
- Checking `a.val != b.val` before making sure neither node is `None`.
- Deep recursion on skewed trees.
