# Approach: level-order scan with a "previous value" check

A breadth-first search produces each level from left to right, which is
exactly the order the rules are stated in. For every level, check each value's
parity and compare it with the value just before it on the same level.

```python
def is_even_odd_tree(root):
    level = [root]
    depth = 0
    while level:
        even_level = depth % 2 == 0
        prev = None
        nxt = []
        for node in level:
            v = node.val
            if even_level:
                if v % 2 == 0 or (prev is not None and v <= prev):
                    return False
            else:
                if v % 2 == 1 or (prev is not None and v >= prev):
                    return False
            prev = v
            if node.left:
                nxt.append(node.left)
            if node.right:
                nxt.append(node.right)
        level = nxt
        depth += 1
    return True
```

In typed languages, a sentinel just outside the value range replaces `None`:
`0` before the first value of an increasing level and a huge value before the
first value of a decreasing level.

## Complexity

- Time: O(n): each node is checked once.
- Space: O(w) for the widest level.

## Pitfalls

- **Strictly** increasing / decreasing: equal neighbours (Example 2) break the
  rule.
- Forgetting to reset the previous value at each new level: the last value of
  one level must not be compared with the first value of the next.
- Mixing up which level wants which parity: level 0 holds odd values.
