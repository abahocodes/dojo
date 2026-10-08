You are given the `root` of a binary search tree that **may contain
duplicate values**. Here the ordering is non-strict: every value in a node's
left subtree is less than or equal to the node's value, and every value in its
right subtree is greater than or equal to it.

Return every value that occurs the largest number of times (the **modes**), in
ascending order. If several values tie for the highest count, return all of
them.

## Example 1

```
root   = [1, null, 2, 2]
output = [2]          # 2 occurs twice, 1 once
```

## Example 2

```
root   = [3, 1, 5, 1, 3, 4, 5]
output = [1, 3, 5]    # 1, 3 and 5 each occur twice
```

## Constraints

- The tree has between `1` and `10^4` nodes.
- `-10^5 <= node.val <= 10^5`

**Follow-up:** can you do it without a hash map, using only `O(1)` extra space
besides the traversal stack and the output?
