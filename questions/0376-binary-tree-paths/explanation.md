# Approach: depth-first search carrying the path text

Every root-to-leaf path is discovered by a depth-first walk. Carry the string
for the path so far with each node; when the walk reaches a leaf, that string
is one answer.

```python
def binary_tree_paths(root):
    paths = []
    stack = [(root, str(root.val))]
    while stack:
        node, path = stack.pop()
        if node.left is None and node.right is None:
            paths.append(path)
            continue
        if node.right:
            stack.append((node.right, path + "->" + str(node.right.val)))
        if node.left:
            stack.append((node.left, path + "->" + str(node.left.val)))
    return paths
```

A backtracking variant keeps one shared list of values, appends on the way
down, joins it at each leaf and pops on the way back up. It builds each answer
string only once.

## Complexity

- Time: O(n * h): there are at most `n` leaves and each string has up to `h`
  values, where `h` is the tree height. The output itself can be that large.
- Space: O(n * h) for the strings held on the stack and in the output.

## Pitfalls

- Emitting a path at a node with only one child: it is not a leaf.
- Leaving a trailing `"->"` or adding spaces around the arrow.
- Dropping the minus sign of negative values.
