You are given two strings, a text `s` and a pattern `t`. A **window** of `s`
is a contiguous substring. A window *covers* `t` when every character of `t`
appears in it at least as many times as it appears in `t` (so duplicates in `t`
must be matched by duplicates in the window; extra characters are allowed).

Return the shortest window of `s` that covers `t`. If several covering windows
share that minimum length, return the one that starts furthest to the left. If
no window covers `t`, return the empty string `""`.

## Example 1

```
s      = "XAYBZCAB"
t      = "ABC"
output = "CAB"      # "AYBZC" (length 5) also covers, but "CAB" is shorter
```

## Example 2

```
s      = "aab"
t      = "aaa"
output = ""         # s has only two 'a's
```

## Constraints

- `1 <= len(s), len(t) <= 10^5`
- `s` and `t` contain only English letters (upper and lower case are different).

**Follow-up:** can you solve it in O(len(s) + len(t)) time?
