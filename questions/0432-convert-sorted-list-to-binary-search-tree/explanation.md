# Approach: inorder construction

The required tree's inorder sequence is the list itself. So instead of finding
middles in the list (O(n) each), we simulate an inorder traversal of the tree
we are about to build, using only positions:

- `build(lo, hi)` returns the subtree for positions `lo .. hi`.
- It first builds the left part `lo .. mid-1`. Building it consumes exactly
  `mid - lo` list nodes, so afterwards the list pointer sits on position `mid`.
- That node becomes the root; advance the pointer and build `mid+1 .. hi`.

```python
def sorted_list_to_bst(head):
    n = 0
    node = head
    while node:
        n += 1
        node = node.next

    cur = head

    def build(lo, hi):
        nonlocal cur
        if lo > hi:
            return None
        mid = (lo + hi + 1) // 2
        left = build(lo, mid - 1)
        root = TreeNode(cur.val, left)
        cur = cur.next
        root.right = build(mid + 1, hi)
        return root

    return build(0, n - 1)
```

The recursion depth is only O(log n), because the tree is balanced.

## Complexity

- Time: O(n): one pass to count, one to build.
- Space: O(log n) for the recursion (the output tree itself is O(n)).

An alternative is to copy the values into an array and recurse on index
ranges: also O(n) time, but O(n) extra space. Finding the middle with slow and
fast pointers on every call costs O(n log n).

## Pitfalls

- Choosing the left middle `(lo + hi) // 2`: still balanced, but a different
  tree from the one requested.
- Losing track of the list pointer: the left subtree must be built *before*
  the root node is taken from the list.
- Forgetting the empty list.
