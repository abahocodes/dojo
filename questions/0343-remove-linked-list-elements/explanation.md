# Approach: dummy head and a trailing pointer

A sentinel ("dummy") node placed before `head` turns the special case of
removing the first node(s) into the ordinary case: every real node now has a
predecessor.

Walk with `prev`, the last node known to stay. Look at `prev.next`: if it holds
`val`, splice it out and look at the new `prev.next` again; otherwise it stays,
so advance `prev`.

```python
def remove_elements(head, val):
    dummy = ListNode(0, head)
    prev = dummy
    while prev.next:
        if prev.next.val == val:
            prev.next = prev.next.next
        else:
            prev = prev.next
    return dummy.next
```

A recursive version (`head.next = remove_elements(head.next, val)`) is short,
but uses one stack frame per node.

## Complexity

- Time: O(n) - every node is examined once.
- Space: O(1).

## Pitfalls

- Advancing `prev` right after a removal skips the next node, so two adjacent
  matches leave one behind.
- Returning `head` instead of `dummy.next` returns a removed node when the list
  started with `val`.
- The result may be empty; make sure that case returns `None`/`null`.
