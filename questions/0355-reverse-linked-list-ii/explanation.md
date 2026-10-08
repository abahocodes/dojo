# Approach: dummy node and head insertion

Walk to `before`, the node at position `left - 1` (a dummy node stands in when
`left == 1`). The first node of the run, `tail`, will end up last in the
reversed run. Then, `right - left` times, pull the node right after `tail` out
and insert it directly after `before`. Each move puts the next node of the run
at the front, which reverses the run while keeping it connected to the rest of
the list throughout.

```python
def reverse_between(head, left, right):
    dummy = ListNode(0, head)
    before = dummy
    for _ in range(left - 1):
        before = before.next
    tail = before.next
    for _ in range(right - left):
        move = tail.next
        tail.next = move.next
        move.next = before.next
        before.next = move
    return dummy.next
```

## Complexity

- Time: O(n) — one walk to `left`, then `right - left` constant-time moves.
- Space: O(1).

## Pitfalls

- Returning `head` instead of `dummy.next`: when `left == 1` the old head is no
  longer first.
- Inserting after `tail` instead of after `before`, which leaves the run in the
  original order.
- Mixing up 0-based and 1-based positions while walking to `before`.
