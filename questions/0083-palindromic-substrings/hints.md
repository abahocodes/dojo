# Hints

## Hint 1
Checking each of the O(n²) substrings separately costs O(n) each, O(n³) in
total. Can you reuse work between substrings that are nested inside each
other?

## Hint 2
A palindrome stays a palindrome when you peel off its first and last
characters. Turned around: every palindrome grows outward from a center.

## Hint 3
There are `2n − 1` centers: each character (odd lengths) and each gap between
neighbours (even lengths). From each center, expand while the characters on
both sides match, counting one palindrome per successful step.
