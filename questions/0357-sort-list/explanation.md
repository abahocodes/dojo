# Approach: merge sort on the list

Merge sort fits linked lists perfectly: splitting at the middle costs one pass
with slow/fast pointers, and merging two sorted lists only rewires `next`
pointers, so no auxiliary arrays are needed.

```python
def sort_list(head):
    if head is None or head.next is None:
        return head
    slow, fast = head, head.next
    while fast and fast.next:
        slow = slow.next
        fast = fast.next.next
    mid = slow.next
    slow.next = None
    left = sort_list(head)
    right = sort_list(mid)
    dummy = tail = ListNode(0)
    while left and right:
        if left.val <= right.val:
            tail.next, left = left, left.next
        else:
            tail.next, right = right, right.next
        tail = tail.next
    tail.next = left if left else right
    return dummy.next
```

The recursion is only `O(log n)` deep. A bottom-up version (merge runs of
length 1, 2, 4, ... in place) removes even that stack and uses `O(1)` extra
space.

## Complexity

- Time: O(n log n) — `log n` levels, each doing O(n) splitting and merging.
- Space: O(log n) for the recursion stack (O(1) with the bottom-up variant).

## Pitfalls

- Starting `fast` at `head` makes a two-node list split into two and zero
  nodes, so the recursion never shrinks.
- Forgetting to cut the list (`slow.next = None`) before recursing.
- Losing the leftover tail of one list at the end of the merge.
