# Approach: prefix sums and a last-occurrence map

If the prefix sum (from a sentinel `0` in front of the head) is equal at nodes
`p` and `q`, every node after `p` up to and including `q` sums to zero, so the
whole stretch can be cut. The rule in the statement says: from each kept node,
jump to the **last** node with the same prefix sum.

Two passes do exactly that:

1. Walk the list with a running total and store `last[total] = node`. Later
   nodes overwrite earlier ones, so the map holds the last occurrence.
2. Walk again from the sentinel with a fresh total. At each node set
   `node.next = last[total].next`, then advance. If the node is itself the
   last occurrence the link is unchanged.

```python
def remove_zero_sum_sublists(head):
    dummy = ListNode(0, head)
    last = {}
    total = 0
    node = dummy
    while node:
        total += node.val
        last[total] = node
        node = node.next
    total = 0
    node = dummy
    while node:
        total += node.val
        node.next = last[total].next
        node = node.next
    return dummy.next
```

The kept nodes have pairwise different prefix sums, so no zero-sum run
remains.

## Complexity

- Time: O(n) — two passes with O(1) map operations.
- Space: O(n) for the prefix-sum map.

## Pitfalls

- Forgetting the sentinel: a zero-sum run that starts at the head (or the whole
  list summing to `0`) would be missed.
- Storing the *first* occurrence instead of the last only removes part of a
  removable stretch.
- Returning `head` instead of `dummy.next`: the original head may have been cut.
