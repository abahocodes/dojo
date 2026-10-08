# Hints

## Hint 1
Trying every tuple is `O(n^4)`: 1.6 billion combinations at `n = 200`. Can you
split the four arrays into two halves that you handle separately?

## Hint 2
The equation `a + b + c + d == 0` is the same as `a + b == -(c + d)`. Every
pair from `nums1`/`nums2` needs a partner pair from `nums3`/`nums4` with the
opposite sum.

## Hint 3
Count every sum `nums1[i] + nums2[j]` in a hash map (`n^2` entries at most).
Then, for every `nums3[k] + nums4[l]`, add the stored count of its negation.
