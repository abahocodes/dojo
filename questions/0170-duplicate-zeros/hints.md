# Hints

## Hint 1
Building a second list and truncating it is easy. The classic follow-up is to
do it in place in one buffer with O(1) extra memory.

## Hint 2
Working left to right in place overwrites values you have not read yet. What
if you fill the array from the right instead?

## Hint 3
Element `i` moves right by the number of zeros before it. Count all zeros
first, then walk `i` from the end down to 0, keeping that count up to date.
Write `arr[i]` at `i + shift` (if it is still inside the array), and for a
zero also write the extra copy, skipping any position past the end.
