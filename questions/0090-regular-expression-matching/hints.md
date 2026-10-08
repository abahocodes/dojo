# Hints

## Hint 1
Look at the pattern two characters at a time. If the next pattern element is
**not** followed by `*`, it must match exactly one character of `s`. If it is
followed by `*`, you have a choice to make.

## Hint 2
For `x*` there are two options: skip it entirely (match zero copies), or, if
`x` matches the current character of `s`, consume that one character and stay
on `x*` (it may match more). Trying both recursively is correct but
exponential without memoisation.

## Hint 3
Define `dp[i][j]` = "does `s[i:]` match `p[j:]`?". Then
`dp[i][j] = dp[i][j+2] or (first and dp[i+1][j])` when `p[j+1] == '*'`, and
`first and dp[i+1][j+1]` otherwise, where `first` means `i < len(s)` and
`p[j]` is `s[i]` or `.`. The base case is `dp[len(s)][len(p)] = true`.
