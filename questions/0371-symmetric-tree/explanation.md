# Approach: compare mirrored pairs with a stack

A tree is symmetric when its left and right subtrees are mirrors of each other.
Two subtrees `a` and `b` are mirrors when:

- both are empty, or
- both exist, `a.val == b.val`, `a.left` mirrors `b.right`, and `a.right`
  mirrors `b.left`.

Instead of recursing, keep a stack of pairs that still have to be checked:

```python
def is_symmetric(root):
    stack = [(root.left, root.right)]
    while stack:
        a, b = stack.pop()
        if a is None and b is None:
            continue
        if a is None or b is None or a.val != b.val:
            return False
        stack.append((a.left, b.right))
        stack.append((a.right, b.left))
    return True
```

A queue works equally well (breadth-first comparison). The recursive version
is a direct translation of the definition and is fine at this tree size.

## Complexity

- Time: O(n) — each node is in at most one pair.
- Space: O(n) for the stack in the worst case.

## Pitfalls

- Comparing `a.left` with `b.left` checks that the two halves are *equal*, not
  mirrored.
- Checking only that each level's values read the same forwards and backwards
  ignores missing children (Example 2 would wrongly pass).
- Comparing values before checking for `None`.
