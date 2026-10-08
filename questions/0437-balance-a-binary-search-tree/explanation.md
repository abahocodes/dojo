# Approach: flatten in order, rebuild from the middle

The in-order traversal of a BST lists its values in sorted order. Any sorted
array can be turned into a balanced BST by making the middle element the root
and recursing on the two halves: the halves differ in size by at most one, so
the subtree heights differ by at most one at every node.

The input tree can be a single chain of 10^4 nodes, so the traversal uses an
explicit stack. The rebuild recursion halves the range each time and is only
about log2(n) deep.

```python
def balance_bst(root):
    values = []
    stack, node = [], root
    while stack or node:
        while node:
            stack.append(node)
            node = node.left
        node = stack.pop()
        values.append(node.val)
        node = node.right

    def build(lo, hi):
        if lo > hi:
            return None
        mid = (lo + hi) // 2
        return TreeNode(values[mid], build(lo, mid - 1), build(mid + 1, hi))

    return build(0, len(values) - 1)
```

## Complexity

- Time: O(n): one traversal plus one node built per value.
- Space: O(n) for the sorted values and the new tree; the traversal stack is
  O(h) of the original tree.

## Pitfalls

- A recursive in-order traversal overflows the stack on a 10^4-long chain in
  some languages. Use an explicit stack.
- The problem fixes the middle as `(lo + hi) // 2` (the lower middle). Picking
  the upper middle is also balanced but produces a different tree.
- Do not sort the values again: the in-order traversal already returns them
  sorted, so sorting only adds an O(n log n) step.
