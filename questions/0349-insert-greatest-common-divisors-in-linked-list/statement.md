You are given `head`, the first node of a non-empty singly linked list of
positive integers.

For every pair of neighbouring nodes, insert a new node between them whose
value is the **greatest common divisor** of the two neighbours' values. The
original nodes keep their order. Return the head of the resulting list.

A list with a single node has no neighbouring pairs and is returned unchanged.

## Example 1

```
head   = 12 -> 18 -> 7 -> 21
output = 12 -> 6 -> 18 -> 1 -> 7 -> 7 -> 21
```

`gcd(12, 18) = 6`, `gcd(18, 7) = 1`, `gcd(7, 21) = 7`.

## Example 2

```
head   = 9
output = 9
```

## Constraints

- The list has between `1` and `5000` nodes.
- `1 <= node.val <= 1000`
