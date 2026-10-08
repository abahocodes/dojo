# Hints

## Hint 1
You never need to know *how* you reach an index, only whether you can. If index
`i` is reachable, which indices does that make reachable?

## Hint 2
The reachable indices always form a contiguous block starting at 0: if you can
reach `i`, you can reach everything before it too. So one number describes them.

## Hint 3
Walk left to right keeping `furthest`, the largest index reachable so far. If you
ever stand on an index beyond `furthest`, you're stuck; otherwise update
`furthest = max(furthest, i + nums[i])`.
