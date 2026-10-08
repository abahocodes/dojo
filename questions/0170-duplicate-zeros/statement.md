You are given an integer array `arr`. Build a new array by reading `arr` from
left to right and writing every element once, except that every `0` is
written **twice**. Then cut the result down to the original length of `arr`
and return it.

(Equivalently: each zero is duplicated in place, pushing the later elements to
the right, and anything pushed past the end of the array falls off.)

## Example 1

```
arr    = [4, 0, 2, 7, 0, 5, 1]
output = [4, 0, 0, 2, 7, 0, 0]
```

## Example 2

```
arr    = [3, 1, 2]
output = [3, 1, 2]   # no zeros, nothing moves
```

## Constraints

- `1 <= len(arr) <= 10^4`
- `0 <= arr[i] <= 9`
