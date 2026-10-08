# Approach: splice each left subtree in place

Walk a cursor down the right pointers. When the cursor has a left subtree, that
subtree must come next in preorder, and the cursor's current right subtree must
follow the *last* preorder node of the left subtree, which is its rightmost
node. So:

1. find `tail`, the rightmost node of `node.left`;
2. `tail.right = node.right`;
3. `node.right = node.left`, `node.left = None`;
4. move on to `node.right`.

```python
def flatten(root):
    node = root
    while node is not None:
        if node.left is not None:
            tail = node.left
            while tail.right is not None:
                tail = tail.right
            tail.right = node.right
            node.right = node.left
            node.left = None
        node = node.right
    return root
```

**Alternative:** a stack-based preorder (push right, then left) that links each
popped node to the previous one. It is just as fast but uses O(h) extra space.

## Complexity

- Time: O(n). Each search for `tail` walks a right spine that is then spliced
  behind the cursor, so every node is walked over a constant number of times.
- Space: O(1) extra.

## Pitfalls

- Forgetting to clear `left` leaves a tree that isn't a chain.
- Attaching `node.right` to `node.left` directly (instead of to the rightmost
  node of the left subtree) drops nodes when the left subtree is larger than
  one node.
- Recursive versions can hit the call-stack limit on a 2000-node chain in some
  languages.
