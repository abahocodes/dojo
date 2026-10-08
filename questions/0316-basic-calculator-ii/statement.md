You are given a string `s` holding an arithmetic expression built from
non-negative integers, the four operators `+`, `-`, `*` and `/`, and spaces.
There are no parentheses. Compute its value and return it.

- `*` and `/` bind tighter than `+` and `-`; operators of the same strength are
  applied left to right.
- `/` is integer division that **truncates toward zero** (`7 / 2 = 3`, and a
  negative intermediate such as `-7 / 2` becomes `-3`).
- Spaces may appear anywhere between tokens (including before the first and
  after the last token) and carry no meaning.

Do not use a built-in expression evaluator such as `eval`.

## Example 1

```
s      = "8+2*5"
output = 18    # 2 * 5 is evaluated first
```

## Example 2

```
s      = " 20 - 9 / 2 * 3 "
output = 8     # 9 / 2 = 4, 4 * 3 = 12, 20 - 12 = 8
```

## Constraints

- `1 <= len(s) <= 3 * 10^5`
- `s` contains only digits, `+`, `-`, `*`, `/` and spaces.
- `s` is a valid expression: it starts and ends with a number (ignoring
  spaces), operators and numbers alternate, and there is no unary minus.
- Every number in `s` lies in `[0, 2^31 - 1]`; no division by zero occurs.
- Every intermediate result (each product or quotient, and the running sum
  when the terms are added left to right) fits in a 32-bit signed integer.
