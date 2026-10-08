# Approach: stacks and front insertion

Digits must be added from the least significant end, which is the tail of
each list. Pushing every digit onto a stack reverses the order for free. Then
pop digit pairs, add with carry, and prepend each new digit to the answer so
the result comes out most-significant-first without a final reversal.

```python
def add_two_numbers_ii(l1, l2):
    a, b = [], []
    while l1:
        a.append(l1.val)
        l1 = l1.next
    while l2:
        b.append(l2.val)
        l2 = l2.next
    head = None
    carry = 0
    while a or b or carry:
        s = carry + (a.pop() if a else 0) + (b.pop() if b else 0)
        head = ListNode(s % 10, head)
        carry = s // 10
    return head
```

Reversing both input lists in place, adding as in the classic problem, and
reversing the result also works in O(1) extra space, at the cost of modifying
the inputs.

## Complexity

- Time: O(m + n).
- Space: O(m + n) for the stacks (the output list aside).

## Pitfalls

- Converting to an integer overflows: inputs can have 100 digits.
- Dropping the final carry (`99 + 1` must give `100`).
- Appending digits at the tail produces the sum backwards.
