# Approach: carry the prefix down a DFS

Appending a digit `d` to a number `x` gives `10 * x + d`. So each node's value
is fully determined by its parent's value, and a depth-first walk that carries
the prefix along can add up the leaf values directly.

```python
def sum_numbers(root):
    total = 0
    stack = [(root, 0)]
    while stack:
        node, prefix = stack.pop()
        value = prefix * 10 + node.val
        if node.left is None and node.right is None:
            total += value
            continue
        if node.left:
            stack.append((node.left, value))
        if node.right:
            stack.append((node.right, value))
    return total
```

## Complexity

- Time: O(n): every node is visited once.
- Space: O(h) for the stack, where h is the height (at most 10 here).

## Pitfalls

- Only leaves contribute. A node with one child is not the end of a path, so
  adding its value would double count.
- Don't build strings and parse them at the end; the arithmetic prefix is
  simpler and faster.
- A single node is both the root and a leaf: the answer is its digit.
