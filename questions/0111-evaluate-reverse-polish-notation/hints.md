# Hints

## Hint 1
Read the tokens left to right. When you meet an operator, which two values
does it act on?

## Hint 2
It acts on the two most recent values that have not been used yet. A stack
gives you exactly those. Push numbers; on an operator, pop two, combine, and
push the result.

## Hint 3
The first value popped is the **right** operand: for `"a b -"` you pop `b`
then `a` and compute `a - b`. For `/`, truncate toward zero; in Python
`a // b` floors instead, so use `int(a / b)` or adjust the sign yourself.
