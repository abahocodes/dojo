# Approach: merge into the first tree with a stack of pairs

At any position, if one tree is missing a node the merged subtree is simply the
other tree's subtree, so we can attach it as-is. Only positions where both trees
have nodes need work: add the values and continue with the children.

```python
def merge_trees(root1, root2):
    if root1 is None:
        return root2
    stack = [(root1, root2)]
    while stack:
        a, b = stack.pop()
        if b is None:
            continue
        a.val += b.val
        if a.left is None:
            a.left = b.left
        else:
            stack.append((a.left, b.left))
        if a.right is None:
            a.right = b.right
        else:
            stack.append((a.right, b.right))
    return root1
```

The recursive form reads the same way: return the other tree when one side is
missing, otherwise add the values and merge the left and right children.

## Complexity

- Time: O(min(n1, n2)): only positions present in both trees are visited;
  the rest is attached in O(1).
- Space: O(h) for the stack, bounded by the smaller tree's height.

## Pitfalls

- Copying the leftover subtree node by node is correct but slower; attaching
  it is enough.
- Returning `root1` when it is empty: the answer is then `root2`.
- Forgetting that a node present only in `root2` keeps its whole subtree.
