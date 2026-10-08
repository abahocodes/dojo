# Approach: two pointers with a gap of n

Put a dummy node in front of the list so that removing the real head is not a
special case. Advance a `fast` pointer `n` nodes ahead, then move `fast` and
`slow` together until `fast` sits on the last node. Because the gap stays at
`n`, `slow` now stands just before the node that is `n`-th from the end, so we
unlink it.

```python
def remove_nth_from_end(head, n):
    dummy = ListNode(0, head)
    fast = slow = dummy
    for _ in range(n):
        fast = fast.next
    while fast.next:
        fast = fast.next
        slow = slow.next
    slow.next = slow.next.next
    return dummy.next
```

Two-pass alternative: count the length `L`, then walk `L - n` steps from the
dummy and unlink the next node. Same complexity, just two traversals.

## Complexity

- Time: O(L): one pass over the list.
- Space: O(1).

## Pitfalls

- Removing the head (`n == L`): without a dummy node you must special-case it.
- Off-by-one in the gap: `slow` must stop *before* the target, not on it.
- A single-node list with `n = 1` must return an empty list (`None`).
