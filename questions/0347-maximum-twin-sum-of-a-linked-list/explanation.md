# Approach: reverse the second half

With `n` even, slow/fast pointers leave `slow` at index `n / 2`, the start of
the second half. After reversing the second half, its head is node `n - 1`,
then `n - 2`, and so on - exactly the twins of nodes `0, 1, ...` in order. A
lockstep walk over both halves visits every twin pair once.

```python
def pair_sum(head):
    slow = fast = head
    while fast and fast.next:
        slow = slow.next
        fast = fast.next.next
    prev = None
    while slow:
        nxt = slow.next
        slow.next = prev
        prev = slow
        slow = nxt
    best = 0
    a, b = head, prev
    while b:
        best = max(best, a.val + b.val)
        a, b = a.next, b.next
    return best
```

A simpler O(n)-space alternative pushes the first half onto a stack and pops
while walking the second half.

## Complexity

- Time: O(n).
- Space: O(1) extra.

## Pitfalls

- Stopping the comparison on the first-half pointer instead of the reversed
  half: the first half still links into the (now last) middle node.
- Twin sums reach `2 * 10^5`, comfortably within a 32-bit integer.
