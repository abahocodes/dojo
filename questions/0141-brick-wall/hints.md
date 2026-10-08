# Hints

## Hint 1
Minimizing the bricks crossed is the same as maximizing the number of rows
in which the line passes through a seam.

## Hint 2
A seam in a row sits at a prefix sum of that row's widths. Which prefix sums
must you leave out because they are not allowed positions?

## Hint 3
Walk each row, accumulating widths and skipping the last brick (its right
end is the outer edge). Count each prefix sum in a hash map. The answer is
`rows - (largest count)`, or `rows` if there are no seams at all.
