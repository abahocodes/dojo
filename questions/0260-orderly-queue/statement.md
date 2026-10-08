You are given a string `s` of lowercase letters and an integer `k`. A **move**
picks any one of the first `k` characters of `s`, removes it from its place,
and appends it to the end of the string.

You may make as many moves as you like (including none). Return the
lexicographically smallest string you can end up with.

## Example 1

```
s      = "dcab"
k      = 1
output = "abdc"   # with k = 1 only rotations are reachable:
                  # dcab, cabd, abdc, bdca
```

## Example 2

```
s      = "zebra"
k      = 2
output = "aberz"
```

## Constraints

- `1 <= k <= len(s) <= 1000`
- `s` consists of lowercase English letters (`a` to `z`).
