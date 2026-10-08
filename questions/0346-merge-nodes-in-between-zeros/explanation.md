# Approach: running sum, emit at each zero

Because the list starts and ends with `0` and zeros are never adjacent, every
`0` after the first one ends exactly one non-empty group. Keep a running sum;
when a `0` arrives, the sum is that group's total.

To avoid allocating, reuse each closing `0` node as the output node: overwrite
its value with the group sum and link it after the previous output node.

```python
def merge_nodes(head):
    dummy = ListNode()
    tail = dummy
    total = 0
    cur = head.next
    while cur:
        if cur.val == 0:
            cur.val = total
            tail.next = cur
            tail = cur
            total = 0
        else:
            total += cur.val
        cur = cur.next
    tail.next = None
    return dummy.next
```

Re-linking `tail.next = cur` only changes a node that has already been passed,
so the walk itself is not disturbed. The last node of the input is the last
output node, so its `next` is already `None`; the final assignment just makes
that explicit.

## Complexity

- Time: O(n).
- Space: O(1) extra (nodes are reused).

## Pitfalls

- Emitting a node at the very first `0` produces a spurious `0` at the front.
- Forgetting to reset the running sum after each group.
- Group sums reach at most `2 * 10^8`, which fits in a 32-bit integer.
