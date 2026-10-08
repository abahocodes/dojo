Given a string `s` and an integer `k`, return the length of the longest
substring (contiguous block of characters) of `s` that uses **at most** `k`
different characters. If `k` is `0`, the answer is `0`.

Characters are compared exactly: uppercase and lowercase letters are
different, and the space is a character like any other.

## Example 1

```
s      = "abaccc"
k      = 2
output = 4    # "accc" uses only 'a' and 'c'
```

## Example 2

```
s      = "zz yy"
k      = 2
output = 3    # "zz " or " yy"; the whole string has 3 different characters
```

## Constraints

- `1 <= len(s) <= 5 * 10^4`
- `0 <= k <= 50`
- `s` consists of printable ASCII characters (codes 32 to 126).
