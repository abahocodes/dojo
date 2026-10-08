# Approach: one pass, looking one digit ahead

When a number is doubled, each digit `d` becomes `2d`, which is at most `18`,
so it contributes `(2d) % 10` to its own position and a carry of `1` to the
position on its left exactly when `d >= 5`. The carry into a position never
cascades further: `(2d) % 10` is even, so at most `8`, and adding `1` cannot
overflow.

That means each new digit depends only on the old digit and the old digit to
its right, and the list can be rewritten in place from the head:

```python
def double_it(head):
    if head.val >= 5:
        head = ListNode(0, head)
    node = head
    while node:
        node.val = (node.val * 2) % 10
        if node.next and node.next.val >= 5:
            node.val += 1
        node = node.next
    return head
```

A leading digit of `5` or more produces a carry out of the top, so a new head
(initially `0`, which then receives that carry) is added first.

## Complexity

- Time: O(n).
- Space: O(1) extra.

## Pitfalls

- Converting to an integer overflows (or, in Python, is slow for huge inputs).
- Reading `node.next.val` after it has already been doubled — check the next
  digit before overwriting it (the loop above reads it before it is updated).
- Forgetting the extra leading `1` when the first digit is `5` or more.
- Reversing the list twice works too, but is more code and more chances to
  break links.
