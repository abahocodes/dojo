## Hint 1
The rank of a value is one more than the number of *distinct* values smaller
than it.

## Hint 2
Sort a copy of the array and remove duplicates. In that list, each value's
rank is its position plus one.

## Hint 3
Build a map from value to rank from the sorted distinct values (or binary
search each element in them), then look up every element of the original
array.
