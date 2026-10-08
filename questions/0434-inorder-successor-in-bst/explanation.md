# Approach: descend like a search, remembering candidates

Every value greater than `p` is a candidate. Walking down from the root:

- If `node.val > p`, this node is a candidate, and every candidate in its right
  subtree is larger than it, so only the left subtree can hold a better one.
  Record it and go left.
- Otherwise (`node.val <= p`), the node and its whole left subtree are too
  small; go right.

The last candidate recorded is the smallest value above `p`. This covers both
classic cases at once: if `p` has a right subtree the walk ends at that
subtree's leftmost node, and if not, the answer is the lowest ancestor whose
left subtree contains `p`.

```python
def inorder_successor(root, p):
    answer = -1
    node = root
    while node:
        if node.val > p:
            answer = node.val
            node = node.left
        else:
            node = node.right
    return answer
```

## Complexity

- Time: O(h).
- Space: O(1).

## Pitfalls

- Using `>=` instead of `>`: the successor is strictly greater than `p`.
- Only looking in `p`'s right subtree: when it is empty, the successor is an
  ancestor.
- Producing the full inorder list: correct, but O(n) time and space instead of
  O(h).
