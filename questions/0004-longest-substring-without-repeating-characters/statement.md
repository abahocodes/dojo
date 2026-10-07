Given a string `s`, find the length of the longest **contiguous** stretch of
`s` in which no character appears more than once.

A stretch must be consecutive characters of `s`; picking characters with gaps
between them does not count.

## Example 1

```
s      = "dvdfz"
output = 4         # "vdfz"
```

## Example 2

```
s      = "zzzz"
output = 1         # any single "z"
```

## Constraints

- `0 <= len(s) <= 5 * 10^4`
- `s` consists of printable ASCII characters (letters, digits, symbols, spaces).
