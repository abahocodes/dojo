# Hints

## Hint 1
Think about one column at a time. How high can the water above column `i` rise
before it spills to the left or right?

## Hint 2
The water level at `i` is `min(tallest to the left, tallest to the right)`, and
the water held there is that level minus `height[i]` (never negative).
Precomputing both maxima gives an O(n) solution with O(n) extra space.

## Hint 3
To drop the extra space, use two pointers moving inward with running
`left_max` and `right_max`. Always advance the side whose running max is
smaller: that side's water level is already fully determined by its own max.
