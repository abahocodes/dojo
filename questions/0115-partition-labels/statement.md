Cut the lowercase string `s` into consecutive, non-empty pieces so that every
letter appears in **at most one** piece (all copies of a letter end up
together). Among all such cuttings, use one with as **many** pieces as
possible.

Return the lengths of the pieces, from left to right. (The cutting with the
most pieces is always unique.)

## Example 1

```
s      = "abcbadefegdhijhklk"
output = [5, 6, 4, 3]
# "abcba" | "defegd" | "hijh" | "klk"
```

## Example 2

```
s      = "caccb"
output = [4, 1]   # "cacc" | "b"
```

## Constraints

- `1 <= len(s) <= 10^4`
- `s` contains only lowercase English letters.
