# Hints

## Hint 1
Only one value matters: `M`, the global maximum. Every other element is just
filler.

## Hint 2
If a subarray `nums[l..r]` contains `M` at least `k` times, so does every
subarray that starts at or before `l` and ends at `r`. Counts are monotone in
the left edge.

## Hint 3
Sweep `r` from left to right and keep `left` as the smallest start for which
`nums[left..r]` has fewer than `k` copies of `M` (advance it while the window
has `k` or more). Then exactly `left` valid subarrays end at `r`. Sum them in
a 64-bit integer.
