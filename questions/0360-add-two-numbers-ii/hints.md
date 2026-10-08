# Hints

## Hint 1
Addition starts from the least significant digit, but that is at the *end* of
each list. The numbers can have 100 digits, so converting to a built-in integer
type is not an option in most languages.

## Hint 2
Push each list's digits onto a stack (or reverse the lists). Popping then gives
the digits from least significant upward.

## Hint 3
Pop one digit from each stack (0 if empty), add the carry, and create a node
for `sum % 10` that you put in *front* of the result built so far. Keep going
while either stack is non-empty or the carry is non-zero.
