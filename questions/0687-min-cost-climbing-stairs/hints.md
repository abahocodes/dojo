# Hints

## Hint 1
Think about the last move. To reach the landing (position `n`) you must
have stepped off either step `n - 1` or step `n - 2`. The same is true for
any step `i`: you arrive from `i - 1` or from `i - 2`.

## Hint 2
Let `best[i]` be the cheapest total cost of standing on position `i`. Then
`best[0] = best[1] = 0` (free starts) and
`best[i] = min(best[i - 1] + cost[i - 1], best[i - 2] + cost[i - 2])`.
The answer is `best[n]`.

## Hint 3
Each `best[i]` only uses the two values before it, so two variables are
enough: O(n) time, O(1) extra space.
