# Hints

## Hint 1
Count a `1` as `+1` and a `0` as `-1`. What is the sum of a subarray that has
as many `0`s as `1`s?

## Hint 2
With that encoding, keep a running balance. A subarray `(j, i]` is balanced
exactly when the balance after index `i` equals the balance after index `j`.

## Hint 3
To make `(j, i]` as long as possible, `j` must be the **first** index where
that balance appeared. Store the first index of every balance (balance `0`
first appears at index `-1`, before the array starts) and compare at each step.
