You are given two strings `s` and `t` of the same length. In one **step**
you choose one position of `t` and overwrite the character there with any
lowercase letter.

Return the fewest steps needed so that `t` becomes an **anagram** of `s`,
that is, `t` contains exactly the same letters as `s` with the same
multiplicities, in any order. `s` is never changed.

## Example 1

```
s      = "listen"
t      = "lisper"
output = 2       # change 'p' -> 't' and 'r' -> 'n'
```

## Example 2

```
s      = "dusty"
t      = "study"
output = 0       # already an anagram
```

## Constraints

- `1 <= len(s) == len(t) <= 5 * 10^4`
- `s` and `t` consist of lowercase English letters.
