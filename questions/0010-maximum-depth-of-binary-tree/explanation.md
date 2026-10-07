# Approach: recursion or level-by-level BFS

The depth of a tree is one more than the deeper of its two subtrees, and an
empty tree has depth zero. That translates directly into a short recursion:

```python
def max_depth(root):
    if root is None:
        return 0
    return 1 + max(max_depth(root.left), max_depth(root.right))
```

An iterative version processes the tree one level at a time and counts levels.
It never recurses, so a long, one-sided tree can't overflow the call stack:

```python
def max_depth(root):
    if root is None:
        return 0
    depth, level = 0, [root]
    while level:
        depth += 1
        level = [c for n in level for c in (n.left, n.right) if c]
    return depth
```

## Complexity

- Time: O(n): every node is visited once.
- Space: O(h) for the recursion, where `h` is the height (O(n) for a skewed
  tree). The BFS version holds at most one level, up to O(n) for a wide tree.

## Pitfalls

- Return `0` for an empty tree, not `1`.
- Counting edges instead of nodes gives an answer that is off by one.
- In languages with small stacks, a degenerate "linked list" tree with
  thousands of nodes can overflow a recursive solution. Use BFS or an explicit
  stack there.
