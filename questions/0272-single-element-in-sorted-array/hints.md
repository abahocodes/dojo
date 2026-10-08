# Hints

## Hint 1
XOR-ing everything finds the answer in `O(n)`, but that ignores the sorting.
Look at where pairs start: before the single element, does a pair start at an
even or an odd index? What about after it?

## Hint 2
Left of the single element, every pair occupies indices `(2j, 2j + 1)`. The
single element shifts everything after it by one, so to its right pairs sit
at `(2j + 1, 2j + 2)`. For an even index `i`, `nums[i] == nums[i + 1]` holds
exactly when `i` is left of the single element.

## Hint 3
Binary search over even indices only. Take `mid`, make it even (subtract 1 if
odd). If `nums[mid] == nums[mid + 1]`, the single element is after this pair:
`lo = mid + 2`. Otherwise it is at `mid` or before: `hi = mid`. Stop when
`lo == hi`.
