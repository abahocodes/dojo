# Hints

## Hint 1
The altitude at point `i + 1` is the altitude at point `i` plus `gain[i]`.

## Hint 2
So the altitudes are the running (prefix) sums of `gain`, starting from `0`.

## Hint 3
Keep the current altitude and the best seen so far, starting both at `0` so
the starting point counts.
