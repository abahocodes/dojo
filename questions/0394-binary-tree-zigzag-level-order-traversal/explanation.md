# Approach: level-order traversal, reverse every other level

The node order inside the queue never needs to change: a standard
breadth-first search produces each level left to right. On odd levels we just
emit those values reversed.

```python
def zigzag_level_order(root):
    result = []
    level = [root] if root else []
    left_to_right = True
    while level:
        values = [node.val for node in level]
        if not left_to_right:
            values.reverse()
        result.append(values)
        nxt = []
        for node in level:
            if node.left:
                nxt.append(node.left)
            if node.right:
                nxt.append(node.right)
        level = nxt
        left_to_right = not left_to_right
    return result
```

Instead of reversing, you can allocate the level's array up front and write
the `i`-th node to index `i` or `size - 1 - i`; the Java, C++ and Go solutions
do that.

## Complexity

- Time: O(n): each node is enqueued once, and reversing a level is linear in
  its size.
- Space: O(w) for the widest level, plus the output.

## Pitfalls

- Reversing the order in which children are *enqueued*: it seems natural but
  easily scrambles deeper levels. Keep the queue in plain left-to-right order
  and only reverse the output.
- Starting with the wrong direction: the root's level is left to right.
- Returning `[[]]` instead of `[]` for an empty tree.
