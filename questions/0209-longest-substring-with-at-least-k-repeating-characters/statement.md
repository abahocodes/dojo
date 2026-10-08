You are given a string `s` of lowercase English letters and an integer `k`.
A substring (a contiguous, non-empty piece of `s`) is **balanced** if every
letter that occurs in it occurs there at least `k` times.

Return the length of the longest balanced substring of `s`, or `0` if `s` has
no balanced substring.

## Example 1

```
s      = "abbaacbb"
k      = 2
output = 5    # "abbaa": 'a' three times, 'b' twice
```

## Example 2

```
s      = "xyz"
k      = 2
output = 0    # every letter occurs only once
```

## Constraints

- `1 <= len(s) <= 10^4`
- `s` consists of lowercase English letters
- `1 <= k <= 10^5`
