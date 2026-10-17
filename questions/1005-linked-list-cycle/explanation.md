# Approach: Floyd's tortoise and hare

Walk two pointers from `head`: `slow` advances one node per step, `fast` two.

- With no cycle, `fast` reaches the end of the list (`fast` or `fast.next` is
  `None`) and we return `False`.
- With a cycle, both pointers eventually enter it. From then on, each step
  brings `fast` one node closer to `slow` (it gains one per step), so the gap
  shrinks to zero and they land on the **same node**: return `True`.

```python
def has_cycle(head):
    slow = fast = head
    while fast is not None and fast.next is not None:
        slow = slow.next
        fast = fast.next.next
        if slow is fast:
            return True
    return False
```

A simpler alternative stores every visited node in a hash set and reports a
cycle when a node repeats. It is just as fast, but uses O(n) memory.

## Complexity

- Time: O(n). Before `slow` enters the cycle it takes at most `n` steps; once
  both are inside, `fast` catches up within one lap.
- Space: O(1) for Floyd; O(n) for the hash-set version.

## Pitfalls

- Comparing values (`slow.val == fast.val`) instead of node identity gives
  false positives whenever values repeat, for example a long list of zeros.
- Checking `fast.next.next` without first checking `fast.next` crashes at
  the end of an odd- or even-length list; test both `fast` and `fast.next`.
- Comparing the pointers before moving them reports a cycle immediately,
  since both start at `head`.
- Don't forget the empty list and a single node pointing at itself.
