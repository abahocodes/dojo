The array `nums` is a permutation of `1, 2, ..., n`. Insert its values one by
one, from left to right, into an initially empty binary search tree (each new
value walks down from the root, going left when smaller and right when larger,
and becomes a new leaf).

Count how many **other** orderings of `nums` (permutations different from
`nums` itself) produce exactly the same tree when inserted the same way.
Return the count modulo `10^9 + 7`.

## Example 1

```
nums   = [2, 1, 3]
output = 1      # [2, 3, 1] builds the same tree: 2 at the root, 1 left, 3 right
```

## Example 2

```
nums   = [3, 4, 5, 1, 2]
output = 5
```

The tree is `3` with left chain `1 -> 2` and right chain `4 -> 5`. Any ordering
that starts with `3`, keeps `1` before `2`, and keeps `4` before `5` works:
there are 6 such orderings, and one of them is `nums` itself.

## Constraints

- `1 <= nums.length <= 1000`
- `nums` is a permutation of `1..nums.length`.
- Return the answer modulo `10^9 + 7`.
