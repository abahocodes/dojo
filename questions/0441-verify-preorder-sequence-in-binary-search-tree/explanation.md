# Approach: monotonic stack with a growing lower bound

Simulate the preorder walk. The stack holds the current path of nodes whose
right subtree has not been entered yet; its values decrease from bottom to
top, because each one is in the left subtree of the one below it.

When the next value `x` is larger than the top of the stack, the walk has
finished that node's left side and moved into a right subtree. The ancestor we
move right from is the *last* stack value smaller than `x`, so we pop all
smaller values and remember the last one as `low`. From then on, every value
must exceed `low`: it lies in that ancestor's right subtree. A value below
`low` is impossible, so the sequence is not a preorder of any BST.

```python
def verify_preorder(preorder):
    low = float("-inf")
    stack = []
    for x in preorder:
        if x < low:
            return False
        while stack and stack[-1] < x:
            low = stack.pop()
        stack.append(x)
    return True
```

The stack can also be stored in the input array itself (overwriting a prefix),
which brings extra space down to O(1) when mutating the input is allowed.

## Complexity

- Time: O(n): each value is pushed and popped at most once.
- Space: O(n) for the stack in the worst case (a strictly decreasing input).

## Pitfalls

- Only checking each value against its immediate neighbour misses cases like
  `[8, 4, 10, 2, 6]`: the violation is against an ancestor, not the previous
  value.
- Set `low` to the *last* popped value (the closest ancestor), not the first.
- Rebuilding the tree with recursion and comparing traversals works but is
  O(n^2) on a sorted input and can recurse 10^4 deep.
