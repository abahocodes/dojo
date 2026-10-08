# Approach: slow and fast pointers

Move `slow` one node per step and `fast` two nodes per step. After `t` steps
`slow` sits at index `t` and `fast` at index `2t`. The loop stops when `fast`
is the last node (odd `n`, `2t = n - 1`) or has run past it (even `n`,
`2t = n`), so in both cases `slow` is at index `floor(n / 2)` - exactly the
required middle.

```python
def middle_node(head):
    slow = fast = head
    while fast and fast.next:
        slow = slow.next
        fast = fast.next.next
    return slow
```

## Complexity

- Time: O(n) - `fast` crosses the list once.
- Space: O(1).

## Pitfalls

- Testing only `fast.next` (and not `fast`) dereferences `None` on
  even-length lists.
- Using `while fast.next and fast.next.next` returns the *first* middle node
  for even lengths, which is not what is asked.
