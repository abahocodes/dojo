# Hints

## Hint 1
How many different values can each appear more than `n / 3` times? Try to
fit three of them into an array of length `n`.

## Hint 2
At most two values qualify. Generalise the majority vote: keep two
candidates with a counter each. When a new value matches neither and both
counters are positive, it "cancels" one copy of each candidate.

## Hint 3
Each cancellation removes three distinct values, so a value with more than
`n / 3` copies cannot be wiped out and survives as a candidate. The vote can
also leave behind false candidates, so make a second pass to count both
candidates and keep those above `n / 3`.
