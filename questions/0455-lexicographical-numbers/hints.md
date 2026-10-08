# Hints

## Hint 1
Sorting the numbers by their strings works but costs O(n log n). Picture the
numbers as a tree instead: the children of `x` are `10x, 10x + 1, ..., 10x + 9`.

## Hint 2
Dictionary order is a preorder walk of that tree, visiting children in digit
order and skipping values above `n`. Can you find the next number from the
current one without a stack?

## Hint 3
From `cur`: if `cur * 10 <= n`, go down to `cur * 10`. Otherwise go to the
next sibling `cur + 1`, but first climb up (`cur //= 10`) while `cur` ends in
9 or `cur + 1 > n`.
