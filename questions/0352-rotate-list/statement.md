You are given `head`, the first node of a singly linked list (or `None` if the
list is empty), and a non-negative integer `k`.

Rotate the list to the right by `k` places. One rotation takes the last node
off the end and puts it at the front. Return the head of the rotated list.

`k` can be far larger than the length of the list.

## Example 1

```
head   = 10 -> 20 -> 30 -> 40 -> 50
k      = 2
output = 40 -> 50 -> 10 -> 20 -> 30
```

## Example 2

```
head   = 1 -> 2 -> 3
k      = 7
output = 3 -> 1 -> 2
```

Rotating a 3-node list 7 times is the same as rotating it `7 mod 3 = 1` time.

## Constraints

- The list has between `0` and `500` nodes.
- `-100 <= node.val <= 100`
- `0 <= k <= 2 * 10^9`
