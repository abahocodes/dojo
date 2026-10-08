You are given a string `s`. Rearrange its characters so that:

- all copies of the same character are next to each other,
- characters that occur more often come first, and
- among characters with the same number of occurrences, the one with the
  smaller character code comes first (so digits before uppercase letters,
  and uppercase before lowercase).

Return the rearranged string. Upper- and lowercase versions of a letter are
different characters.

## Example 1

```
s      = "banana"
output = "aaannb"     # a x3, n x2, b x1
```

## Example 2

```
s      = "zzY99Y"
output = "99YYzz"     # every character appears twice: '9' < 'Y' < 'z'
```

## Constraints

- `1 <= len(s) <= 5 * 10^5`
- `s` consists of English letters (upper- and lowercase) and digits.
