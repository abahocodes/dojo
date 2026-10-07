# Hints

## Hint 1
Start by counting how many times each value appears. Once you have the counts,
the problem is "pick the `k` largest counts".

## Hint 2
Sorting the counts works. To do better, keep a min-heap of size `k` (O(n log k)),
or notice that every count is a whole number between `1` and `len(nums)`.

## Hint 3
Bucket sort: make `len(nums) + 1` buckets, put each value into `buckets[count]`,
then walk the buckets from the highest count down, collecting values until you
have `k` of them.
