# Approach: flatten to a sorted array, then binary search

Walking down a BST answers floor and ceiling in O(height), but the tree may be
a chain, making that O(n) per query and O(n * q) overall. Instead, read the
values once with an in-order traversal (sorted, since it is a BST) and answer
each query with a binary search.

```python
from bisect import bisect_left

def closest_nodes(root, queries):
    values, stack, node = [], [], root
    while stack or node:
        while node:
            stack.append(node)
            node = node.left
        node = stack.pop()
        values.append(node.val)
        node = node.right

    answer = []
    for q in queries:
        i = bisect_left(values, q)
        if i < len(values) and values[i] == q:
            answer.append([q, q])
            continue
        floor = values[i - 1] if i > 0 else -1
        ceil = values[i] if i < len(values) else -1
        answer.append([floor, ceil])
    return answer
```

## Complexity

- Time: O(n + q log n): one traversal, then a binary search per query.
- Space: O(n) for the sorted values (plus the output).

## Pitfalls

- Searching the tree per query looks natural but times out on a skewed tree.
- A recursive traversal can overflow the call stack on a deep chain; use an
  explicit stack.
- Handle the exact-match case: both floor and ceiling equal `q`.
- Off-by-one at the edges: a query below the minimum has no floor, one above
  the maximum has no ceiling.
