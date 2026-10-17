# Hints

## Hint 1
Without wildcards you would track one number: how many `'('` are still
open. A string is balanced when that count never drops below zero and ends at
zero. Trying all three choices for every `'*'` is exponential.

## Hint 2
With wildcards, the open count isn't a single number any more, it is a set
of possible values. Each character shifts the set: `'('` adds 1, `')'`
subtracts 1, `'*'` could do either or nothing. Is the set always a
contiguous range?

## Hint 3
Track only `lo` and `hi`, the smallest and largest possible open counts. For
`'*'`, do `lo -= 1, hi += 1`. If `hi` ever goes below 0, there are too many
`')'`: return false. Clamp `lo` at 0 (a negative count is never a valid
choice). At the end, the string is valid exactly when `lo == 0`.
