# Hints

## Hint 1
At any moment only the innermost open call is running, and calls nest like
brackets. Which data structure tracks "the innermost open call"?

## Hint 2
Keep a stack of function ids and a variable `prev`: the first time unit not yet
credited to anyone. Every log entry closes a stretch of time that belongs to
whatever is on top of the stack.

## Hint 3
On `start` at `t`: credit `t - prev` units to the current top (if any), push the
new id, set `prev = t`. On `end` at `t`: pop the top and credit it with
`t - prev + 1` units (the end is inclusive), then set `prev = t + 1`.
