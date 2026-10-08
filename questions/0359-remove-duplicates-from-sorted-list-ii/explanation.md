# Approach: dummy head and run skipping

Equal values form contiguous runs. Walk the list with `prev`, the last node
that is definitely kept (a dummy node at first), and `cur`. When `cur` and
`cur.next` share a value, advance `cur` past the entire run and link
`prev.next` to whatever follows; `prev` stays put because the next run might
also be a duplicate. Otherwise `cur` is unique, so `prev` moves onto it.

```python
def delete_all_duplicates(head):
    dummy = ListNode(0, head)
    prev, cur = dummy, head
    while cur:
        if cur.next and cur.next.val == cur.val:
            v = cur.val
            while cur and cur.val == v:
                cur = cur.next
            prev.next = cur
        else:
            prev = cur
            cur = cur.next
    return dummy.next
```

## Complexity

- Time: O(n) — every node is visited once.
- Space: O(1).

## Pitfalls

- Keeping one copy of each duplicate (that is a different problem).
- Advancing `prev` after removing a run; the next run may need removal too,
  as in `1 -> 1 -> 2 -> 2`.
- Forgetting that the head can be removed — return `dummy.next`, not `head`.
