# Approach: monotonic decreasing stack

Process the values from left to right while keeping the **right spine** of
the tree built so far on a stack, whose values decrease from bottom to top.

When a new value `x` arrives:

- Every spine node smaller than `x` lies to the left of `x` with nothing
  larger between them, so they belong in `x`'s left subtree. Pop them; the
  last one popped is the largest of them and becomes `x.left` (the others are
  already hanging below it).
- The remaining top, if any, is larger than `x` and is the nearest such value
  on the left, so `x` becomes its right child (replacing any old right child,
  which now sits under `x`).
- Push `x`; it is the new end of the right spine.

The first element on the stack is never popped by anything larger, so it is
the overall maximum: the root.

```python
def construct_maximum_binary_tree(nums):
    stack = []
    for x in nums:
        node = TreeNode(x)
        last = None
        while stack and stack[-1].val < x:
            last = stack.pop()
        node.left = last
        if stack:
            stack[-1].right = node
        stack.append(node)
    return stack[0]
```

The direct recursive build (find the max, split, recurse) is correct too but
costs O(n^2) on sorted inputs and recurses n levels deep.

## Complexity

- Time: O(n): each node is pushed and popped at most once.
- Space: O(n) for the stack and the tree.

## Pitfalls

- The left child is the **last** node popped, not the first.
- Assign `stack[-1].right = node` after popping, so the parent is the nearest
  larger value.
- Return the bottom of the stack, not the top.
