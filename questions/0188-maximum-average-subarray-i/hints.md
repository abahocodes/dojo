# Hints

## Hint 1
Since every block has the same length `k`, the block with the largest average
is simply the block with the largest sum. Divide once, at the end.

## Hint 2
Recomputing each block's sum from scratch costs O(k) per block. Two
neighbouring blocks share `k - 1` elements. How does the sum change when the
block moves one step to the right?

## Hint 3
Keep a running sum of the current block. When sliding right, add the element
that enters and subtract the one that leaves: `sum += nums[i] - nums[i - k]`.
Track the maximum sum and return `max_sum / k`.
