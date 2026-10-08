# Approach: bottom-up merge sort

A single element is already a sorted run. Merge neighbouring runs of width 1
into sorted runs of width 2, then those into width 4, and keep doubling until
one run covers the whole array. Each pass is a linear merge, and there are
`ceil(log2 n)` passes.

Merging two sorted runs `a[lo:mid]` and `a[mid:hi]` walks both with a pointer
and copies the smaller front element into a second buffer. Taking from the
left run on ties keeps the sort stable. After each pass the two buffers swap
roles, so no copying back is needed.

```python
def sort_array(nums):
    a = list(nums)
    n = len(a)
    buf = [0] * n
    width = 1
    while width < n:
        for lo in range(0, n, 2 * width):
            mid = min(lo + width, n)
            hi = min(lo + 2 * width, n)
            i, j, k = lo, mid, lo
            while i < mid and j < hi:
                if a[i] <= a[j]:
                    buf[k] = a[i]
                    i += 1
                else:
                    buf[k] = a[j]
                    j += 1
                k += 1
            buf[k:hi] = a[i:mid] + a[j:hi]
        a, buf = buf, a
        width *= 2
    return a
```

Heap sort (build a max-heap in place, then repeatedly move the root to the
end) is an O(1)-extra-space alternative. With values bounded by `5 * 10^4`,
counting sort would also work in O(n + range).

## Complexity

- Time: O(n log n) in every case.
- Space: O(n) for the second buffer.

## Pitfalls

- Quick sort with a fixed first/last pivot degrades to O(n^2) on already
  sorted or all-equal input. Randomize the pivot and partition three ways if
  you choose it.
- Off-by-one errors on the last, shorter run of each pass: clamp `mid` and
  `hi` to `n`.
- Forgetting to copy the leftover tail of whichever run is not exhausted.
