# Hints

## Hint 1
A group can only be expanded once you know its whole body, and the body may
contain groups of its own. That inside-out order suggests a stack.

## Hint 2
Repeat counts can have several digits: accumulate them as you read
(`k = k * 10 + digit`) rather than taking a single character.

## Hint 3
On `[`, push the text built so far together with the pending count, then start
a fresh buffer. On `]`, pop the saved text and count, and set the buffer to
`saved + buffer * count`. Letters are appended to the current buffer.
