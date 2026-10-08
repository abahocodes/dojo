# Approach: one pass, skip by relinking

Keep a pointer `cur` at the first node of the current "keep" block. Walk it
`m - 1` steps to the last kept node. Then probe forward `n` more nodes with a
second pointer and connect the last kept node directly to the node after the
deleted block. Continue from there.

```python
def delete_nodes(head, m, n):
    cur = head
    while cur is not None:
        for _ in range(m - 1):
            if cur.next is None:
                return head
            cur = cur.next
        skip = cur.next
        for _ in range(n):
            if skip is None:
                break
            skip = skip.next
        cur.next = skip
        cur = skip
    return head
```

## Complexity

- Time: O(L) for a list of length L — each node is visited at most once.
- Space: O(1).

## Pitfalls

- Walking `m` steps instead of `m - 1`: you need to stand *on* the last kept
  node to relink it.
- Not checking for the end of the list inside both inner loops — the final
  block of either kind may be short.
- Since `head` is always kept (`m >= 1`), the head never changes; no dummy node
  is needed.
