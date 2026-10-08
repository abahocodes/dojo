# Hints

## Hint 1
If `p` survives the first `k` removals, it also survives the first `k - 1`:
removing fewer characters cannot hurt. What does that monotonicity suggest?

## Hint 2
Binary search on `k`. You need a fast test: "is `p` a subsequence of `s`
with the first `k` removable positions deleted?"

## Hint 3
Store, for every position of `s`, the step at which it is removed
(`removed_at[removable[j]] = j`, others never). Position `i` is deleted after
`k` steps exactly when `removed_at[i] < k`. Then one greedy two-pointer pass
checks the subsequence in O(len(s)).
