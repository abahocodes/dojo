# Hints

## Hint 1
Only parity matters. Replace each number by `1` if it is odd and `0` if it is
even: you want subarrays whose sum is exactly `k`.

## Hint 2
Let `P[i]` be the number of odd elements among the first `i`. The subarray
`nums[i..j]` is nice exactly when `P[j + 1] - P[i] == k`.

## Hint 3
Scan once, keeping for each value `p` how many prefixes so far had `P == p`.
At each step add the number of earlier prefixes with `P == current - k`.
(Alternatively: count subarrays with at most `k` odds minus those with at most
`k - 1`, each with a sliding window.)
