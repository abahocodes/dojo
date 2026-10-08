# Hints

## Hint 1
Simulating second by second can take O(n^2). Instead, look at each stretch of
upright dominoes between two pushed ones. Its fate depends only on the two
pushed dominoes at its ends.

## Hint 2
Treat the row as if there were an extra `'L'` before it and an extra `'R'`
after it. Then each stretch of dots lies between two letters, and there are
four cases: `L...L`, `R...R`, `L...R` and `R...L`.

## Hint 3
`L...L` all fall left, `R...R` all fall right, `L...R` stays upright. For
`R...L`, the left half falls right and the right half falls left; with an odd
number of dots the middle one stays upright.
