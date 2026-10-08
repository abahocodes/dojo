Cut the string `s` into one or more consecutive, non-empty pieces so that
every piece reads the same forwards and backwards (a palindrome). Return every
possible way to do this.

Each way is a list of pieces in the order they appear in `s`, so joining the
pieces gives back `s`. The ways themselves may be listed in any order, but
each must appear exactly once.

## Example 1

```
s      = "noon"
output = [["n", "o", "o", "n"], ["n", "oo", "n"], ["noon"]]
```

## Example 2

```
s      = "abcba"
output = [["a", "b", "c", "b", "a"], ["a", "bcb", "a"], ["abcba"]]
```

## Constraints

- `1 <= len(s) <= 12`
- `s` contains only lowercase English letters.
