You are given the `root` of a binary search tree, a real number `target` and an
integer `k`. Return the `k` values in the tree that are closest to `target`
(smallest absolute difference `|value - target|`).

The values may be returned in **any order**. The inputs are chosen so that the
answer set is unique: there is never a tie between the k-th closest value and
the next one.

If the tree is balanced, try to beat O(n) time.

## Example 1

```
root   = [4, 2, 5, 1, 3]
target = 3.714286
k      = 2
output = [3, 4]     # distances 0.71 and 0.29; the next closest, 5, is 1.29 away
```

## Example 2

```
root   = [1]
target = 0.25
k      = 1
output = [1]
```

## Constraints

- The tree has `n` nodes, `1 <= k <= n <= 10^4`.
- `0 <= node.val <= 10^9`, and all values are distinct.
- `-10^9 <= target <= 2 * 10^9`
- The k closest values are uniquely determined.
- The input is a valid BST, given in level order with `null` for a missing
  child.
