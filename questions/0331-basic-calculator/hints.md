# Hints

## Hint 1
With only `+` and `-`, the expression is a signed sum of its literals. Each
literal's sign is the product of its own operator sign and the signs in front
of every parenthesised group that contains it.

## Hint 2
Scan left to right keeping `result` (the running sum at the current nesting
level), `num` (the literal being read) and `sign` (`+1` or `-1`, the operator
before `num`). An operator finishes the current literal: `result += sign * num`.

## Hint 3
On `"("`, push the current `result` and `sign` onto a stack and start fresh
(`result = 0`, `sign = 1`). On `")"`, finish the pending literal, then combine:
`result = saved_result + saved_sign * result`. A unary minus needs no special
case: it simply behaves like `0 - ...` at the start of a level.
