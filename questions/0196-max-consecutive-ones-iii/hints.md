# Hints

## Hint 1
Rephrase the question: find the longest contiguous subarray that contains at
most `k` zeros. Every zero inside it gets flipped.

## Hint 2
Use a window `[left, right]`. Extend `right` one step at a time and keep a
count of the zeros inside the window.

## Hint 3
Whenever the zero count exceeds `k`, advance `left` (decrementing the count
when a zero leaves) until the window is valid again. Record `right - left + 1`
after each step; the maximum is the answer.
