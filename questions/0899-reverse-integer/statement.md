You are given a signed 32-bit integer `x`. Write its decimal digits in the
opposite order, keeping the sign, and return the result. Leading zeros that
appear after reversing are dropped, so `120` becomes `21`.

If the reversed number does not fit in a signed 32-bit integer (outside
`[-2^31, 2^31 - 1]`, that is `[-2147483648, 2147483647]`), return `0` instead.

Pretend the machine you run on has only 32-bit integers: your solution should
not hold the reversed value in a 64-bit (or arbitrary-precision) number just to
check it afterwards. Detect the overflow before it happens.

## Example 1

```
x      = 123
output = 321
```

## Example 2

```
x      = -120
output = -21
```

## Example 3

```
x      = 1534236469
output = 0
# 9646324351 is larger than 2147483647
```

## Constraints

- `-2^31 <= x <= 2^31 - 1`
