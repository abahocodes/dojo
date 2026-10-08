# Approach: a stack of operands

Scan the tokens. A number is pushed onto a stack. An operator pops the right
operand, then the left operand, applies itself, and pushes the result. A valid
expression leaves exactly one value on the stack: the answer.

```python
def eval_rpn(tokens):
    stack = []
    for t in tokens:
        if t in ("+", "-", "*", "/"):
            b = stack.pop()
            a = stack.pop()
            if t == "+":
                stack.append(a + b)
            elif t == "-":
                stack.append(a - b)
            elif t == "*":
                stack.append(a * b)
            else:
                q = abs(a) // abs(b)
                stack.append(q if (a < 0) == (b < 0) else -q)
        else:
            stack.append(int(t))
    return stack[0]
```

## Complexity

- Time: O(n): each token is pushed or popped a constant number of times.
- Space: O(n) for the stack in the worst case (all numbers first).

## Pitfalls

- Operand order: `"5 2 -"` is `3`, not `-3`. The first pop is the right side.
- Python's `//` floors: `-7 // 2 == -4`, but the answer needs `-3`. Compute the
  quotient on absolute values and fix the sign, or use `int(a / b)` (fine for
  32-bit values).
- In JavaScript, `Math.trunc(a / b)` truncates correctly but can produce `-0`
  for results like `-1 / 3`; make sure that ends up as plain `0`.
- `"-7"` is a number, not the operator `"-"`: test for the operator tokens
  exactly rather than checking the first character.
