# Hints

## Hint 1
A range can only continue while each next number is exactly one more than the
previous one. Where does a range have to end?

## Hint 2
Walk the array keeping the index where the current run started. Extend the
run while `nums[j + 1] == nums[j] + 1`; when that fails (or the array ends),
the run from the start index to `j` is one range.

## Hint 3
Format the run as `"a"` if it has one element, otherwise `"a->b"`, then start
a new run at `j + 1`. In 32-bit languages, test consecutiveness with
`nums[j + 1] == nums[j] + 1` rather than `nums[j + 1] - nums[j] == 1`: the
difference of `2^31 - 1` and `-2^31` overflows.
