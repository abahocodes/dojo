# Hints

## Hint 1
Only the highest and lowest picked score matter. If you fix the lowest and the
highest, which other scores can you add for free?

## Hint 2
Sort the scores. In an optimal pick you can always replace the chosen scores by
`k` scores that are adjacent in the sorted order without making the spread
larger.

## Hint 3
After sorting, the answer is the minimum of `sorted[i + k - 1] - sorted[i]`
over every `i` from `0` to `n - k`: a fixed-size window of length `k`.
