# Approach: monotonic stack

A node survives exactly when no later node is strictly larger. Scan the list
left to right and keep the current survivors on a stack; their values are
non-increasing from bottom to top. When a new node arrives, every survivor
smaller than it now has a larger node to its right, so pop those. Push the new
node. The stack at the end is the answer, in order.

```python
def remove_nodes(head):
    stack = []
    node = head
    while node:
        while stack and stack[-1].val < node.val:
            stack.pop()
        stack.append(node)
        node = node.next
    for i in range(len(stack) - 1):
        stack[i].next = stack[i + 1]
    stack[-1].next = None
    return stack[0]
```

An O(1)-space alternative: reverse the list, walk it keeping the running
maximum and unlinking nodes below it, then reverse back.

## Complexity

- Time: O(n) — every node is pushed and popped at most once.
- Space: O(n) for the stack (O(1) with the reverse-twice variant).

## Pitfalls

- Popping on `<=` instead of `<` removes equal values, which must stay.
- Forgetting to set the last survivor's `next` to `None` leaves removed nodes
  hanging off the end.
- A recursive solution recurses once per node and overflows the call stack on
  `10^5` nodes.
