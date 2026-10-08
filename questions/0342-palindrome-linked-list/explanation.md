# Approach: reverse the second half in place

1. Use slow/fast pointers to find the middle: when `fast` runs off the end,
   `slow` is at index `floor(n / 2)`.
2. Reverse the sublist starting at `slow`. Its new head is the old tail.
3. Walk from `head` and from the reversed head together, comparing values,
   until the reversed half runs out. Any mismatch means "not a palindrome".
4. (Optional but polite) reverse the second half again to restore the input.

For odd `n` the middle node ends up at the end of both walks; it is compared
with itself, which is harmless.

```python
def is_palindrome_list(head):
    slow = fast = head
    while fast and fast.next:
        slow = slow.next
        fast = fast.next.next

    def reverse(node):
        prev = None
        while node:
            node.next, prev, node = prev, node, node.next
        return prev

    tail = reverse(slow)
    ok = True
    a, b = head, tail
    while b:
        if a.val != b.val:
            ok = False
            break
        a, b = a.next, b.next
    reverse(tail)  # restore the original list
    return ok
```

## Complexity

- Time: O(n) - a constant number of passes.
- Space: O(1) extra.

## Pitfalls

- Looping on the first half's pointer instead of the reversed half: the first
  half still links into the middle node, so stop when the *reversed* half ends.
- Python's tuple assignment `node.next, prev, node = prev, node, node.next`
  works because the right side is evaluated first; when writing it as separate
  statements, save `node.next` before overwriting it.
- A recursive comparison is elegant but uses O(n) stack and can overflow on
  `10^5` nodes.
