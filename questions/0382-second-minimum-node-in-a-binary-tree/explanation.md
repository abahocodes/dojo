# Approach: smallest value above the root's

Because each parent equals the minimum of its two children, values never
decrease going down the tree. The root therefore holds the global minimum, and
the answer is the smallest value strictly greater than `root.val`.

The same monotonicity gives a pruning rule: once a node's value exceeds the
root's, its whole subtree is at least that large, so the node itself is the
best candidate from that subtree and we need not look further down.

```python
def find_second_minimum_value(root):
    smallest = root.val
    best = -1
    stack = [root]
    while stack:
        node = stack.pop()
        if node.val > smallest:
            if best == -1 or node.val < best:
                best = node.val
            continue
        if node.left:
            stack.append(node.left)
            stack.append(node.right)
    return best
```

Collecting all values into a set and sorting it also works, at O(n log n),
and does not rely on the tree's special structure.

## Complexity

- Time: O(n) in the worst case: each node is visited at most once.
- Space: O(h) for the stack, where `h` is the tree's height.

## Pitfalls

- Returning the second smallest *node* instead of the second smallest
  *distinct value*: duplicates of the minimum do not count.
- Using `0` or `INT_MAX` as a "not found" sentinel: `2^31 - 1` can be a real
  answer, so track "not found" separately (here with `-1`, which is never a
  node value).
- Assuming the answer is one of the root's children: it can sit deep below a
  chain of nodes equal to the minimum.
