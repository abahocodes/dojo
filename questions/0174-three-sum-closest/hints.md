# Hints

## Hint 1
Trying every triple is O(n^3). Sorting the array first lets you steer a sum
up or down deliberately.

## Hint 2
Fix the smallest element `nums[i]` of the triple. The other two come from the
part of the sorted array to its right: start one pointer just after `i` and
one at the end.

## Hint 3
If the current three-element sum is below `target`, move the left pointer
right to increase it; if above, move the right pointer left; if equal, you can
return immediately. Track the best sum seen across all `i`.
