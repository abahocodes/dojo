# Hints

## Hint 1
Checking all O(n²) substrings and testing each one in O(n) is O(n³). A
palindrome is symmetric around its middle; can you grow candidates from there?

## Hint 2
Every palindrome has a center: a single character (odd length) or the gap
between two characters (even length). There are only `2n - 1` centers.

## Hint 3
For each center, expand `lo` left and `hi` right while `s[lo] == s[hi]`.
Visit centers from left to right and only replace the best answer when you
find a strictly longer one; that keeps the earliest start on ties.
