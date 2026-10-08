You are given `head`, the first node of a singly linked list with an **even**
number of nodes `n`.

Number the nodes `0` to `n - 1` from the head. Node `i` and node `n - 1 - i` are
called **twins** (so the first node pairs with the last, the second with the
second-to-last, and so on). The **twin sum** of a pair is the sum of the two
values.

Return the largest twin sum in the list.

## Example 1

```
head   = 3 -> 8 -> 2 -> 6
output = 10    # pairs: 3 + 6 = 9 and 8 + 2 = 10
```

## Example 2

```
head   = 5 -> 1
output = 6
```

## Constraints

- The list has between `2` and `10^5` nodes, and the count is even.
- `1 <= node.val <= 10^5`
