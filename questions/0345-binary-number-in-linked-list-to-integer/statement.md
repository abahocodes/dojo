You are given `head`, the first node of a non-empty singly linked list in
which every node holds a single bit, `0` or `1`.

Reading the bits from the head to the tail spells a binary number, with the
head holding the **most significant** bit. Return that number's value in
decimal.

## Example 1

```
head   = 1 -> 1 -> 0 -> 1
output = 13    # 1101 in binary
```

## Example 2

```
head   = 0 -> 0 -> 1 -> 0
output = 2     # leading zeros do not change the value
```

## Constraints

- The list has between `1` and `30` nodes.
- Every `node.val` is `0` or `1`.
