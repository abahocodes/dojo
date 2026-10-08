# Approach: dummy node and pointer surgery

Put a dummy node in front of the list and keep `prev` at the node just before
the pair being swapped. Each swap rewires three pointers, after which the first
node of the pair (`a`) has become the second, so it is the `prev` for the next
pair.

```python
def swap_pairs(head):
    dummy = ListNode(0, head)
    prev = dummy
    while prev.next is not None and prev.next.next is not None:
        a = prev.next
        b = a.next
        a.next = b.next
        b.next = a
        prev.next = b
        prev = a
    return dummy.next
```

A recursive version — swap the first two, then attach the result of swapping
the rest — is shorter but uses O(n) stack.

## Complexity

- Time: O(n).
- Space: O(1) iterative; O(n) recursive.

## Pitfalls

- Rewiring in the wrong order and losing the rest of the list: set `a.next`
  from `b.next` before overwriting `b.next`.
- Forgetting to link the previous pair to the new first node (`prev.next = b`).
- Swapping values instead of nodes defeats the purpose of the exercise.
