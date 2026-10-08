# Approach: slow and fast pointers

To unlink the middle node we need the node just before it. Run two pointers:
`slow` moves one step at a time, `fast` two. Starting `fast` two nodes ahead
of `slow` makes `slow` stop at index `floor(n / 2) - 1` instead of at the
middle itself. A one-node list has no predecessor, so it is handled up front.

```python
def delete_middle(head):
    if head.next is None:
        return None
    slow = head
    fast = head.next.next
    while fast is not None and fast.next is not None:
        slow = slow.next
        fast = fast.next.next
    slow.next = slow.next.next
    return head
```

Check with `n = 4`: `fast` starts at index 2, which has a successor, so both
pointers advance once (`slow` to index 1, `fast` off the end). `slow` stops at
index 1 and index 2 is removed. With `n = 5` the same single step happens, and
then `fast` (index 4) has no successor, so again index 2 is removed. In general
the removed index is `floor(n / 2)`.

## Complexity

- Time: O(n) — one pass over the list.
- Space: O(1).

## Pitfalls

- Stopping `slow` *on* the middle node: in a singly linked list you can't unlink
  a node without its predecessor.
- Forgetting the single-node case, where the answer is the empty list.
- Off-by-one for even lengths: the middle is index `n / 2`, the second of the
  two central nodes.
