# Approach: one pass with a pending term

Because there are no parentheses, the expression is a sum of signed terms,
where each term is a product/quotient chain. A stack of terms would work
(push `num`, push `-num`, or replace the top with `top * num` / `top / num`, then sum
the stack), but only the top of that stack is ever touched, so it can be
collapsed into two variables:

- `total`: the sum of all completed terms,
- `last`: the term currently being built.

We read the current number digit by digit. When an operator or the end of the
string is reached, we apply the operator *preceding* that number (`op`, which
starts as `+`), then remember the new operator.

```python
def calculate_no_parens(s):
    total = last = num = 0
    op = "+"
    n = len(s)
    for i, ch in enumerate(s):
        if ch.isdigit():
            num = num * 10 + ord(ch) - 48
        if (ch != " " and not ch.isdigit()) or i == n - 1:
            if op == "+":
                total, last = total + last, num
            elif op == "-":
                total, last = total + last, -num
            elif op == "*":
                last *= num
            else:
                q = abs(last) // num
                last = q if last >= 0 else -q
            op, num = ch, 0
    return total + last
```

Java, C++ and Go integer division already truncates toward zero; JavaScript
uses `Math.trunc(last / num)`.

## Complexity

- Time: O(n), one pass over the string.
- Space: O(1).

## Pitfalls

- Python's `//` floors: `-7 // 2 == -4`. Divide absolute values and restore
  the sign.
- Applying the operator that *follows* a number instead of the one that
  precedes it.
- Forgetting the final number: there is no operator after it, so the end of
  the string must also trigger the apply step (even when it ends in spaces).
- Treating spaces as operators: they must not trigger the apply step.
- Evaluating strictly left to right ignores precedence: `8+2*5` is 18, not 50.
