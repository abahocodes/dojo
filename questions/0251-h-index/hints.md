# Hints

## Hint 1
Sort the citations in descending order. What does it mean for the paper at
position `i` (0-based) to have at least `i + 1` citations?

## Hint 2
In descending order, `h` works exactly when the `h`-th paper has at least `h`
citations. That gives an O(n log n) solution. Can you avoid the sort?

## Hint 3
The answer is at most `n`, so a paper with more than `n` citations counts the
same as one with exactly `n`. Count papers into `n + 1` buckets by
`min(citations, n)`, then sweep `h` from `n` down, accumulating how many papers
have at least `h` citations, and stop at the first `h` where that count
reaches `h`.
