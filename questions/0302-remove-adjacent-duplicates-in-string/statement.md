You are given a string `s` of lowercase letters. A **removal** deletes two
neighbouring characters that are equal. Removing a pair can bring two new
characters next to each other, and they may form a new pair.

Keep making removals until no two neighbouring characters are equal, and
return the resulting string. The result is the same whatever order the
removals are made in. It may be the empty string.

## Example 1

```
s      = "bookkeeper"
output = "bper"
# "bookkeeper" -> "bkkeeper" -> "beeper" -> "bper"
```

## Example 2

```
s      = "xyyxz"
output = "z"
# Removing "yy" makes the two x's neighbours, and they are removed too.
```

## Constraints

- `1 <= len(s) <= 10^5`
- `s` consists of lowercase English letters.
