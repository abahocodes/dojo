# Hints

## Hint 1
The spiral peels the grid one ring at a time. Describe one ring with four
boundaries: `top`, `bottom`, `left` and `right`.

## Hint 2
Read the top row from `left` to `right`, then move `top` down by one. Do the same
for the right column, the bottom row and the left column, shrinking the matching
boundary each time.

## Hint 3
A ring can collapse into a single row or column. Before reading the bottom row
and the left column, check that `top <= bottom` and `left <= right` still hold,
or you'll read some cells twice.
