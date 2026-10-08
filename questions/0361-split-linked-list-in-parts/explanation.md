# Approach: count, then cut

Count the nodes, then compute the size of every part: `n // k` each, with the
first `n % k` parts taking one extra node. A second walk cuts the list at the
right places — no copying needed.

```python
def split_list_to_parts(head, k):
    n = 0
    cur = head
    while cur:
        n += 1
        cur = cur.next
    base, extra = divmod(n, k)
    parts = []
    cur = head
    for i in range(k):
        size = base + (1 if i < extra else 0)
        part_head = cur
        for _ in range(size - 1):
            cur = cur.next
        if size > 0:
            nxt = cur.next
            cur.next = None
            cur = nxt
        parts.append(part_head)
    return parts
```

When a part has size 0, every node has already been used, so `cur` is `None`
and the part is empty.

## Complexity

- Time: O(n + k).
- Space: O(k) for the returned array of heads.

## Pitfalls

- Giving the extra nodes to the *last* parts instead of the first ones.
- Forgetting to cut each part, so the first part still reaches the end of the
  original list.
- Returning fewer than `k` parts when `n < k`.
