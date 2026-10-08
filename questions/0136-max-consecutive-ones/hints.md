# Hints

## Hint 1
One left-to-right pass is enough. What do you need to remember as you go?

## Hint 2
Track the length of the run of 1s that ends at the current position.

## Hint 3
On a `1`, extend the current run and update the best seen so far; on a `0`,
reset the current run to zero.
