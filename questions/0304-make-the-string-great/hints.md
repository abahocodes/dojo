# Hints

## Hint 1
Removing a clash can create a new clash between the characters on either
side. That chain reaction is the same shape as matching brackets.

## Hint 2
Scan left to right and keep the characters that survive so far on a stack.
A new character can only clash with the top of the stack.

## Hint 3
Two letters clash when they differ but are equal after lowercasing. In ASCII
that is the same as their codes differing by exactly 32. Pop on a clash and
push otherwise. The stack is the answer.
