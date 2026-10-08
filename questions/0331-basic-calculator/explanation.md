# Approach: one pass with a stack of saved contexts

Since only addition and subtraction appear, there is no precedence to worry
about — only parentheses and signs. Scan the string once with three pieces of
state for the current nesting level:

- `result`: the sum of everything finished so far at this level;
- `num`: the digits of the literal currently being read;
- `sign`: `+1` or `-1`, the operator in front of `num`.

Then:

- digit: `num = num * 10 + digit`;
- `"+"` / `"-"`: fold the pending literal in (`result += sign * num`), reset
  `num`, and set `sign` from the operator;
- `"("`: save `result` and `sign` on a stack and start a fresh level;
- `")"`: fold the pending literal, then pop the saved sign and result and set
  `result = saved_result + saved_sign * result`.

A unary minus at the start of a level is handled for free: `result` and `num`
are 0 there, so folding them in changes nothing, and `sign` becomes `-1`.

```python
def calculate(s):
    result, num, sign = 0, 0, 1
    stack = []
    for ch in s:
        if ch.isdigit():
            num = num * 10 + (ord(ch) - 48)
        elif ch == "+" or ch == "-":
            result += sign * num
            num = 0
            sign = 1 if ch == "+" else -1
        elif ch == "(":
            stack.append(result)
            stack.append(sign)
            result, sign = 0, 1
        elif ch == ")":
            result += sign * num
            num = 0
            saved_sign = stack.pop()
            saved_result = stack.pop()
            result = saved_result + saved_sign * result
    return result + sign * num
```

Using an explicit stack instead of recursion keeps deeply nested inputs (tens of
thousands of parentheses) from overflowing the call stack.

## Complexity

- Time: O(n) — every character is handled once.
- Space: O(n) for the stack in the worst case of deep nesting.

## Pitfalls

- Forgetting to fold the last literal in after the loop (`result + sign * num`).
- Multi-digit literals: accumulate digits instead of treating each digit as a
  number.
- Resetting `num` after a `")"`; otherwise a stale value is added again at the
  next operator.
- Recursive descent is natural but can overflow the call stack on deep nesting.
