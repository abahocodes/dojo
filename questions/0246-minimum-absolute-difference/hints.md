# Hints

## Hint 1
Checking every pair is O(n^2). Is there an ordering of the values in which
the closest pairs must sit side by side?

## Hint 2
After sorting, the minimum difference is attained only between adjacent
elements: for `x < y < z`, `z - x` is larger than both `y - x` and `z - y`.

## Hint 3
Sort, take the minimum of `arr[i+1] - arr[i]` in one pass, then collect every
adjacent pair whose gap equals it. The pairs come out already ordered by `a`.
