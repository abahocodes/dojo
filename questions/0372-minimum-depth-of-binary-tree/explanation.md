# Approach: breadth-first search, stop at the first leaf

Breadth-first search visits nodes level by level, so the first leaf it reaches
is a shallowest leaf. We can return as soon as we see it, without touching the
deeper parts of the tree.

```python
from collections import deque


def min_depth(root):
    if root is None:
        return 0
    queue = deque([(root, 1)])
    while queue:
        node, depth = queue.popleft()
        if node.left is None and node.right is None:
            return depth
        if node.left:
            queue.append((node.left, depth + 1))
        if node.right:
            queue.append((node.right, depth + 1))
    return 0
```

**Alternative (DFS):** recursively, a leaf has depth 1; a node with one child
has `1 +` that child's depth; a node with two children takes `1 + min` of
both. It visits every node and can recurse as deep as the tree, so on a long
chain prefer an explicit stack.

## Complexity

- Time: O(n) in the worst case; BFS stops early when a shallow leaf exists.
- Space: O(w) for the queue, where `w` is the widest level reached.

## Pitfalls

- `1 + min(left, right)` with an empty side returns 1 for a node that is not a
  leaf. A missing child is not a path to a leaf.
- Return `0` for an empty tree.
- Plain recursion on a chain of `10^5` nodes overflows the stack in most
  languages.
