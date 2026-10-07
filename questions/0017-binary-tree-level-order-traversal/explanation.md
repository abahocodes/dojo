# Approach: BFS one level at a time

Breadth-first search visits nodes in order of depth. The only extra work is to
cut the stream of nodes into levels. When a new level starts, the queue holds
exactly the nodes of that level, so we snapshot its length and pop that many.

```python
from collections import deque

def level_order(root):
    if root is None:
        return []
    result, queue = [], deque([root])
    while queue:
        level = []
        for _ in range(len(queue)):
            node = queue.popleft()
            level.append(node.val)
            if node.left:
                queue.append(node.left)
            if node.right:
                queue.append(node.right)
        result.append(level)
    return result
```

**Alternative (DFS):** do a pre-order traversal that passes the depth along.
When `depth == len(result)`, append a new empty list. Then append the value to
`result[depth]`. Visiting left before right keeps each level in left-to-right
order.

## Complexity

- Time: O(n): each node is enqueued and dequeued once.
- Space: O(w) for the queue, where `w` is the widest level (up to about n/2),
  plus the output.

## Pitfalls

- Using `len(queue)` as the loop bound while you push children: capture the
  size *before* the inner loop (Python's `range(len(queue))` does this).
- `list.pop(0)` is O(n). Use `collections.deque`.
- Push the left child before the right, or each level comes out reversed.
- Return `[]` for an empty tree, not `[[]]`.
