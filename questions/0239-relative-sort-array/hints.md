## Hint 1
You could sort with a custom key: the position of a value in `arr2` if it is
there, otherwise something larger than any position, then the value itself.

## Hint 2
The values are small (0 to 1000). Count how many times each value occurs in
`arr1` instead of comparing elements.

## Hint 3
Walk `arr2` and emit each value as many times as it was counted, zeroing its
count. Then walk every value from 0 to 1000 in order and emit whatever counts
remain: those are exactly the leftover values, already in ascending order.
