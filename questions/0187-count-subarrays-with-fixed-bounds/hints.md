# Hints

## Hint 1
An element outside `[min_k, max_k]` can never be inside a valid subarray. It
splits the array into independent pieces.

## Hint 2
Count valid subarrays by their right end `i`. A start `s` works when no
out-of-range element lies in `s..i` and the window contains at least one
`min_k` and at least one `max_k`. Which positions do you need to remember?

## Hint 3
Track three indices while scanning: the last out-of-range position `bad`, the
last position of `min_k`, and the last position of `max_k`. Every start in
`bad+1 .. min(last_min, last_max)` is valid, so add
`max(0, min(last_min, last_max) - bad)` for each `i`.
