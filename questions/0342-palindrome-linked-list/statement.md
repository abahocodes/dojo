You are given `head`, the first node of a non-empty singly linked list whose
values are single digits.

Return `true` if the sequence of values is the same when read from the head to
the tail as when read from the tail to the head, and `false` otherwise.

## Example 1

```
head   = 3 -> 7 -> 0 -> 7 -> 3
output = true
```

## Example 2

```
head   = 4 -> 1 -> 1 -> 5
output = false   # read backwards it is 5 -> 1 -> 1 -> 4
```

## Constraints

- The list has between `1` and `10^5` nodes.
- `0 <= node.val <= 9`

**Follow-up:** an O(n)-time, O(1)-extra-space solution is expected: avoid
copying the values into an array or a stack.
