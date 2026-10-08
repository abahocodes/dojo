# Hints

## Hint 1
You could push every `(` on a stack and pop on `)`. What would the size of the
stack tell you?

## Hint 2
The stack's size is the current depth, and you never look at what is inside
it. A single counter can replace it.

## Hint 3
Add 1 on `(` and record the maximum, subtract 1 on `)`, and ignore every
other character.
