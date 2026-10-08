# Approach: two pointers with a dummy head

Both lists are sorted, so the next node of the result is always the smaller of
the two current heads. Splice that node onto the tail of the result and advance
in its list. Once either list is exhausted, the other list's remaining nodes are
already in order and can be attached all at once.

```python
def merge_two_lists(list1, list2):
    dummy = ListNode()
    tail = dummy
    while list1 and list2:
        if list1.val <= list2.val:
            tail.next = list1
            list1 = list1.next
        else:
            tail.next = list2
            list2 = list2.next
        tail = tail.next
    tail.next = list1 if list1 else list2
    return dummy.next
```

A recursive version (pick the smaller head, set its `next` to the merge of the
rest) is shorter but uses one stack frame per node, which is risky for lists
with thousands of nodes.

## Complexity

- Time: O(n + m): every node is attached exactly once.
- Space: O(1): nodes are re-linked, not copied.

## Pitfalls

- Forgetting to attach the leftover list after the loop, which drops nodes.
- Not handling an empty input list: the dummy head makes this automatic.
- Returning `dummy` instead of `dummy.next`.
