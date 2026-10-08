You are given `head`, the first node of a singly linked list with `n` nodes whose
values are distinct integers in the range `[0, n - 1]`, and an array `nums`
containing some of those values (each at most once).

A **component** is a maximal run of consecutive nodes in the list whose values
all appear in `nums` — it cannot be extended by the node just before it or the
node just after it. Return the number of components.

## Example 1

```
head   = 0 -> 1 -> 2 -> 3
nums   = [0, 1, 3]
output = 2
```

The components are `0 -> 1` and `3`.

## Example 2

```
head   = 3 -> 0 -> 4 -> 2 -> 1
nums   = [4, 3, 1, 2]
output = 2
```

The components are `3` and `4 -> 2 -> 1`; the node `0` separates them.

## Constraints

- `1 <= n <= 10^4`
- The node values are a permutation of `0, 1, ..., n - 1`.
- `1 <= nums.length <= n`
- The values in `nums` are distinct and each is a node value.
