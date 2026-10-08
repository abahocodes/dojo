# Approach: insert while walking

Walk the original nodes. At each node that has a successor, build the new GCD
node, splice it in between the two, and then advance past it to the original
successor so the inserted node is never treated as an original one.

```python
def insert_gcds(head):
    def gcd(a, b):
        while b:
            a, b = b, a % b
        return a

    cur = head
    while cur.next is not None:
        nxt = cur.next
        cur.next = ListNode(gcd(cur.val, nxt.val), nxt)
        cur = nxt
    return head
```

## Complexity

- Time: O(n log V) — one pass, and each Euclid run takes O(log V) steps for
  values up to V.
- Space: O(1) besides the `n - 1` nodes the output requires.

## Pitfalls

- Advancing `cur` only one step after inserting: you'd then compute the GCD of
  an inserted node and its neighbour, and the loop would never end.
- Losing the original `next` pointer before linking the new node in.
