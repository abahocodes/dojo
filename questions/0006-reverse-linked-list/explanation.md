# Approach: iterative pointer reversal

Walk the list while maintaining two pointers: `prev`, the head of the already
reversed part, and `cur`, the next node to process. Each step detaches `cur`,
points it back at `prev`, and moves both pointers forward.

```python
def reverse_list(head):
    prev = None
    cur = head
    while cur:
        nxt = cur.next
        cur.next = prev
        prev = cur
        cur = nxt
    return prev
```

Recursive alternative: reverse everything after `head`, then hook `head` onto
the end — `head.next.next = head; head.next = None`. It is elegant but uses
O(n) call stack, which can hit Python's recursion limit on long lists.

## Complexity

- Time: O(n) — each node is visited once.
- Space: O(1) iterative; O(n) recursive.

## Pitfalls

- Overwriting `cur.next` before saving it loses the rest of the list.
- Returning `head` instead of `prev` — the old head is now the tail.
- Forgetting to handle the empty list (`head is None`); the loop above does it
  naturally.
