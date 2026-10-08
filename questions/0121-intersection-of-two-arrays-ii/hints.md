# Hints

## Hint 1
For each value, the answer only depends on how many times it occurs in each
array.

## Hint 2
Count the values of one array. Then each element of the other array can
"claim" one copy from those counts.

## Hint 3
Walk `nums2`; when `count[x] > 0`, decrement it and output `x`. Values are in
`0..1000`, so the counts fit in an array of 1001.
