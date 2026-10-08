# Hints

## Hint 1
Don't search over interleavings. Just replay the process with a real stack.

## Hint 2
When is popping the right move? If the top of the stack equals the next
value you need to pop, popping it now can never hurt: if you wait, more
values land on top of it.

## Hint 3
For each value in `pushed`: push it, then pop while the stack is non-empty and
its top equals `popped[j]`, advancing `j`. At the end the sequence is valid
exactly when the stack is empty.
