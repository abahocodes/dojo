# Approach: prefix sums along the current path

Define `prefix(v)` as the sum from the root to `v`, inclusive. A downward path
ending at `v` that starts just below some ancestor `a` (or at the root, using
an imaginary prefix of `0`) has sum `prefix(v) - prefix(a)`. So the number of
valid paths ending at `v` equals the number of ancestors-or-root-sentinel whose
prefix is `prefix(v) - target_sum`.

Keep a hash map of how often each prefix sum occurs on the **current**
root-to-node path. Add the current node's prefix when you enter it and remove
it when you leave, so sibling branches never see each other's sums. The DFS
below is iterative: each node is pushed a second time as a "leaving" marker.

```python
def path_sum_count(root, target_sum):
    if root is None:
        return 0
    seen = {0: 1}
    count = 0
    stack = [(root, 0, False)]
    while stack:
        node, before, leaving = stack.pop()
        prefix = before + node.val
        if leaving:
            seen[prefix] -= 1
            continue
        count += seen.get(prefix - target_sum, 0)
        seen[prefix] = seen.get(prefix, 0) + 1
        stack.append((node, before, True))
        if node.right is not None:
            stack.append((node.right, prefix, False))
        if node.left is not None:
            stack.append((node.left, prefix, False))
    return count
```

## Complexity

- Time: O(n) expected: one hash-map lookup and update per node.
- Space: O(h) for the map and stack, where `h` is the height (O(n) worst
  case).

## Pitfalls

- Seed the map with `{0: 1}` so paths that start at the root are counted.
- Look up `prefix - target_sum` **before** inserting the current prefix, or a
  `target_sum` of `0` would count the empty path.
- Forgetting to remove the prefix when backtracking lets one branch match
  against sums from a sibling branch.
- Values reach 10^9 and paths have up to 1000 nodes: 32-bit sums overflow.
