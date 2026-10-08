# Hints

## Hint 1
You do not know the length in advance, so you cannot directly tell what power
of two the head's bit is worth. Can you avoid needing it?

## Hint 2
Think about how you would read a decimal number digit by digit from the left:
each new digit shifts the value so far one place to the left.

## Hint 3
Start with `value = 0`. For each node, set `value = value * 2 + node.val`
(equivalently `(value << 1) | node.val`).
