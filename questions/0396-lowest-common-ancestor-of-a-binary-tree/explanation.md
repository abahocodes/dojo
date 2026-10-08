# Approach: parent pointers, then climb

The lowest common ancestor is the point where the root-to-`p` path and the
root-to-`q` path split. Nodes don't point upward, so first build a map from
each value to its parent's value with an iterative DFS. Then collect `p` and
every ancestor of `p` in a set, and climb from `q` until you hit a member of
that set. The first one you hit is the deepest shared ancestor.

```python
def lowest_common_ancestor(root, p, q):
    parent = {root.val: None}
    stack = [root]
    while stack and (p not in parent or q not in parent):
        node = stack.pop()
        for child in (node.left, node.right):
            if child is not None:
                parent[child.val] = node.val
                stack.append(child)
    ancestors = set()
    while p is not None:
        ancestors.add(p)
        p = parent[p]
    while q not in ancestors:
        q = parent[q]
    return q
```

**Alternative (recursive):** `lca(node)` returns `node` if it is `None`, `p`
or `q`; otherwise it recurses into both children. If both sides return
something, `node` is the answer; otherwise pass up whichever side was
non-empty. It is elegant, but its recursion depth equals the tree height, which
can reach 10^5 here.

## Complexity

- Time: O(n): each node is visited at most once, and each climb is at most the
  tree height.
- Space: O(n) for the parent map and the ancestor set.

## Pitfalls

- The tree is not a BST: you can't steer left or right by comparing values.
- A node is its own ancestor. If `q` lies below `p`, the answer is `p`, which
  the climb handles because `p` itself goes into the set.
- Recursion on a chain-shaped tree with 10^5 nodes overflows the default
  Python (and often JavaScript) call stack.
