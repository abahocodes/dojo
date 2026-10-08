# Approach: close the ring, then cut it

Measure the length `n` and remember the tail. The effective rotation is
`r = k mod n`. Joining the tail to the head turns the list into a ring; the new
tail is the node at index `n - r - 1`, and the node after it is the new head.
Breaking the ring there produces the answer.

```python
def rotate_right(head, k):
    if head is None:
        return None
    n = 1
    tail = head
    while tail.next is not None:
        tail = tail.next
        n += 1
    r = k % n
    if r == 0:
        return head
    tail.next = head
    new_tail = head
    for _ in range(n - r - 1):
        new_tail = new_tail.next
    new_head = new_tail.next
    new_tail.next = None
    return new_head
```

## Complexity

- Time: O(n) — at most two passes, independent of `k`.
- Space: O(1).

## Pitfalls

- Simulating `k` single rotations: with `k` up to 2·10^9 this is far too slow.
- Dividing by zero (`k % 0`) on an empty list — return early.
- Forgetting to cut the ring, which leaves a cycle and hangs whatever reads the
  result.
