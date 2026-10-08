You are given the `head` of a singly linked list whose values are sorted in
non-decreasing order. Build a height-balanced binary search tree containing
exactly those values and return its root.

So that the answer is unique, build it like this: number the list positions
`0 .. n-1`. For a range of positions `lo .. hi`, the root is the element at
position `(lo + hi + 1) // 2` (when the range has an even length, this is the
**right** one of the two middle elements); its left subtree is built the same
way from `lo .. mid-1`, and its right subtree from `mid+1 .. hi`. An empty range
gives an empty tree.

## Example 1

```
head   = [-7, -2, 0, 4, 9]
output = [0, -2, 9, -7, null, 4]
```

```
          0
        /   \
      -2     9
      /     /
    -7     4
```

## Example 2

```
head   = [3, 8]
output = [8, 3]
```

For two elements the right middle, `8`, is the root.

## Constraints

- The list has between `0` and `2 * 10^4` nodes.
- `-10^5 <= node.val <= 10^5`
- Values are sorted in non-decreasing order (duplicates are allowed).
- The list is given as an array of values; the tree is returned in level order
  with `null` marking a missing child.
