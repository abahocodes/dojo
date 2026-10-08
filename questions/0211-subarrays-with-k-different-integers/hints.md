# Hints

## Hint 1
"Exactly `k` distinct" is awkward for a sliding window: extending a window can
both create and keep valid subarrays. "At most `k` distinct" is much easier.

## Hint 2
Exactly `k` = (at most `k`) - (at most `k - 1`).

## Hint 3
To count subarrays with at most `k` distinct values, sweep the right end,
keep a frequency table, and move the left end forward while the window has
more than `k` distinct values. Every start from `left` to `right` is then
valid, which adds `right - left + 1` subarrays.
