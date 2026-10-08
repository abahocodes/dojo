# Approach: weave two chains in place

Maintain the tail of the odd chain (`odd`) and the tail of the even chain
(`even`). Because positions alternate, the node after `even` is the next odd
node and the node after that is the next even node, so each step extends both
chains by one. When the even chain can't grow any more, attach it after the
odd chain.

```python
def odd_even_list(head):
    if head is None:
        return None
    odd = head
    even = head.next
    even_head = even
    while even is not None and even.next is not None:
        odd.next = even.next
        odd = odd.next
        even.next = odd.next
        even = even.next
    odd.next = even_head
    return head
```

## Complexity

- Time: O(n).
- Space: O(1) — only a few pointers.

## Pitfalls

- Grouping by node *value* parity instead of position.
- Losing the start of the even chain — save `even_head` before the loop.
- The loop condition must check both `even` and `even.next`, otherwise it
  dereferences `None` at the end of odd- or even-length lists.
