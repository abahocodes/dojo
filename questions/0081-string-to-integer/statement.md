Read a whole number from the start of the text `s`, following these rules in
order:

1. **Spaces:** skip any space characters (`" "`) at the very beginning. Only
   the space character is skipped.
2. **Sign:** if the next character is `"+"` or `"-"`, it gives the sign and is
   consumed. At most one sign character is allowed. Without one, the number is
   positive.
3. **Digits:** read the digits `0`–`9` that follow, stopping at the first
   character that is not a digit or at the end of the text. Everything after
   that point is ignored. Leading zeros are allowed.
4. **Nothing read:** if step 3 reads no digits at all, the result is `0`.
5. **Clamp:** if the signed value is below `-2^31`, return `-2^31`
   (`-2147483648`); if it is above `2^31 - 1`, return `2^31 - 1`
   (`2147483647`).

Return the resulting integer.

## Example 1

```
s      = "   -0042abc"
output = -42      # spaces skipped, "-" read, digits "0042", stop at "a"
```

## Example 2

```
s      = "9876543210"
output = 2147483647   # too large, clamped
```

## Example 3

```
s      = "words 7"
output = 0        # the first character is not a space, sign or digit
```

## Constraints

- `0 <= len(s) <= 200`
- `s` contains only English letters, digits, `" "`, `"+"`, `"-"` and `"."`.
