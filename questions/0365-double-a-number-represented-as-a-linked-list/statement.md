A non-empty singly linked list stores a non-negative integer, one decimal digit
per node, with the **most significant** digit at the head. The number has no
leading zeros, except for the number `0` itself, which is a single node `0`.

Return the head of a list that represents **twice** that number, in the same
format (most significant digit first, no leading zeros).

## Example 1

```
head   = 4 -> 7 -> 2
output = 9 -> 4 -> 4
```

`472 * 2 = 944`.

## Example 2

```
head   = 8 -> 0 -> 5
output = 1 -> 6 -> 1 -> 0
```

`805 * 2 = 1610`: the answer has one more digit than the input.

## Constraints

- The list has between `1` and `10^4` nodes.
- `0 <= node.val <= 9`
- The number has no leading zeros, except the number `0` itself.
