# Hints

## Hint 1
Slashes only separate names, and repeated slashes mean nothing. Split the path
on `/` and look at the pieces one at a time; empty pieces can be ignored.

## Hint 2
The directories you are currently inside form a stack: entering a directory
pushes it, and `..` pops the most recent one.

## Hint 3
For each piece: skip it if it is empty or `.`; pop if it is `..` and the stack
is non-empty; otherwise push it. Finally join the stack with `/` and put a
`/` in front (an empty stack gives `"/"`).
