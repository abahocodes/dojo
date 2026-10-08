# Hints

## Hint 1
Think of it as a shortest-path problem: each jump is an edge of weight 1. Which
indices can you reach with 0 jumps, with exactly 1 jump, with 2?

## Hint 2
The indices reachable in exactly `j` jumps form a contiguous range, and the next
range starts right after it and ends at the furthest `i + nums[i]` over the
current range.

## Hint 3
Scan once, tracking `end` (the last index of the current range) and `furthest`
(the furthest index reachable with one more jump). When `i` reaches `end`, take a
jump: `jumps += 1`, `end = furthest`. Stop before the last index.
