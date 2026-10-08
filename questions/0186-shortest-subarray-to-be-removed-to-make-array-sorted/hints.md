# Hints

## Hint 1
Whatever is left after the removal is some prefix of `arr` followed by some
suffix of `arr`. Both pieces must be non-decreasing on their own. How long can
each of them be at most?

## Hint 2
Find the longest non-decreasing prefix `arr[0..L]` and the longest
non-decreasing suffix `arr[R..n-1]`. Removing everything after the prefix,
or everything before the suffix, is always valid. Can you keep part of both?

## Hint 3
Keeping `arr[0..i]` and `arr[j..]` together is valid when `arr[i] <= arr[j]`.
As `i` moves right through the prefix, the smallest valid `j` in the suffix
only moves right too, so a two-pointer sweep finds the best pair in O(n).
