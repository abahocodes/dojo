# Approach: find the inversions in inorder

In a valid BST the inorder sequence is sorted. Swapping two values creates
either one "drop" (`prev.val > node.val`), when they are adjacent in inorder, or
two drops otherwise. In both cases:

- the first misplaced node is `prev` at the **first** drop (the too-large value
  that moved left), and
- the second misplaced node is `node` at the **last** drop (the too-small value
  that moved right).

Run an iterative inorder traversal with an explicit stack, collect those two
nodes, and swap their values.

```python
def recover_tree(root):
    first = second = prev = None
    stack, node = [], root
    while stack or node:
        while node:
            stack.append(node)
            node = node.left
        node = stack.pop()
        if prev is not None and prev.val > node.val:
            if first is None:
                first = prev
            second = node
        prev = node
        node = node.right
    first.val, second.val = second.val, first.val
    return root
```

For the O(1)-space follow-up, replace the stack with a **Morris traversal**:
for each node with a left child, find its inorder predecessor; if the
predecessor's right pointer is empty, point it back at the node and go left;
otherwise remove that temporary thread, visit the node and go right. The drop
detection is unchanged, and every temporary link is removed before the
traversal ends, so the tree's shape is restored.

## Complexity

- Time: O(n).
- Space: O(h) for the stack (O(1) with Morris traversal).

## Pitfalls

- Only handling the two-drop case: when the swapped nodes are adjacent in
  inorder (e.g. parent and child) there is a single drop and both nodes come
  from it.
- Taking `node` instead of `prev` at the first drop, or `prev` instead of
  `node` at the last.
- Swapping node pointers or subtrees instead of just the two values.
- Using sentinel values like `INT_MIN` for `prev`: node values may be the
  extreme 32-bit integers, so start with "no previous node" instead.
