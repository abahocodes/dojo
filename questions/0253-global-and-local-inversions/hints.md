# Hints

## Hint 1
Local inversions are a subset of global ones. So the counts are equal exactly
when there is no global inversion that is not local.

## Hint 2
A non-local inversion is a pair `i < j` with `j >= i + 2` and
`nums[i] > nums[j]`. For a fixed `j`, which `i` is the most dangerous?

## Hint 3
Keep the maximum of `nums[0..j-2]` as you scan. If it ever exceeds `nums[j]`,
answer `false`.
