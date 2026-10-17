# Approach: digit-by-digit schoolbook multiplication

Write `num1` with `m` digits and `num2` with `n` digits. Their product has at
most `m + n` digits, so allocate `out` of that length, indexed from the left.

Digit `num1[i]` sits `m - 1 - i` places from the right and `num2[j]` sits
`n - 1 - j` places from the right, so their product belongs
`(m - 1 - i) + (n - 1 - j)` places from the right, which is index `i + j + 1`
in `out`. Anything that spills over carries into index `i + j`.

Process pairs from the right so each carry is pushed leftward before that cell
is finalised:

```python
def multiply(num1, num2):
    if num1 == "0" or num2 == "0":
        return "0"
    m, n = len(num1), len(num2)
    out = [0] * (m + n)
    for i in range(m - 1, -1, -1):
        a = ord(num1[i]) - 48
        for j in range(n - 1, -1, -1):
            total = a * (ord(num2[j]) - 48) + out[i + j + 1]
            out[i + j + 1] = total % 10
            out[i + j] += total // 10
    start = 0
    while start < len(out) - 1 and out[start] == 0:
        start += 1
    return "".join(str(d) for d in out[start:])
```

`out[i + j]` can briefly exceed 9, but it is normalised when the loop later
treats it as `out[i' + j' + 1]` (for the same `i`, the next `j` to the left),
or, for the leftmost cell `out[i]`, by the next row. The leftmost cell
`out[0]` only ever receives a final carry, which is at most 9.

## Complexity

- Time: O(m · n), one step per pair of digits.
- Space: O(m + n) for the digit array.

## Pitfalls

- Converting the strings to integers (or using `BigInt`/`BigInteger`) defeats
  the purpose and is not allowed; in most languages it overflows anyway.
- A zero factor gives `"0"`, not a string of zeros: strip leading zeros but
  always keep one digit.
- Don't forget the `- '0'` when reading digits; multiplying character codes
  gives nonsense.
- Building each partial product as a string and adding strings works, but is
  more code and slower; the position trick does it all in one array.
