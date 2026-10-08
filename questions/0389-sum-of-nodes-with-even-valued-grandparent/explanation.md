# Approach: let each even node collect its grandchildren

Every node has at most one grandparent, so the answer is the same as summing,
over all even-valued nodes, the values of their grandchildren. Traverse the
tree with an explicit stack; for each node and each of its children, if the
node is even, add the child's own children.

```python
def sum_even_grandparent(root):
    total = 0
    stack = [root]
    while stack:
        node = stack.pop()
        for child in (node.left, node.right):
            if child is None:
                continue
            if node.val % 2 == 0:
                if child.left:
                    total += child.left.val
                if child.right:
                    total += child.right.val
            stack.append(child)
    return total
```

**Alternative:** a DFS that passes the parent's and grandparent's values down
to each child, and adds the current node's value when the grandparent value
exists and is even.

## Complexity

- Time: O(n): each node is pushed once and looks at most four grandchildren.
- Space: O(h) for the stack in the worst case (O(n) for a chain).

## Pitfalls

- Adding the even node's *children* instead of its *grandchildren*.
- Treating a missing grandparent as value `0`, which is even, and wrongly
  counting the root's children.
- Recursion depth on a 10^4-node chain: prefer an explicit stack.
