# Hints

## Hint 1
How big can the next number be? A 10-digit number has digit squares summing to
at most `10 · 81 = 810`, so after one step every value is small. The sequence
can't grow forever, so it must either reach `1` or repeat a value.

## Hint 2
Detecting "this sequence repeats" is the same problem as detecting a cycle in a
linked list where each number points to its successor. A set of values seen so
far answers it directly.

## Hint 3
For O(1) extra space, use two pointers: `slow` advances one step at a time,
`fast` two steps. If `fast` reaches `1`, the number is happy; if `slow` and
`fast` meet first, you're in a cycle without `1`.
