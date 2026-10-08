# Approach: collect coordinates, then sort

Every ordering rule in the problem is a comparison on three numbers: column
first, then row, then value. So record a `(col, row, val)` triple for every
node, sort the triples, and cut them into groups by column.

```python
def vertical_traversal(root):
    entries = []
    stack = [(root, 0, 0)]
    while stack:
        node, row, col = stack.pop()
        entries.append((col, row, node.val))
        if node.left:
            stack.append((node.left, row + 1, col - 1))
        if node.right:
            stack.append((node.right, row + 1, col + 1))
    entries.sort()
    result, prev_col = [], None
    for col, _, val in entries:
        if col != prev_col:
            result.append([])
            prev_col = col
        result[-1].append(val)
    return result
```

The traversal order doesn't matter, because the sort imposes the final order.

## Complexity

- Time: O(n log n) for the sort; the traversal is O(n).
- Space: O(n) for the triples.

## Pitfalls

- A BFS alone gives top-to-bottom order but does **not** sort values that
  share a row and column; you still need the value tie-break.
- Nodes in the same column but different rows are ordered by row, not value.
- Columns can be negative. Sort them (or offset them) rather than indexing an
  array with a raw column number.
