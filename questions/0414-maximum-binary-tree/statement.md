You are given an array `nums` of **distinct** non-negative integers. Build a
binary tree from it with this rule:

- The root holds the largest value in the array.
- The root's left subtree is built, by the same rule, from the part of the
  array to the left of that largest value.
- The root's right subtree is built, by the same rule, from the part to its
  right.
- An empty part gives an empty subtree.

Because the values are distinct, the tree is unique. Return its root.

## Example 1

```
nums   = [4, 1, 7, 0, 5, 2]
output = [7, 4, 5, null, 1, 0, 2]
```

`7` is the maximum. `[4, 1]` builds the left subtree (`4` with right child
`1`) and `[0, 5, 2]` builds the right subtree (`5` with children `0` and `2`).

## Example 2

```
nums   = [1, 3, 2]
output = [3, 1, 2]
```

## Constraints

- `1 <= nums.length <= 1000`
- `0 <= nums[i] <= 1000`
- All values in `nums` are distinct.
- Follow-up: build the tree in O(n) time.
