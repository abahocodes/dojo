You are given the `root` of a binary tree. The **subtree** of a node is that
node together with everything below it. Two subtrees are **duplicates** when
they have exactly the same shape and the same value in every matching
position.

Find every kind of subtree that occurs at least twice in the tree. For each
kind, return the root node of **one** of its copies (it doesn't matter which,
since the copies are identical). List each kind once. The kinds may come in
**any order**; return an empty list if there are no duplicates.

## Example 1

```
root   = [7, 3, 3, 5, null, 5]
output = [[3, 5], [5]]     # the subtree 3 -> 5 appears twice, and so does the leaf 5
```

## Example 2

```
root   = [2, 1, 1]
output = [[1]]
```

## Constraints

- The tree has between `1` and `5000` nodes.
- `-200 <= node.val <= 200`
- The tree is given in level order; `null` marks a missing child. Each returned
  subtree is printed the same way.
