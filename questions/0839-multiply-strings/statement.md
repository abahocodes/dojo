You are given two non-negative integers written out as decimal strings,
`num1` and `num2`. Return their product, also as a decimal string.

The numbers can be far too long for any built-in integer type, so work on the
digits directly. **Do not** use a big-integer library (Python's unbounded
`int`, JavaScript's `BigInt`, Java's `BigInteger`, Go's `math/big`, ...) and do
not convert either whole string into a number.

The result must not have leading zeros, except for the number zero itself,
which is written `"0"`.

## Example 1

```
num1   = "123"
num2   = "45"
output = "5535"
```

## Example 2

```
num1   = "7"
num2   = "0"
output = "0"
```

Anything times zero is zero, written as a single `"0"`, not `"00"`.

## Example 3

```
num1   = "999"
num2   = "999"
output = "998001"
```

The product of an `m`-digit and an `n`-digit number has at most `m + n` digits.

## Constraints

- `1 <= len(num1), len(num2) <= 200`
- `num1` and `num2` contain only the digits `0`-`9`.
- Neither input has a leading zero, unless it is exactly `"0"`.
