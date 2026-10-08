# Approach: follow the search path

Searching for `target` in a BST visits a single root-to-leaf path. Every node
that is *not* on that path is cut off by some ancestor on the path: when we go
left at a node `v`, everything in `v`'s right subtree is even farther above the
target than `v` is. So the nearest value below the target (its floor) and the
nearest value above it (its ceiling) both lie on the path, and the answer is
one of those two.

We therefore walk the search path once, keeping the best value seen so far.
The tie rule (equal distance, prefer the smaller value) is applied in the
comparison.

```python
def closest_value(root, target):
    best = root.val
    node = root
    while node:
        v = node.val
        d, bd = abs(v - target), abs(best - target)
        if d < bd or (d == bd and v < best):
            best = v
        if target < v:
            node = node.left
        elif target > v:
            node = node.right
        else:
            break
    return best
```

## Complexity

- Time: O(h), where `h` is the height of the tree (O(log n) when balanced,
  O(n) for a chain).
- Space: O(1).

## Pitfalls

- Forgetting the tie rule: with `target = 5.0` and values `3` and `7`, the
  answer must be `3`, so a strict `<` alone is not enough when the larger value
  is seen first.
- Visiting the whole tree works but throws away the BST property; the search
  path is all you need.
- Mixing integer and floating-point arithmetic carelessly (e.g. truncating the
  target to an integer before comparing).
