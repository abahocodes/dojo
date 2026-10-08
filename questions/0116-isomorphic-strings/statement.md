You are given two strings `s` and `t` of the same length. Decide whether `t`
can be produced from `s` by a consistent **character substitution**:

- every occurrence of a given character in `s` is replaced by the same
  character, and
- two different characters of `s` are never replaced by the same character.

A character may be replaced by itself. Return `true` if such a substitution
exists, otherwise `false`.

## Example 1

```
s      = "kayak"
t      = "radar"
output = true    # k -> r, a -> a, y -> d
```

## Example 2

```
s      = "moon"
t      = "noon"
output = false   # both 'm' and 'n' would have to become 'n'
```

## Constraints

- `1 <= len(s) == len(t) <= 5 * 10^4`
- `s` and `t` consist of printable ASCII characters (codes 32 to 126),
  including spaces.
