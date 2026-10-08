You are given an integer array `arr`, which may contain repeated values.

Cut `arr` into one or more **chunks**: non-empty pieces of consecutive
elements that together cover the whole array, in order. Then sort every chunk
on its own and glue the chunks back together in their original order. The cut
is **good** if the glued result equals `arr` sorted in non-decreasing order.

Return the largest number of chunks a good cut can have. (Cutting nothing,
i.e. a single chunk, is always good, so the answer is at least `1`.)

## Example 1

```
arr    = [2, 1, 3, 4, 4]
output = 4    # [2, 1] [3] [4] [4]
```

## Example 2

```
arr    = [5, 4, 3, 2, 1]
output = 1    # every element must move past the others
```

## Constraints

- `1 <= len(arr) <= 2000`
- `0 <= arr[i] <= 10^8`
