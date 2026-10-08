# Hints

## Hint 1
Checking every substring is O(n^2) or worse. Can a window that slides across
the string do better?

## Hint 2
Keep a count of each character in the window and the number of characters
whose count is positive.

## Hint 3
Extend the right end. While more than `k` distinct characters are inside,
remove characters from the left end, decreasing the distinct count when a
character's count drops to zero. The window length after each step is a
candidate answer.
