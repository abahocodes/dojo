# Hints

## Hint 1
The sum of `nums[i..j]` equals `prefix[j + 1] - prefix[i]`, where `prefix[t]` is
the sum of the first `t` elements. Restate the question in terms of prefix sums.

## Hint 2
A stretch ending at `j` sums to `k` exactly when some earlier prefix equals
`prefix[j + 1] - k`. Because values can be negative, a sliding window won't work —
but counting earlier prefixes will.

## Hint 3
Walk the list keeping a running sum and a map from prefix sum to how many times it
has occurred, seeded with `{0: 1}` for the empty prefix. At each step, add
`count[running - k]` to the answer, then record `running`.
