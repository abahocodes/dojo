# Approach: pop and push digits, check before you push

Take digits off the end of `x` and push them onto the end of `rev`:

```
digit = x % 10      (truncated toward zero)
x     = x / 10      (truncated toward zero)
rev   = rev * 10 + digit
```

`rev * 10 + digit` is the only operation that can leave the 32-bit range, so
check just before it. Let `MAX = 2^31 - 1 = 2147483647`, so `MAX / 10 =
214748364` and the last digit of `MAX` is `7`:

- if `rev > 214748364`, then `rev * 10` already exceeds `MAX`;
- if `rev == 214748364`, the result is safe only when `digit <= 7`.

For negative numbers the mirror image uses `MIN = -2147483648`: overflow when
`rev < -214748364`, or `rev == -214748364` and `digit < -8`. That keeps every
intermediate value inside 32 bits.

In Java, C++, JavaScript and Go `%` and `/` on a negative `x` truncate toward
zero, so `digit` is negative too and the loop works on signed values directly.
Python's `%` and `//` round toward negative infinity, so the Python version
reverses the absolute value and applies the sign at the end. The absolute
value of `-2^31` is `2^31`, which is one past `MAX`, but that input reverses to
`8463847412` and is rejected by the same check, since the check runs on the
reversed digits.

```python
def reverse_integer(x):
    INT_MAX = 2**31 - 1
    sign = -1 if x < 0 else 1
    x = abs(x)
    limit = INT_MAX if sign == 1 else INT_MAX + 1  # |MIN| = MAX + 1
    rev = 0
    while x:
        digit = x % 10
        x //= 10
        if rev > limit // 10 or (rev == limit // 10 and digit > limit % 10):
            return 0
        rev = rev * 10 + digit
    return sign * rev
```

The signed version, as written in Java:

```java
int rev = 0;
while (x != 0) {
    int digit = x % 10;
    x /= 10;
    if (rev > Integer.MAX_VALUE / 10 || (rev == Integer.MAX_VALUE / 10 && digit > 7)) return 0;
    if (rev < Integer.MIN_VALUE / 10 || (rev == Integer.MIN_VALUE / 10 && digit < -8)) return 0;
    rev = rev * 10 + digit;
}
return rev;
```

In fact, because `x` itself fits in 32 bits, when `rev == 214748364` and a digit
is still left, that digit is the first digit of a 10-digit `x`, so it is at most
`2` and the `digit > 7` part can never fire. Keeping it costs nothing and makes
the check obviously correct.

## Complexity

- Time: O(log |x|): one step per decimal digit, at most 10.
- Space: O(1).

## Pitfalls

- Computing the reversed value in a 64-bit integer (or as a string) and
  comparing at the end gets the right answers, but it sidesteps the point of
  the question.
- In Python, `-123 % 10` is `7`, not `-3`. Work on `abs(x)`.
- In C++ and Java, `abs(INT_MIN)` overflows. The signed loop above never takes
  an absolute value.
- In JavaScript, use `Math.trunc(x / 10)`, not `Math.floor`, for negatives.
- Trailing zeros (`120`) become leading zeros and simply vanish; no special
  case needed.
