# Approach: compare the two root-to-node paths

Let `P` be the moves from the root to the start node and `Q` the moves from
the root to the destination. Both begin with the moves from the root to the
**lowest common ancestor** (LCA), which is exactly their longest common prefix.
After that they diverge.

The shortest walk climbs from the start up to the LCA (one `'U'` for every
move left in `P` after the prefix) and then follows the rest of `Q` down.

To get the paths without recursion, traverse the tree once with a stack and
record, for every value, its parent and whether it is a left or right child.
Then walk up from each target to the root and reverse.

```python
def get_directions(root, start_value, dest_value):
    parent = {root.val: (None, "")}
    stack = [root]
    while stack:
        node = stack.pop()
        if node.left is not None:
            parent[node.left.val] = (node.val, "L")
            stack.append(node.left)
        if node.right is not None:
            parent[node.right.val] = (node.val, "R")
            stack.append(node.right)

    def path_from_root(value):
        moves = []
        while parent[value][0] is not None:
            value, move = parent[value]
            moves.append(move)
        moves.reverse()
        return moves

    to_start = path_from_root(start_value)
    to_dest = path_from_root(dest_value)
    common = 0
    while (common < len(to_start) and common < len(to_dest)
           and to_start[common] == to_dest[common]):
        common += 1
    return "U" * (len(to_start) - common) + "".join(to_dest[common:])
```

## Complexity

- Time: O(n) for the traversal, plus O(h) to build and compare the two paths.
- Space: O(n) for the parent map.

## Pitfalls

- Turning the start's remaining moves into their opposites (`'L'` into `'R'`):
  going back up is always `'U'`, whichever side you came from.
- Forgetting to strip the common prefix. That routes the walk through the
  root even when the LCA is much lower.
- One node may be an ancestor of the other. Then one remaining path is empty
  and the answer is all `'U'`s or has no `'U'`s at all.
- Building strings by repeated concatenation in a loop is O(h^2) in some
  languages. Use a list or builder.
- Recursive path search can overflow on path-shaped trees with `10^5` nodes.
