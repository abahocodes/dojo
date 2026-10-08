# Hints

## Hint 1
Sorting by numeric value fails (`7` should precede `12`), and sorting the
decimal strings in descending lexicographic order fails too (`"40"` comes
before `"4"`, but `"440"` beats `"404"`).

## Hint 2
Decide the order of two numbers by asking directly which concatenation is
larger: should `a` come before `b`, compare `a + b` with `b + a` as strings.

## Hint 3
Sort the strings with that comparator, join them, and handle the all-zero
case: if the first string is `"0"`, the answer is `"0"`.
