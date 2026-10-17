# Hints

## Hint 1
Generating all `2^n` index subsets and throwing away repeats works, but you
need a way to recognise repeats. Can you avoid creating them at all?

## Hint 2
Sort `nums` first so equal values sit next to each other. A duplicate subset
appears when you build the same choice of values starting from two different
copies of one value.

## Hint 3
Backtrack with a start index as in plain Subsets. Inside the loop over
`i = start, start + 1, ...`, skip `nums[i]` when `i > start` and
`nums[i] == nums[i - 1]`: that value has already been tried as the next pick
at this depth, and the earlier copy covered every subset this one would make.
