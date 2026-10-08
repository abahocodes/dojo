You are given two strings `s` and `p`, where `p` is a subsequence of `s`, and
an array `removable` of **distinct** indices into `s`.

For a number `k`, delete from `s` the characters at the positions
`removable[0], removable[1], ..., removable[k - 1]` (positions always refer to
the original `s`). Return the **largest** `k` (with
`0 <= k <= len(removable)`) for which `p` is still a subsequence of what is
left of `s`.

A string `p` is a subsequence of `t` if `p` can be obtained from `t` by
deleting zero or more characters without changing the order of the rest.

## Example 1

```
s         = "xyzxyz"
p         = "xz"
removable = [0, 2, 3, 5, 1]
output    = 2    # after removing 0 and 2: "_y_xyz" still has x(3) then z(5);
                 # removing 3 as well leaves no 'x'
```

## Example 2

```
s         = "ab"
p         = "ab"
removable = [0]
output    = 0    # removing anything destroys "ab"
```

## Constraints

- `1 <= len(p) <= len(s) <= 10^5`
- `0 <= len(removable) < len(s)`
- `0 <= removable[i] < len(s)`, all distinct
- `s` and `p` consist of lowercase English letters
- `p` is a subsequence of `s`
