# Hints

## Hint 1
A set or a counting array would solve it immediately, but both use O(n)
memory. Sorting would modify the input. Try reading the array as a graph.

## Hint 2
Draw an arrow from every index `i` to index `nums[i]`. Start at index 0 and
keep following arrows. Because no value is 0, you never return to index 0,
and since the walk is finite it must eventually loop. Which index has two
arrows pointing into it?

## Hint 3
The duplicate value is the entrance of that loop. Find it with Floyd's
tortoise and hare: a slow pointer moves one step, a fast one two steps, until
they meet; then restart one pointer from the beginning and move both one step
at a time. They meet at the loop entrance.
