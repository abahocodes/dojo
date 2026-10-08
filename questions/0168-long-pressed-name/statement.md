A friend tried to type their `name` on a sticky keyboard. Every character of
`name` was typed in order, but some keys may have stuck, so a character could
come out **one or more extra times** in a row. No character was skipped and
no other character was added.

Given what was actually recorded, `typed`, return `true` if it could have
come from typing `name` this way, otherwise `false`.

## Example 1

```
name   = "kate"
typed  = "kkaatte"
output = true
```

## Example 2

```
name   = "moon"
typed  = "mon"
output = false   # the second 'o' of "moon" is missing
```

## Constraints

- `1 <= len(name), len(typed) <= 1000`
- Both strings consist of lowercase English letters.
