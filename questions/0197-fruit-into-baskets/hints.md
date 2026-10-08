# Hints

## Hint 1
Ignore the story: you need the longest contiguous subarray with at most two
distinct values.

## Hint 2
Keep a window `[left, right]` and a count of how many of each value it holds.
Extend `right` one step at a time.

## Hint 3
When the window holds three distinct values, move `left` forward, decrementing
counts and dropping a value when its count reaches zero, until only two remain.
Track the largest window length seen.
