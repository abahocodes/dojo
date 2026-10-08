# Approach: Horner's rule

Reading the bits from most to least significant, every new bit doubles the
value accumulated so far and adds itself - the same way you read decimal
digits left to right, but in base 2.

```python
def get_decimal_value(head):
    value = 0
    while head:
        value = value * 2 + head.val
        head = head.next
    return value
```

With at most 30 bits the result is below `2^30`, so it fits in a 32-bit
integer.

## Complexity

- Time: O(n).
- Space: O(1).

## Pitfalls

- Treating the head as the least significant bit reverses the number.
- Building a string and parsing it works, but is unnecessary work; the
  arithmetic version is just as short.
