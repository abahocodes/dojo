# Approach: splice into a sorted list behind a dummy head

Maintain a sorted list behind a dummy node. Take input nodes one by one; for
each, find the last sorted node whose value is `<=` the new value and splice the
new node after it. Tracking the tail lets already-sorted runs append in O(1).

```python
def insertion_sort_list(head):
    dummy = ListNode(0)
    tail = None
    cur = head
    while cur:
        nxt = cur.next
        if tail is not None and tail.val <= cur.val:
            tail.next = cur
            cur.next = None
            tail = cur
        else:
            p = dummy
            while p.next and p.next.val <= cur.val:
                p = p.next
            cur.next = p.next
            p.next = cur
            if cur.next is None:
                tail = cur
        cur = nxt
    return dummy.next
```

## Complexity

- Time: O(n^2) in the worst case (each insertion may scan the whole sorted
  list); O(n) for already-sorted input thanks to the tail shortcut.
- Space: O(1) — nodes are relinked, never copied.

## Pitfalls

- Not saving `cur.next` before splicing `cur` into the sorted list.
- Inserting at the front without a dummy node needs a special case that is
  easy to get wrong.
- Forgetting to update `tail` when a node is inserted at the end by the scan.
