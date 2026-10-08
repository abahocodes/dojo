# Hints

## Hint 1
Copying the values into an array makes the twins easy to index:
`a[i] + a[n - 1 - i]`. Can you do it without the extra array?

## Hint 2
The twin of a node in the first half lives in the second half, in reverse
order. What if the second half were reversed?

## Hint 3
Find the middle with slow/fast pointers, reverse the second half in place, then
walk the first half and the reversed half together, tracking the largest
`a.val + b.val`.
