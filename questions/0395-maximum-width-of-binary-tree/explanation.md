# Approach: BFS with heap-style positions, re-based per level

Give the root position `0`; a node at position `p` has children at `2p` and
`2p + 1`. These are exactly the slot numbers of a complete-tree layout, so a
level's width is `last - first + 1`.

Positions double on every level, so a 3000-level chain would need numbers with
3000 bits. Only differences within a level matter, so at the start of each
level we subtract that level's first position (`base`). After re-basing, every
position on a level is at most the width (which fits in 32 bits), and the
children's positions fit comfortably in 64 bits.

```python
def width_of_binary_tree(root):
    best = 0
    level = [(root, 0)]
    while level:
        base = level[0][1]
        best = max(best, level[-1][1] - base + 1)
        nxt = []
        for node, pos in level:
            pos -= base
            if node.left:
                nxt.append((node.left, 2 * pos))
            if node.right:
                nxt.append((node.right, 2 * pos + 1))
        level = nxt
    return best
```

## Complexity

- Time: O(n): each node is enqueued once.
- Space: O(w) for the widest level.

## Pitfalls

- Overflow: without re-basing, positions exceed 64 bits on deep trees. Even
  with re-basing, `2 * pos + 1` can exceed 32 bits when the width is near
  `2^31`, so use 64-bit positions (`long`, `long long`, Go's `int`).
- Counting only the nodes on a level instead of the slots between the ends:
  the empty slots in between count.
- Slots outside the leftmost and rightmost nodes do not count.
