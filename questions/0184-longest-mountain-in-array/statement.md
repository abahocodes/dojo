Call a contiguous subarray of `arr` a **mountain** if it has length at least 3
and there is a peak position inside it (not at either end) such that the
values strictly increase from the first element up to the peak and then
strictly decrease from the peak to the last element.

Return the length of the longest mountain in `arr`, or `0` if there is none.
Plateaus (equal neighbours) are never part of a slope.

## Example 1

```
arr    = [1, 4, 6, 3, 2, 5, 1]
output = 5    # [1, 4, 6, 3, 2]
```

## Example 2

```
arr    = [5, 5, 5]
output = 0    # no strict rise or fall at all
```

## Constraints

- `1 <= len(arr) <= 10^4`
- `0 <= arr[i] <= 10^4`
