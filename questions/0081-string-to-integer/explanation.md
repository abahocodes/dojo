# Approach: one forward scan

Walk through the string with one index, applying the rules in order. Skip
spaces; read one optional sign; then accumulate digits. Because every digit
can only increase the magnitude, the moment the magnitude passes `2^31 − 1`
the answer is decided: `2^31 − 1` for a positive number, `−2^31` for a
negative one (whose magnitude is at least `2^31`).

```python
def string_to_integer(s):
    INT_MAX = 2**31 - 1
    INT_MIN = -(2**31)
    i, n = 0, len(s)
    while i < n and s[i] == " ":
        i += 1
    sign = 1
    if i < n and s[i] in "+-":
        if s[i] == "-":
            sign = -1
        i += 1
    value = 0
    while i < n and "0" <= s[i] <= "9":
        value = value * 10 + (ord(s[i]) - ord("0"))
        if value > INT_MAX:
            return INT_MAX if sign == 1 else INT_MIN
        i += 1
    return sign * value
```

The early return also means `value` never grows beyond about `2^35`, which
keeps it exact in JavaScript and prevents overflow in languages with 32-bit
integers.

## Complexity

- Time: O(n), one pass over the string.
- Space: O(1).

## Pitfalls

- `"+-5"` and `"-+5"` are `0`: only one sign character is read, and the next
  character is not a digit.
- A space between the sign and the digits (`"- 7"`) also stops the reading.
- `"-2147483648"` is exactly `−2^31` and is not clamped; `"2147483648"` is.
- Using a general-purpose parser (`int()`, `parseInt`, `Number`) skips
  whitespace or accepts formats that these rules don't, so follow the rules
  by hand.
- Long runs of leading zeros are fine: `"0000000000123"` is `123`.
