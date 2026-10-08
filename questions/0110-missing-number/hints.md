# Hints

## Hint 1
Sorting and looking for the first gap works in O(n log n). Can you use what
you know about the complete range `0..n` instead?

## Hint 2
You know exactly what the sum (or XOR) of `0..n` should be. How does it differ
from the sum (or XOR) of `nums`?

## Hint 3
The answer is `n * (n + 1) / 2 - sum(nums)`. Equivalently, XOR every index
`0..n` together with every value; everything cancels except the missing number.
