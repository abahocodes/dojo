# Hints

## Hint 1
Split the expression mentally into terms joined by `+` and `-`. Each term is a
run of numbers joined by `*` and `/`, and the answer is just the sum of the
terms (with signs).

## Hint 2
Scan left to right, building the current number digit by digit. When you reach
an operator (or the end), apply the operator that came *before* the number you
just finished.

## Hint 3
Keep the running total of finished terms and the value of the term in
progress (`last`). For a preceding `+` or `-`, add `last` to the total and
start a new term `num` or `-num`. For `*` or `/`, fold `num` into `last`.
Remember `/` truncates toward zero, so a negative `last` needs care in
languages whose integer division floors.
