# Hints

## Hint 1
The straightforward way is to simulate typing with a stack for each string and
compare the results. That uses O(n) memory. Can you avoid building the strings?

## Hint 2
A `#` only affects characters to its left. If you read a string from the end,
you already know how many pending backspaces will erase the next letter.

## Hint 3
Walk both strings backwards. For each, skip characters while counting `#`s:
a `#` adds one to the skip count, a letter with a positive skip count is
erased. When both pointers land on a surviving letter, compare them; if one
string runs out before the other, the answer is `false`.
