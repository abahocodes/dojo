# Approach: breadth-first search, one level at a time

A breadth-first search with a queue processes nodes in level order. Snapshot the
queue's size when a level begins, pop that many nodes while summing their
values, and divide.

```python
from collections import deque


def average_of_levels(root):
    averages = []
    queue = deque([root])
    while queue:
        size = len(queue)
        total = 0
        for _ in range(size):
            node = queue.popleft()
            total += node.val
            if node.left:
                queue.append(node.left)
            if node.right:
                queue.append(node.right)
        averages.append(total / size)
    return averages
```

**Alternative (DFS):** walk the tree with the depth attached and keep two lists,
`sums[depth]` and `counts[depth]`. Divide them pairwise at the end.

## Complexity

- Time: O(n): each node is enqueued and dequeued once.
- Space: O(w) for the queue, where `w` is the widest level.

## Pitfalls

- Summing in a 32-bit `int`: two values near `2^31 - 1` overflow. Use a
  64-bit integer (`long`, `long long`) for the sum.
- Integer division: convert to floating point before dividing, or `5.5`
  becomes `5`.
- Letting the loop bound change while children are pushed; capture the size
  before the inner loop.
