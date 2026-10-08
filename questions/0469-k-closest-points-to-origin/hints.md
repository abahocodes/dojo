# Hints

## Hint 1
You never need the actual distance, only comparisons between distances.
`x² + y²` orders points the same way as `sqrt(x² + y²)` and avoids floating
point entirely.

## Hint 2
Sorting all points by distance works in `O(n log n)`. Can you do better by
keeping only `k` candidates around at any time?

## Hint 3
Keep a max-heap of size `k` keyed by squared distance. For each point, push it
while the heap has fewer than `k` items; otherwise, if it is closer than the
heap's top (the farthest kept point), replace the top. At the end the heap
holds exactly the `k` closest points. Quickselect is an `O(n)` average
alternative.
