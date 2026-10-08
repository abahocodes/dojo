You are given a string `s` of lowercase letters. Rearrange its letters (use
every letter exactly as many times as it appears in `s`) so that no two
neighbouring characters are the same.

Many rearrangements may work. Return the one that comes first in
lexicographic (dictionary) order. If no valid rearrangement exists, return the
empty string `""`.

## Example 1

```
s      = "baaca"
output = "abaca"
```

`a` appears three times, so it must sit at positions 0, 2 and 4. Filling the
gaps with `b` then `c` gives the smallest arrangement.

## Example 2

```
s      = "xxxy"
output = ""
```

Three `x`s need at least two separators between them, but there is only one
other letter.

## Constraints

- `1 <= len(s) <= 500`
- `s` consists of lowercase English letters.
