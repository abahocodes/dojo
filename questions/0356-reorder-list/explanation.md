# Approach: split, reverse, weave

The target order takes one node from the front, then one from the back, and so
on. That is exactly the first half of the list merged, node by node, with the
second half reversed.

1. Use slow/fast pointers to find the end of the first half. With `fast`
   stopping when it cannot take two more steps, the first half gets the extra
   node when `n` is odd.
2. Cut the list after `slow` and reverse the second half in place.
3. Weave: take a node from the first half, then one from the reversed second
   half, until the second half runs out.

```python
def reorder_list(head):
    if head is None or head.next is None:
        return head
    slow = fast = head
    while fast.next and fast.next.next:
        slow = slow.next
        fast = fast.next.next
    second = slow.next
    slow.next = None
    prev = None
    while second:
        nxt = second.next
        second.next = prev
        prev = second
        second = nxt
    first, second = head, prev
    while second:
        n1, n2 = first.next, second.next
        first.next = second
        second.next = n1
        first, second = n1, n2
    return head
```

## Complexity

- Time: O(n) — finding the middle, reversing and weaving are each one pass.
- Space: O(1) — only a handful of pointers.

## Pitfalls

- Forgetting `slow.next = None`: the first half still points into the second
  half and the result contains a cycle.
- Choosing the wrong middle for even lengths, so the halves differ by two.
- Saving `first.next` and `second.next` *before* relinking; otherwise the rest
  of each half is lost.
