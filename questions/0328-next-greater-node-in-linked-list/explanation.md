# Approach: monotonic stack over the values

First copy the list into an array `vals` so that answers can be written by
index. Then scan from left to right, keeping a stack of indices whose next
larger value has not been seen yet.

When value `v` arrives, every waiting index with a smaller value has just found
its answer: `v` is the first later value larger than it. Pop those and record
`v`. The current index then starts waiting. The stack's values are
non-increasing from bottom to top, so the pops always stop at the first value
`>= v`.

```python
def next_larger_nodes(head):
    vals = []
    while head:
        vals.append(head.val)
        head = head.next
    answer = [0] * len(vals)
    waiting = []
    for i, v in enumerate(vals):
        while waiting and vals[waiting[-1]] < v:
            answer[waiting.pop()] = v
        waiting.append(i)
    return answer
```

## Complexity

- Time: O(n) — each index is pushed and popped at most once.
- Space: O(n) for the value array, the stack and the answer.

## Pitfalls

- "Larger" means strictly larger: equal values must stay on the stack
  (`<`, not `<=`, when popping).
- Answers are by position, not by value, so the stack must hold indices.
- Nodes that never find a larger value keep the default `0`.
