# Approach: right-to-left breadth-first search

BFS visits nodes level by level, and within a level in the order children were
enqueued. If we enqueue each node's **right** child before its **left** child,
every level is visited from right to left. The final node dequeued is then
the last node of the deepest level in this reversed order, which is exactly
the leftmost node of the deepest level.

```python
from collections import deque

def find_bottom_left_value(root):
    queue = deque([root])
    node = root
    while queue:
        node = queue.popleft()
        if node.right:
            queue.append(node.right)
        if node.left:
            queue.append(node.left)
    return node.val
```

**Alternative (DFS):** traverse pre-order, left before right, passing the
depth. The first time you reach a depth greater than any seen so far, record
that node's value. Left-first order guarantees the first node seen at each
depth is the leftmost one.

## Complexity

- Time: O(n): each node is enqueued and dequeued once.
- Space: O(w) for the queue, where `w` is the widest level.

## Pitfalls

- Returning the deepest *left child*: the answer may be a right child when it
  is alone on the deepest level.
- Returning the leftmost leaf of the whole tree, which may sit on a shallower
  level.
- Using a sentinel like `0` or `INT_MIN` for "no value yet": any 32-bit value
  can be the answer.
