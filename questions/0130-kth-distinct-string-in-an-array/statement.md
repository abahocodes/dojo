A string in the array `arr` is **distinct** if it occurs in `arr` exactly
once. Given `arr` and a positive integer `k`, return the `k`-th distinct
string, counting in order of appearance and starting from 1. If `arr` has
fewer than `k` distinct strings, return the empty string `""`.

## Example 1

```
arr    = ["x", "yy", "x", "z", "w", "yy", "v"]
k      = 2
output = "w"     # the distinct strings, in order, are "z", "w", "v"
```

## Example 2

```
arr    = ["ab", "ab", "cd"]
k      = 2
output = ""      # only "cd" is distinct
```

## Constraints

- `1 <= k <= len(arr) <= 1000`
- `1 <= len(arr[i]) <= 5`
- Strings consist of lowercase English letters.
