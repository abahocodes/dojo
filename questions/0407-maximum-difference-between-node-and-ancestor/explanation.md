# Approach: carry the path's minimum and maximum

Any two distinct nodes on a single root-to-node path are an ancestor and a
descendant, and every ancestor/descendant pair lies on such a path. So the
answer is the largest value of `max(path) - min(path)` over all downward paths
starting at the root.

Walk the tree from the root, carrying the smallest (`lo`) and largest (`hi`)
values seen on the way down. At each node, fold in its value and update the
answer with `hi - lo`. An explicit stack keeps it safe on deep trees.

```python
def max_ancestor_diff(root):
    best = 0
    stack = [(root, root.val, root.val)]
    while stack:
        node, lo, hi = stack.pop()
        lo = min(lo, node.val)
        hi = max(hi, node.val)
        best = max(best, hi - lo)
        if node.left is not None:
            stack.append((node.left, lo, hi))
        if node.right is not None:
            stack.append((node.right, lo, hi))
    return best
```

**Alternative:** a post-order DFS that returns each subtree's min and max and
compares them against the current node. Same O(n), but the top-down version
is shorter.

## Complexity

- Time: O(n), one visit per node.
- Space: O(h) for the stack frames on the current path (O(n) in the worst
  case for a path-shaped tree).

## Pitfalls

- Pairing the global minimum with the global maximum: they may sit in
  different branches, where neither is an ancestor of the other.
- Comparing each node only with its parent misses gaps that span several
  levels.
- Recursion depth: a 5000-node path overflows Python's default recursion
  limit. Use an explicit stack.
