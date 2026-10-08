# Approach: iterative reversal group by group

Put a sentinel `dummy` before the head and keep `group_prev`, the node just
before the group being processed.

1. Walk `k` steps from `group_prev` to find `kth`, the last node of the group.
   If the list ends first, the remaining nodes form an incomplete group: stop.
2. Reverse the `k` nodes starting at `first = group_prev.next`. Initialising
   `prev = kth.next` makes the old first node point at the start of the next
   group automatically.
3. Hook the reversed group in: `group_prev.next = kth`. The old `first` is now
   the group's tail, so it becomes the next `group_prev`.

```python
def reverse_k_group(head, k):
    dummy = ListNode(0, head)
    group_prev = dummy
    while True:
        kth = group_prev
        for _ in range(k):
            kth = kth.next
            if kth is None:
                return dummy.next
        first = group_prev.next
        prev, cur = kth.next, first
        for _ in range(k):
            nxt = cur.next
            cur.next = prev
            prev, cur = cur, nxt
        group_prev.next = kth
        group_prev = first
```

A recursive version (reverse the first group, recurse on the rest) is shorter,
but uses O(n / k) call-stack space.

## Complexity

- Time: O(n) — each node is visited a constant number of times (once while
  looking ahead, once while reversing).
- Space: O(1) extra.

## Pitfalls

- Reversing the trailing incomplete group: always check that `k` nodes exist
  before touching any pointer.
- Losing the link to the next group: either remember `kth.next` before
  reversing or start `prev` there as above.
- Forgetting to advance `group_prev` to the old first node of the group (now
  its tail).
- `k = 1` must leave the list unchanged.
