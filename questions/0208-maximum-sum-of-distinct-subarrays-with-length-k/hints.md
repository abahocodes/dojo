# Hints

## Hint 1
There are only `n - k + 1` windows of length `k`. Can you move from one window
to the next in O(1) instead of rebuilding it?

## Hint 2
Keep the running sum of the window and a count of how many times each value
occurs inside it.

## Hint 3
Also keep the number of values whose count is at least 2. When a value's count
rises to 2 that number goes up; when it falls back to 1 it goes down. A window
is valid exactly when that number is 0.
