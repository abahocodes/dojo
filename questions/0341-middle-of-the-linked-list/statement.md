You are given `head`, the first node of a singly linked list with at least
one node.

Find the list's **middle node** and return it; the returned node, together with
every node after it, is the answer. For a list of `n` nodes (indexed from `0`)
the middle node is the one at index `n / 2` rounded down. In particular, when
`n` is even there are two central nodes and you must return the **second** one.

## Example 1

```
head   = 8 -> 3 -> 5 -> 1 -> 9
output = 5 -> 1 -> 9
```

## Example 2

```
head   = 2 -> 4 -> 6 -> 8 -> 10 -> 12
output = 8 -> 10 -> 12    # 6 and 8 are both central; take the second
```

## Constraints

- The list has between `1` and `10^4` nodes.
- `-10^5 <= node.val <= 10^5`

**Follow-up:** can you do it in a single pass, without counting the nodes first?
