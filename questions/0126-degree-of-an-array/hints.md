# Hints

## Hint 1
A subarray with the same degree as `nums` must contain every occurrence of
at least one value that occurs "degree" times. Which value, and which
subarray around it?

## Hint 2
For a value with the maximum frequency, the shortest subarray containing all
its occurrences runs from its first occurrence to its last. Several values
may tie for the maximum frequency.

## Hint 3
In one pass, record each value's first index and count. The span of `x` is
`last - first + 1`. Answer: the smallest span among values whose count
equals the degree.
