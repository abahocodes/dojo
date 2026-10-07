# Hints

## Hint 1
When you meet a closing bracket, which opening bracket is it allowed to match?
Think about the most recent one that is still unmatched.

## Hint 2
"Most recent unmatched opener" is exactly what a last-in, first-out structure
gives you. Push openers; a closer must match the top.

## Hint 3
Scan left to right. Push each opener onto a stack. For each closer, fail if the
stack is empty or its top isn't the matching opener; otherwise pop. At the end
the string is balanced only if the stack is empty.
