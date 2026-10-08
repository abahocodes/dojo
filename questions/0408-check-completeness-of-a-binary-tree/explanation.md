# Approach: BFS that stops trusting the tree after the first gap

Breadth-first order visits positions `1, 2, 3, ...` of the tree in exactly
the numbering from the statement. If we also enqueue missing children as
`null`, a complete tree produces a run of real nodes followed only by `null`s.
Any real node that shows up after a `null` means some earlier position was
empty, so the tree is not complete.

```python
from collections import deque

def is_complete_tree(root):
    queue = deque([root])
    seen_gap = False
    while queue:
        node = queue.popleft()
        if node is None:
            seen_gap = True
            continue
        if seen_gap:
            return False
        queue.append(node.left)
        queue.append(node.right)
    return True
```

**Alternative (numbering):** give the root index 1 and the children of index
`i` the indices `2i` and `2i + 1`. The tree is complete exactly when the
largest index equals the node count. This needs care with overflow on deep
trees in fixed-width languages; the BFS has no such issue.

## Complexity

- Time: O(n). Each node is enqueued once, plus at most `n + 1` nulls.
- Space: O(n) for the queue.

## Pitfalls

- Checking only that every node with a right child also has a left child.
  That misses gaps *between* nodes, as in Example 2.
- Checking each level separately and forgetting that once a level is not
  full, no deeper level may contain any node.
- In Java, `ArrayDeque` throws on `null` elements. Use a `LinkedList` or a
  list with a read index.
