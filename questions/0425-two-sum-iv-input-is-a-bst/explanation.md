# Approach: sorted values + two pointers

An in-order traversal of the BST produces the values in ascending order. Two
pointers then solve two-sum on the sorted list in linear time:

- If `values[i] + values[j] == k`, the answer is `true`.
- If the sum is too small, only a larger left value can help, so move `i`
  forward.
- If the sum is too large, move `j` back.

The loop runs only while `i < j`, so a node is never paired with itself.

```python
def find_target(root: "TreeNode", k: int) -> bool:
    # An inorder walk lists the values in ascending order.
    values = []
    stack, node = [], root
    while stack or node:
        while node:
            stack.append(node)
            node = node.left
        node = stack.pop()
        values.append(node.val)
        node = node.right

    i, j = 0, len(values) - 1
    while i < j:
        s = values[i] + values[j]
        if s == k:
            return True
        if s < k:
            i += 1
        else:
            j -= 1
    return False
```

**Alternative:** traverse the tree once with a hash set and check whether
`k - node.val` was already seen before adding `node.val`. It is also `O(n)`,
and it does not need the BST ordering.

## Complexity

- Time: `O(n)` for the traversal plus `O(n)` for the two-pointer scan.
- Space: `O(n)` for the list of values. A version with two in-order iterators,
  one forward and one backward, uses only `O(h)`.

## Pitfalls

- Do not count a value twice: for `k = 6` and a node `3`, `3 + 3` is not
  allowed. With a hash set, check for `k - v` *before* inserting `v`.
- Searching the BST for `k - v` from the root for every `v` is
  `O(n * h)`, which degrades to `O(n^2)` on a skewed tree.
