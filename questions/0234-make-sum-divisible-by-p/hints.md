# Hints

## Hint 1
Let `need = sum(nums) % p`. If `need` is `0`, delete nothing. Otherwise the
deleted subarray's sum must leave the same remainder `need` when divided by
`p`.

## Hint 2
With prefix sums, the subarray `nums[i..j-1]` has sum `prefix[j] - prefix[i]`.
Its remainder is `need` exactly when
`prefix[i] % p == (prefix[j] - need) % p`.

## Hint 3
Scan left to right, storing for each prefix remainder the **latest** index
at which it occurred (remainder `0` at index `-1` to start). At each `j`, look
up the remainder you need; the gap to its latest index is a candidate. Return
`-1` if the best candidate is the whole array or none exists.
