# Hints

## Hint 1
Which people can person `i` see? Walking right from `i`, a person is visible
exactly when they are taller than everyone seen so far between them and `i`,
and the walk stops after the first person taller than `i`.

## Hint 2
Process the line from right to left and keep a stack of people that are still
"exposed" from the left: their heights decrease from the bottom of the stack
to the top.

## Hint 3
For person `i`, pop every stacked person shorter than `heights[i]`; each one
is visible to `i` (and hidden from everyone further left by `i`). If the stack
is still non-empty, its top is the first taller person, also visible. Then
push `i`.
