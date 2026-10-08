# Approach: one pass, compare neighbours

Sorting guarantees that copies of a value form one contiguous run. Keep the
first node of each run and splice out any following node with the same value.

```python
def delete_duplicates(head):
    cur = head
    while cur and cur.next:
        if cur.next.val == cur.val:
            cur.next = cur.next.next
        else:
            cur = cur.next
    return head
```

The head itself is never removed (it is the first of its run), so returning
`head` is safe.

## Complexity

- Time: O(n).
- Space: O(1).

## Pitfalls

- Moving `cur` forward after a removal leaves the third copy of a value in a
  run of three or more.
- Forgetting the empty list: the `while cur and ...` guard handles it.
