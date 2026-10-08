# Approach: split into two lists, then join

Build two chains while scanning: one for nodes with value below `x` and one for
the rest. Appending each node to the tail of its chain preserves the original
relative order inside each group. Dummy heads avoid special cases when a chain
stays empty.

```python
def partition_list(head, x):
    small = ListNode(0)
    large = ListNode(0)
    s, l = small, large
    cur = head
    while cur is not None:
        if cur.val < x:
            s.next = cur
            s = cur
        else:
            l.next = cur
            l = cur
        cur = cur.next
    l.next = None
    s.next = large.next
    return small.next
```

## Complexity

- Time: O(n) — a single pass.
- Space: O(1) — nodes are relinked, not copied.

## Pitfalls

- Forgetting `l.next = None`: the last "large" node may still point to a small
  node, creating a cycle.
- Using `<=` instead of `<`: values equal to `x` belong to the second group.
- Swapping values in place (like an array partition) does not preserve the
  relative order.
