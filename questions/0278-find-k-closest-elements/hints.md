# Hints

## Hint 1
Because `arr` is sorted, the answer is always a contiguous block of `k`
elements. The task is to find where that block starts.

## Hint 2
Compare a candidate window `arr[i .. i+k-1]` with the window shifted one step
right, `arr[i+1 .. i+k]`. They differ only in `arr[i]` (dropped) and
`arr[i+k]` (added). Which one should stay?

## Hint 3
Binary search the start `i` in `[0, n - k]`. If `x - arr[mid] > arr[mid + k] - x`,
the element `arr[mid]` loses to `arr[mid + k]`, so the window must start after
`mid`; otherwise the start is `mid` or earlier. Return `arr[lo : lo + k]`.
