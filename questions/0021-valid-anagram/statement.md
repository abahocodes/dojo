You are given two strings `s` and `t` made of lowercase English letters. `t` is
an **anagram** of `s` if you can produce `t` by rearranging the letters of `s`,
using each letter exactly as many times as it occurs in `s`.

Return `true` if `t` is an anagram of `s`, and `false` otherwise.

## Example 1

```
s      = "listen"
t      = "silent"
output = true        # same letters, different order
```

## Example 2

```
s      = "rotor"
t      = "rotar"
output = false       # s has two o's, t has only one
```

## Constraints

- `1 <= len(s), len(t) <= 5 * 10^4`
- `s` and `t` contain only lowercase English letters `a`-`z`.

**Follow-up:** sorting both strings is O(n log n). Can you do it in linear time?
