Implement a tiny pattern matcher. A pattern `p` is made of lowercase letters
and two special symbols:

- `.` matches any **single** character.
- `*` means "zero or more copies of the element right before it" (a letter or
  a `.`). So `b*` matches `""`, `"b"`, `"bb"`, ... and `.*` matches any string.

Return `true` if the pattern matches the **entire** string `s`, not just a part
of it, and `false` otherwise.

## Example 1

```
s      = "abbbc"
p      = "ab*c"
output = true     # b* takes "bbb"
```

## Example 2

```
s      = "cat"
p      = "c.*a"
output = false    # .* can swallow "a", but then the pattern must end in "a"
```

## Constraints

- `0 <= len(s) <= 100`
- `0 <= len(p) <= 100`
- `s` contains only lowercase English letters.
- `p` contains only lowercase English letters, `.` and `*`.
- Every `*` in `p` comes right after a letter or a `.`.
