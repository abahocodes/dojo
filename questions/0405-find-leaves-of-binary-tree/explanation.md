# Approach: group nodes by height

A node is removed in the round after its last child is removed, and leaves go
in round 0. So the round of a node is exactly its **height**: the number of
edges on the longest downward path from it to a leaf. We don't need to delete
anything. One post-order DFS computes every height and drops each value into
the matching group.

```python
def find_leaves(root):
    result = []

    def height(node):
        if node is None:
            return -1
        h = max(height(node.left), height(node.right)) + 1
        if h == len(result):
            result.append([])
        result[h].append(node.val)
        return h

    height(root)
    return result
```

**Why the order inside a group is left to right:** two nodes with the same
height can't be ancestor and descendant (an ancestor is strictly taller), so
one lies entirely to the left of the other. For such pairs, post-order and
in-order agree on which comes first.

**Why `h == len(result)` is enough:** post-order finishes a child of height
`h - 1` before its parent of height `h`, so group `h - 1` always exists by the
time a node of height `h` is reached.

## Complexity

- Time: O(n), one visit per node.
- Space: O(h) recursion depth plus the output. With at most 100 nodes the
  recursion is shallow.

## Pitfalls

- Using depth (distance from the root) instead of height. Leaves at different
  depths are removed in the same round.
- Simulating the removals works but costs O(n * h) because every round
  rescans the tree.
- Values may repeat, so group by node, never by value.
