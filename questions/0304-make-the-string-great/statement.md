You are given a string `s` of English letters. Call two neighbouring
characters a **clash** if they are the same letter in different cases, such
as `"aA"` or `"Bb"`. (`"aa"` and `"AB"` are not clashes.)

Repeatedly delete a clash (both characters) until the string has none, and
return the result. Deleting a clash can bring two new characters together,
and they may clash in turn. The final string is the same whatever order the
clashes are removed in. It may be empty.

## Example 1

```
s      = "cooOlDdog"
output = "colog"
# "oO" and "Dd" are clashes; removing both leaves "colog".
```

## Example 2

```
s      = "aBbA"
output = ""
# Removing "Bb" leaves "aA", which is also a clash.
```

## Constraints

- `1 <= len(s) <= 10^4`
- `s` consists of lowercase and uppercase English letters.
