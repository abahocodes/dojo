# Approach: grade-school addition with a carry

The lists already store digits in the order you add them by hand: ones first,
then tens, and so on. Walk both lists at once, add the two digits plus the
carry, emit the last digit of that sum and carry the rest. Keep going while
either list has digits left or a carry remains.

```python
def add_two_numbers(l1, l2):
    dummy = ListNode()
    tail = dummy
    carry = 0
    while l1 or l2 or carry:
        total = carry
        if l1:
            total += l1.val
            l1 = l1.next
        if l2:
            total += l2.val
            l2 = l2.next
        carry, digit = divmod(total, 10)
        tail.next = ListNode(digit)
        tail = tail.next
    return dummy.next
```

## Complexity

- Time: O(max(m, n)): one step per digit of the longer number.
- Space: O(max(m, n)) for the result list; O(1) extra besides the output.

## Pitfalls

- Forgetting the final carry: `5 + 5` must give `0 -> 1`, not `0`.
- Stopping when the shorter list ends instead of continuing with the longer one.
- Converting to built-in integers: fixed-width integers overflow, JavaScript
  numbers lose precision past 2^53, and Python limits int/str conversions of
  very long numbers.
