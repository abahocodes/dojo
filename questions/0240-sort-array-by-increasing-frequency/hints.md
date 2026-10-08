## Hint 1
First count how many times each value appears. A hash map works; since values
lie in `[-100, 100]`, an array of 201 counters with an offset of 100 works too.

## Hint 2
Then sort the array itself with a custom order that looks up each element's
count.

## Hint 3
Compare two elements by their counts ascending; if the counts are equal,
compare the values descending. In Python that is the key `(count[x], -x)`.
